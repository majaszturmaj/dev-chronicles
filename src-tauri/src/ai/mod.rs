pub mod client;

use anyhow::{anyhow, Context, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    ai::client::AiClient,
    db::{get_ai_settings, models::{ActivityLog, AiReportRow}},
};
use sqlx::SqlitePool;

use crate::sanitizer::sanitize_json;

const SYSTEM_PROMPT: &str = r#"
**ROLE:**
You are an expert Technical Documentation Assistant. Your goal is to analyze raw activity logs (terminal commands, browser history, and file edits) from a software developer's session and synthesize them into a concise, structured "Work Log" entry that connects to ongoing work.

**CONTEXT AWARENESS:**
You will receive summaries of the user's recent work sessions. Use these to:
- Recognize ongoing projects or tasks
- Note progress and blockers relative to prior sessions
- Highlight connections between current and past work
- Avoid repeating information already documented in recent sessions

**INPUT DATA:**
You will receive a chronological stream of data including:
1. **Terminal:** Shell commands and key outputs
2. **Browser:** Visited URLs, page titles, research activities
3. **VSCode/Editors:** File edits, saves, and language contexts
4. **Recent Context:** Summaries from the last 1-3 sessions (if available)

**INSTRUCTIONS:**
1. **Identify Intent:** Don't just list actions. Analyze correlations to determine high-level goals:
   - "Searched for Docker + Kubernetes" + "edited deployment.yaml" + "ran kubectl commands" = "Setting up Kubernetes deployment"
   - Reference prior sessions if relevant ("Continuing work on feature X from yesterday")

2. **Extract Meaningful Signals:**
   - Terminal: Extract command families (npm build, git, docker, etc.), success/failure patterns
   - Browser: Focus on documentation, GitHub issues, StackOverflow—ignore casual browsing
   - Editor: Note file modifications, languages worked on, project changes

3. **Filter Noise:**
   - Skip trivial navigation (`cd`, `ls`, `pwd`) unless revealing directory context
   - Ignore failed commands that were immediately retried
   - Skip non-work browsing

4. **Synthesize & Sequence:**
   - Group related actions chronologically
   - Show cause-and-effect ("Found solution in X, applied to Y, verified with Z")
   - Highlight blockers or unexpected patterns

5. **Continuity & Progress:**
   - If recent context provided, note whether this session continues, pivots, or completes prior work
   - Flag when a multi-session task shows progress or reaches a milestone

6. **Security & Privacy (CRITICAL):**
   - NEVER include passwords, API keys, secrets, or PII
   - Replace detected secrets with `[REDACTED_SECRET]`
   - Redact internal IPs and identifiable file paths where possible

7. **Format:** Output in Markdown with clear sections.

**OUTPUT FORMAT:**
## ⏱️ [Time Range]
**Focus:** [One-sentence summary of primary task]
**Status:** [Continuing / New / Completed / Blocked]

### 🛠️ Key Actions
- [Technical step + brief outcome]
- [Connected action showing causality]

### 📊 Activity Breakdown
- **Terminal:** [Summary of commands and their intent]
- **Research:** [URLs and findings if applicable]
- **Edits:** [Files modified and changes made]

### 🔗 Related to Prior Work
- [Connection to recent sessions if applicable, or "New focus"]

### ⚠️ Blockers or Notes
- [Any blockers, errors, or notable patterns]

"#;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: String,
}

pub async fn generate_summary(
    pool: &SqlitePool,
    ai_client: &AiClient,
    logs: Vec<ActivityLog>,
) -> Result<String, Error> {
    if logs.is_empty() {
        return Err(anyhow!("no logs provided for summary generation"));
    }

    let ai_settings = get_ai_settings(pool)
        .await
        .context("failed to load AI settings")?;

    // Fetch recent summaries for context (last 3)
    let recent_context = fetch_recent_summaries(pool, 3).await.unwrap_or_default();

    // Dynamic model selection
    let model_name = if ai_settings.provider_url.contains("openai.com") {
        "gpt-4o-mini"
    } else if ai_settings.provider_url.contains("anthropic.com") {
        "claude-3-5-sonnet-20241022"
    } else {
        "local-model"
    };

    let formatted_logs = format_logs(logs)?;
    
    // Build context prefix with recent summaries
    let context_prefix = if !recent_context.is_empty() {
        format!(
            "**RECENT CONTEXT (last sessions):**\n{}\n\n---\n\n",
            recent_context.join("\n\n")
        )
    } else {
        String::new()
    };

    let user_message = format!("{}{}", context_prefix, formatted_logs);

    let temperature = ai_settings.temperature.unwrap_or(0.2);

    let payload = ChatRequest {
        model: model_name.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_message,
            },
        ],
        temperature,
    };

    let response = ai_client
        .send_chat_completion(&ai_settings, &payload)
        .await?;

    let response = response
        .error_for_status()
        .context("AI provider returned an error status")?;

    let chat_response: ChatResponse = response
        .json()
        .await
        .context("failed to parse AI provider response")?;

    let summary = chat_response
        .choices
        .into_iter()
        .find_map(|choice| {
            let content = choice.message.content.trim().to_string();
            if content.is_empty() {
                None
            } else {
                Some(content)
            }
        })
        .ok_or_else(|| anyhow!("AI provider response did not contain a summary"))?;

    Ok(summary)
}

/// Fetch recent summaries for context
async fn fetch_recent_summaries(pool: &SqlitePool, limit: i64) -> Result<Vec<String>, Error> {
    let rows = sqlx::query_as::<_, AiReportRow>(
        "SELECT id, summary, generated_at, log_count, sources, session_id 
         FROM ai_reports 
         ORDER BY generated_at DESC 
         LIMIT ?1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    let summaries: Vec<String> = rows
        .into_iter()
        .map(|row| {
            // Extract first 500 chars of summary to avoid token bloat
            let summary_preview = if row.summary.len() > 500 {
                format!("{}...", &row.summary[..500])
            } else {
                row.summary
            };
            format!("**{}**\n{}", row.generated_at, summary_preview)
        })
        .collect();

    Ok(summaries)
}

fn format_logs(mut logs: Vec<ActivityLog>) -> Result<String, Error> {
    logs.sort_by_key(|log| log.timestamp);

    // Group logs by source for better readability
    let mut terminal_logs = Vec::new();
    let mut browser_logs = Vec::new();
    let mut vscode_logs = Vec::new();
    let mut other_logs = Vec::new();

    for log in logs {
        match log.source.as_str() {
            "terminal" => terminal_logs.push(log),
            "browser" => browser_logs.push(log),
            "vscode" => vscode_logs.push(log),
            _ => other_logs.push(log),
        }
    }

    let mut formatted = String::from("## Activity Logs by Source\n\n");

    if !terminal_logs.is_empty() {
        formatted.push_str("### Terminal Commands\n");
        for log in terminal_logs {
            if let Some(cmd) = &log.command {
                formatted.push_str(&format!("- `{}` at {}\n", cmd, log.timestamp.to_rfc3339()));
            } else {
                let payload = format_payload(&log.payload)?;
                formatted.push_str(&format!("- {}\n", payload));
            }
        }
        formatted.push('\n');
    }

    if !browser_logs.is_empty() {
        formatted.push_str("### Browser Activity\n");
        for log in browser_logs {
            let url = log.url.as_deref().unwrap_or("unknown");
            let title = log.title.as_deref().unwrap_or("");
            formatted.push_str(&format!("- {} - {} at {}\n", url, title, log.timestamp.to_rfc3339()));
        }
        formatted.push('\n');
    }

    if !vscode_logs.is_empty() {
        formatted.push_str("### Code Editor Activity\n");
        for log in vscode_logs {
            if let Some(file) = &log.file_path {
                let lang = log.title.as_deref().unwrap_or("unknown");
                formatted.push_str(&format!("- {} ({}) at {}\n", file, lang, log.timestamp.to_rfc3339()));
            }
        }
        formatted.push('\n');
    }

    if !other_logs.is_empty() {
        formatted.push_str("### Other Activity\n");
        for log in other_logs {
            let payload = format_payload(&log.payload)?;
            formatted.push_str(&format!("- [{}] {}\n", log.source, payload));
        }
    }

    Ok(formatted)
}

fn format_payload(payload: &Value) -> Result<String, Error> {
    let sanitized = sanitize_json(payload);
    serde_json::to_string_pretty(&sanitized).map_err(|err| anyhow!(err))
}

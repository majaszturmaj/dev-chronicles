pub mod client;

use anyhow::{anyhow, Context, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    ai::client::AiClient,
    db::{get_ai_settings, models::ActivityLog},
};
use sqlx::SqlitePool;

use crate::sanitizer::sanitize_json;

const SYSTEM_PROMPT: &str = r#"
**ROLE:**
You are an expert Technical Documentation Assistant. Your goal is to analyze raw activity logs (terminal commands and browser history) from a software developer's session and synthesize them into a concise, structured "Work Log" entry.
**INPUT DATA:**
You will receive a chronological stream of data covering a specific time interval (e.g., 10 minutes). The data includes:
1. **Terminal:** Shell commands (STDIN) and key outputs (STDOUT/STDERR).
2. **Browser:** Visited URLs, page titles, and potentially snippets of page content.
**INSTRUCTIONS:**
1. **Identify Intent:** Do not just list actions. Analyze the correlation between browser searches and terminal commands to determine the user's high-level goal (e.g., "Fixing CORS error in Nginx").
2. **Filter Noise:**
* Ignore trivial terminal navigation commands (`cd`, `ls`, `pwd`) unless relevant to the context.
* Ignore non-work-related browsing (social media, music, news) unless they appear to be the primary activity.
* Ignore repetitious typos or failed commands that were immediately corrected.
3. **Synthesize:** Group related actions.
* *Input:* User searched for "python list comprehension", then wrote a script, then ran it.
* *Output:* "Implemented data processing logic using Python list comprehensions."
4. **Security & Privacy (CRITICAL):**
* **NEVER** include passwords, API keys, secrets, or PII (Personally Identifiable Information) in the output.
* If a secret is detected in the logs, replace it with `[REDACTED_SECRET]`.
5. **Format:** Output strictly in Markdown.
**OUTPUT FORMAT:**
## ⏱️ [Time Range, e.g., 10:00 - 10:15]
**Context:** [One-sentence summary of the main focus]
### 🛠️ Key Actions
- [Bullet point describing a significant technical step]
- [Bullet point connecting a research step to an implementation step]
### 🔗 References & Resources
- [Link Title](URL) - [Brief note on why this was useful, e.g., "Solution for error X"]
### 💻 Notable Commands / Code
```bash
[Insert only the critical/successful commands or snippets here] 
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

    // 🔧 DYNAMIC MODEL SELECTION
    let model_name = if ai_settings.provider_url.contains("openai.com") {
        "gpt-4o-mini"  // or "gpt-3.5-turbo" for cheaper option
    } else if ai_settings.provider_url.contains("anthropic.com") {
        "claude-3-5-sonnet-20241022"
    } else {
        // LM Studio or other local models - use a generic name
        // The actual model is selected in LM Studio UI
        "local-model"
    };

    let formatted_logs = format_logs(logs)?;

    let payload = ChatRequest {
        model: model_name.to_string(),  // 🔧 Use dynamic model
        messages: vec![
            Message {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: formatted_logs,
            },
        ],
        temperature: 0.2,
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
        .ok_or_else(|| anyhow!("LM Studio response did not contain a summary"))?;

    Ok(summary)
}

fn format_logs(mut logs: Vec<ActivityLog>) -> Result<String, Error> {
    logs.sort_by_key(|log| log.timestamp);

    let lines = logs
        .into_iter()
        .map(|log| {
            let payload = format_payload(&log.payload)?;
            Ok(format!(
                "- [{}] {}\n  ```json\n{}\n  ```",
                log.timestamp.to_rfc3339(),
                log.source,
                payload
            ))
        })
        .collect::<Result<Vec<String>, Error>>()?;

    Ok(format!(
        "Chronological activity logs (oldest to newest):\n{}",
        lines.join("\n")
    ))
}

fn format_payload(payload: &Value) -> Result<String, Error> {
    let sanitized = sanitize_json(payload);
    serde_json::to_string_pretty(&sanitized).map_err(|err| anyhow!(err))
}

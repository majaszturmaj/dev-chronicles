use anyhow::{Context, Result};
use reqwest::Client;

use crate::db::models::AiSettings;

#[derive(Clone)]
pub struct AiClient {
    http: Client,
}

impl AiClient {
    pub fn new() -> Self {
        Self {
            http: Client::new(),
        }
    }

    pub async fn send_chat_completion<T: serde::ser::Serialize>(
        &self,
        settings: &AiSettings,
        payload: &T,
    ) -> Result<reqwest::Response> {
        let base = settings.provider_url.trim_end_matches('/');
        let endpoint = format!("{}/chat/completions", base);

        let mut request = self.http.post(&endpoint).json(payload);

        if let Some(api_key) = settings.api_key.as_ref().filter(|value| !value.is_empty()) {
            request = request.bearer_auth(api_key);
        }

        request
            .send()
            .await
            .with_context(|| format!("failed to send request to AI provider at {endpoint}"))
    }
}

// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! meta-client - HTTP client for the Meta metasearch connector.
use sentin8::middleware::http_client::Sentin8HttpClient;

#[derive(Clone)]
pub struct MetaClient {
    base_url: String,
    http: Sentin8HttpClient,
}

impl MetaClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http: Sentin8HttpClient::new(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub async fn health(&self) -> Result<serde_json::Value, reqwest::Error> {
        self.http
            .get(self.url("/v1/health"))
            .send()
            .await?
            .json()
            .await
    }
}

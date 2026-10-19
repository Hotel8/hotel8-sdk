// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! pos-client - HTTP client for the H8POS point-of-sale service.
use hotel8_pos_contracts::{Order, OrderCreate};
use sentin8::middleware::http_client::Sentin8HttpClient;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum POSError {
    #[error("H8POS HTTP error: {0}")]
    Http(String),
    #[error("H8POS decode error: {0}")]
    Decode(String),
}

#[derive(Clone)]
pub struct POSClient {
    base_url: String,
    auth_token: Arc<String>,
    http: Sentin8HttpClient,
}

impl POSClient {
    pub fn new(base_url: impl Into<String>, auth_token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            auth_token: Arc::new(auth_token.into()),
            http: Sentin8HttpClient::new(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub async fn health(&self) -> Result<serde_json::Value, POSError> {
        let resp = self
            .http
            .get(self.url("/v1/health"))
            .send()
            .await
            .map_err(|e| POSError::Http(e.to_string()))?;
        resp.json::<serde_json::Value>()
            .await
            .map_err(|e| POSError::Decode(e.to_string()))
    }

    pub async fn create_order(
        &self,
        outlet_id: uuid::Uuid,
        req: &OrderCreate,
    ) -> Result<Order, POSError> {
        let path = format!("/v1/outlets/{outlet_id}/orders");
        let resp = self
            .http
            .post(self.url(&path))
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .json(req)
            .send()
            .await
            .map_err(|e| POSError::Http(e.to_string()))?;
        resp.json::<Order>()
            .await
            .map_err(|e| POSError::Decode(e.to_string()))
    }
}

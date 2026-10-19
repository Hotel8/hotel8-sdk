// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! # pms-client
//!
//! Rust HTTP client for the PMS Property Management System service.
//!
//! Two identity modes (CERTFREE-4V):
//!
//! * **Workload certificate** ([`H8pmsClient::with_workload_certificate`]) -
//!   the standard service-to-service path: the CALLING service presents its
//!   own Acme-issued workload certificate (`m2m-{workload}` CN) in the TLS
//!   handshake; PMS's TLS layer authenticates it and the enroll-published
//!   registry map authorizes the scopes. No bearer token exists on this
//!   path.
//! * **Static bearer** ([`H8pmsClient::new`]) - user/delegated JWT flows
//!   where the caller already holds a token (BFF-adjacent callers).

use sentin8::middleware::http_client::Sentin8HttpClient;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum H8pmsError {
    #[error("PMS HTTP error: {0}")]
    Http(String),
    #[error("PMS response decode error: {0}")]
    Decode(String),
}

#[derive(Clone)]
pub struct H8pmsClient {
    base_url: String,
    /// Static bearer token (empty on the certificate path - no
    /// Authorization header is sent when it is empty).
    auth_token: Arc<String>,
    /// The presenting transport: a certificate-configured client on the
    /// workload path, a plain correlation-propagating client otherwise.
    http: Sentin8HttpClient,
}

impl H8pmsClient {
    /// Build a client that authenticates every call with a static bearer
    /// token - for user/delegated JWT flows where the caller already holds
    /// a token.
    pub fn new(base_url: impl Into<String>, auth_token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            auth_token: Arc::new(auth_token.into()),
            http: Sentin8HttpClient::new(),
        }
    }

    /// Build a certificate-presenting client for service-to-service calls
    /// (CERTFREE-4V): mint-or-load the CALLING service's own
    /// `m2m-{workload}` Acme certificate and wrap it in the mTLS
    /// transport. PMS's TLS layer authenticates the handshake and the
    /// enroll-published registry map authorizes the scopes - no bearer
    /// token exists on this path.
    ///
    /// `workload` is the CALLING service's registered workload name
    /// (e.g. `"upsell"`, `"loyalty"`) - it must appear in the calling
    /// service's `M2M_GRANTS` declaration so `vertical enroll` publishes
    /// its registry-map row.
    ///
    /// # Errors
    /// Returns the certificate-ensure or transport-build failure (the
    /// caller logs and degrades, or retries while the fleet boots).
    pub async fn with_workload_certificate(
        pms_url: &str,
        sentin8_auth_issuer: &str,
        workload: &str,
    ) -> Result<Self, String> {
        let cert = sentin8::auth::m2m::ensure_workload_certificate(
            sentin8_auth_issuer,
            workload,
            &sentin8::auth::m2m::platform_ca_path(),
        )
        .await
        .map_err(|e| format!("workload '{workload}' certificate: {e}"))?;
        let mtls = sentin8::middleware::mtls::MtlsConfig::new(
            &cert.ca_cert_path,
            &cert.cert_path,
            &cert.key_path,
        );
        let client = sentin8::middleware::mtls::build_mtls_client(&mtls)
            .map_err(|e| format!("workload '{workload}' transport: {e}"))?;
        Ok(Self {
            base_url: pms_url.trim_end_matches('/').to_string(),
            auth_token: Arc::new(String::new()),
            http: Sentin8HttpClient::from_client(client),
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// The Authorization header for the request - `None` on the
    /// certificate path (the TLS handshake IS the authentication; an
    /// empty static token means no header) and on token-less flows.
    fn auth_headers(&self) -> Result<Option<reqwest::header::HeaderMap>, H8pmsError> {
        if self.auth_token.is_empty() {
            return Ok(None);
        }
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", self.auth_token))
                .map_err(|e| H8pmsError::Http(format!("invalid token header: {e}")))?,
        );
        Ok(Some(headers))
    }

    async fn get_json(&self, path: &str) -> Result<serde_json::Value, H8pmsError> {
        let mut req = self.http.get(self.url(path));
        if let Some(headers) = self.auth_headers()? {
            req = req.headers(headers);
        }
        let resp = req
            .send()
            .await
            .map_err(|e| H8pmsError::Http(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(H8pmsError::Http(format!("status {}", resp.status())));
        }
        resp.json::<serde_json::Value>()
            .await
            .map_err(|e| H8pmsError::Decode(e.to_string()))
    }

    async fn post_json(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, H8pmsError> {
        let mut req = self.http.post(self.url(path));
        if let Some(headers) = self.auth_headers()? {
            req = req.headers(headers);
        }
        let resp = req
            .json(body)
            .send()
            .await
            .map_err(|e| H8pmsError::Http(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(H8pmsError::Http(format!("status {}", resp.status())));
        }
        resp.json::<serde_json::Value>()
            .await
            .map_err(|e| H8pmsError::Decode(e.to_string()))
    }

    async fn put_json(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, H8pmsError> {
        let mut req = self.http.put(self.url(path));
        if let Some(headers) = self.auth_headers()? {
            req = req.headers(headers);
        }
        let resp = req
            .json(body)
            .send()
            .await
            .map_err(|e| H8pmsError::Http(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(H8pmsError::Http(format!("status {}", resp.status())));
        }
        resp.json::<serde_json::Value>()
            .await
            .map_err(|e| H8pmsError::Decode(e.to_string()))
    }

    /// GET /v1/properties/{property_id}/reservations/{res_id}
    pub async fn get_reservation(
        &self,
        property_id: Uuid,
        reservation_id: Uuid,
    ) -> Result<serde_json::Value, H8pmsError> {
        let path = format!("/v1/properties/{property_id}/reservations/{reservation_id}");
        self.get_json(&path).await
    }

    /// POST /v1/properties/{property_id}/reservations/{id}/cancel
    pub async fn cancel_reservation(
        &self,
        property_id: Uuid,
        reservation_id: Uuid,
    ) -> Result<serde_json::Value, H8pmsError> {
        let path = format!("/v1/properties/{property_id}/reservations/{reservation_id}/cancel");
        self.post_json(&path, &serde_json::json!({})).await
    }

    /// POST /v1/properties/{property_id}/reservations/{id}/upsell -
    /// PMS-side room-type/rate-plan modification for an accepted upsell.
    pub async fn modify_reservation(
        &self,
        property_id: Uuid,
        reservation_id: Uuid,
        new_room_type_id: Option<Uuid>,
        new_rate_plan_id: Option<Uuid>,
    ) -> Result<serde_json::Value, H8pmsError> {
        let path = format!("/v1/properties/{property_id}/reservations/{reservation_id}/upsell");
        let body = serde_json::json!({
            "new_room_type_id": new_room_type_id.map(|u| u.to_string()),
            "new_rate_plan_id": new_rate_plan_id.map(|u| u.to_string()),
        });
        self.post_json(&path, &body).await
    }

    /// PUT /v1/properties/{property_id}/reservations/{res_id} (modify)
    pub async fn update_reservation(
        &self,
        property_id: Uuid,
        reservation_id: Uuid,
        req: &serde_json::Value,
    ) -> Result<serde_json::Value, H8pmsError> {
        let path = format!("/v1/properties/{property_id}/reservations/{reservation_id}");
        self.put_json(&path, req).await
    }

    /// GET /v1/properties/{property_id}/inventory?start_date=...&end_date=...
    pub async fn get_inventory(
        &self,
        property_id: Uuid,
        start_date: &str,
        end_date: &str,
    ) -> Result<serde_json::Value, H8pmsError> {
        let path = format!(
            "/v1/properties/{property_id}/inventory?start_date={start_date}&end_date={end_date}"
        );
        self.get_json(&path).await
    }

    /// GET /v1/properties/{property_id}/room-types
    pub async fn list_room_types(
        &self,
        property_id: Uuid,
    ) -> Result<Vec<serde_json::Value>, H8pmsError> {
        let path = format!("/v1/properties/{property_id}/room-types");
        let v = self.get_json(&path).await?;
        // The endpoint returns a bare JSON array.
        Ok(v.as_array().cloned().unwrap_or_default())
    }

    /// PUT /v1/properties/{property_id}/profiles/{profile_id}/loyalty-number
    pub async fn set_loyalty_number(
        &self,
        property_id: Uuid,
        profile_id: Uuid,
        loyalty_number: &str,
    ) -> Result<serde_json::Value, H8pmsError> {
        let path = format!("/v1/properties/{property_id}/profiles/{profile_id}/loyalty-number");
        let body = serde_json::json!({ "loyalty_number": loyalty_number });
        self.put_json(&path, &body).await
    }

    /// GET /v1/properties/{property_id}/profiles/{profile_id}
    pub async fn get_profile(
        &self,
        property_id: Uuid,
        profile_id: Uuid,
    ) -> Result<serde_json::Value, H8pmsError> {
        let path = format!("/v1/properties/{property_id}/profiles/{profile_id}");
        self.get_json(&path).await
    }

    /// GET /v1/properties/{property_id}/profiles?search=...
    pub async fn search_profiles(
        &self,
        property_id: Uuid,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Value, H8pmsError> {
        let qs = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");
        let path = format!("/v1/properties/{property_id}/profiles?{qs}");
        self.get_json(&path).await
    }

    /// GET /v1/properties/{property_id}/rate-plans/{plan_id}/schedules
    pub async fn list_rate_schedules(
        &self,
        property_id: Uuid,
        plan_id: Uuid,
    ) -> Result<serde_json::Value, H8pmsError> {
        let path = format!("/v1/properties/{property_id}/rate-plans/{plan_id}/schedules");
        self.get_json(&path).await
    }

    /// GET /v1/health
    pub async fn health(&self) -> Result<serde_json::Value, H8pmsError> {
        self.get_json("/v1/health").await
    }

    /// POST /v1/properties/{property_id}/reservations
    pub async fn create_reservation(
        &self,
        property_id: Uuid,
        req: &serde_json::Value,
    ) -> Result<serde_json::Value, H8pmsError> {
        let path = format!("/v1/properties/{property_id}/reservations");
        self.post_json(&path, req).await
    }

    /// GET /v1/properties/{property_id}/reservations
    pub async fn search_reservations(
        &self,
        property_id: Uuid,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Value, H8pmsError> {
        let qs = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");
        let path = format!("/v1/properties/{property_id}/reservations?{qs}");
        self.get_json(&path).await
    }

    /// GET /v1/properties/{property_id}/availability
    pub async fn search_availability(
        &self,
        property_id: Uuid,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Value, H8pmsError> {
        let qs = params
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&");
        let path = format!("/v1/properties/{property_id}/availability?{qs}");
        self.get_json(&path).await
    }

    /// POST /v1/properties/{property_id}/profiles
    pub async fn create_profile(
        &self,
        property_id: Uuid,
        req: &serde_json::Value,
    ) -> Result<serde_json::Value, H8pmsError> {
        let path = format!("/v1/properties/{property_id}/profiles");
        self.post_json(&path, req).await
    }

    /// POST /v1/properties/{property_id}/reservations/{id}/check-in
    pub async fn check_in(
        &self,
        property_id: Uuid,
        reservation_id: Uuid,
    ) -> Result<serde_json::Value, H8pmsError> {
        let path = format!("/v1/properties/{property_id}/reservations/{reservation_id}/check-in");
        self.post_json(&path, &serde_json::json!({})).await
    }

    /// POST /v1/properties/{property_id}/reservations/{id}/check-out
    pub async fn check_out(
        &self,
        property_id: Uuid,
        reservation_id: Uuid,
    ) -> Result<serde_json::Value, H8pmsError> {
        let path = format!("/v1/properties/{property_id}/reservations/{reservation_id}/check-out");
        self.post_json(&path, &serde_json::json!({})).await
    }
}

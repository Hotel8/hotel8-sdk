// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! meta-contracts - API types for the Meta metasearch connector.
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetasearchChannel {
    pub id: Uuid,
    pub property_id: Uuid,
    pub channel_code: String,
    pub channel_name: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetasearchChannelCreate {
    pub channel_code: String,
    pub channel_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateAdvertisement {
    pub channel_id: Uuid,
    pub room_type_code: String,
    pub rate_plan_code: String,
    pub date: chrono::NaiveDate,
    pub rate_amount: Decimal,
    pub currency_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}
// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - health
//
// The legacy `HealthResponse` above is the minimal 2-field handoff contract;
// the wire section below is the full row the `/v1/health` handler returns.
// ═══════════════════════════════════════════════════════════════════════════

/// Full `/v1/health` response: liveness plus build identity.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MetaHealth {
    pub status: String,
    pub service: String,
    /// Build identifier from the shared `sentin8-build-info` facade.
    pub version: String,
    /// Content hash of the build.
    pub build: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - channels (handlers.rs create/list/update/delete)
//
// Per-operation rows: create includes `property_id`, list rows do not
// (the SELECT projects only the ad-facing columns), and the update
// RETURNING adds `api_key_ref` (always present, `null` when unset).
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `POST /v1/channels`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelCreateRequest {
    pub property_id: Uuid,
    /// Ad-platform channel code (empty string when omitted).
    #[serde(default)]
    pub channel_code: Option<String>,
    #[serde(default)]
    pub channel_name: Option<String>,
    /// `meta.channel_type` label (default `GOOGLE`).
    #[serde(default)]
    pub channel_type: Option<String>,
}

/// Wire row of `POST /v1/channels` (creation projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelCreatedResponse {
    pub id: Uuid,
    pub property_id: Uuid,
    pub channel_code: String,
    pub channel_name: String,
    pub channel_type: String,
    pub is_active: bool,
}

/// Query parameters of `GET /v1/channels`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelListQuery {
    /// Filter by property; absent lists every channel in the environment.
    #[serde(default)]
    pub property_id: Option<Uuid>,
}

/// One item of `GET /v1/channels`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelListRow {
    pub id: Uuid,
    pub channel_code: String,
    pub channel_name: String,
    pub channel_type: String,
    pub is_active: bool,
}

/// Envelope of `GET /v1/channels`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelListResponse {
    pub items: Vec<ChannelListRow>,
}

/// Body of `PUT /v1/channels/{channel_id}` (all fields optional - absent
/// keys keep the stored value via `COALESCE`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelUpdateRequest {
    #[serde(default)]
    pub channel_name: Option<String>,
    #[serde(default)]
    pub is_active: Option<bool>,
    /// Store-by-reference pointer only (never a plaintext key).
    #[serde(default)]
    pub api_key_ref: Option<String>,
}

/// Wire row of `PUT /v1/channels/{channel_id}` (RETURNING projection;
/// `api_key_ref` is always present, `null` when no credential is stored).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelUpdatedResponse {
    pub id: Uuid,
    pub channel_code: String,
    pub channel_name: String,
    pub channel_type: String,
    pub is_active: bool,
    pub api_key_ref: Option<String>,
}

/// Envelope of `DELETE /v1/channels/{channel_id}` (200 with the deleted id;
/// the 404 body is a plain error string).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelDeletedResponse {
    pub deleted: bool,
    pub id: Uuid,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - channel configs (handlers.rs configure/list)
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `POST /v1/channel-configs` (upsert per `(channel_id, property_id)`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelConfigRequest {
    pub channel_id: Uuid,
    pub property_id: Uuid,
    /// Property-system hotel identifier passed through to the ad platform.
    #[serde(default)]
    pub hotel_id: Option<String>,
    /// Rate multiplier applied by `ad_pricing::adjusted_rate` (default `1.0`).
    #[serde(default)]
    pub rate_multiplier: Option<f64>,
    /// Booking landing page the ad deep-links to.
    #[serde(default)]
    pub landing_page_url: Option<String>,
    #[serde(default)]
    pub utm_source: Option<String>,
    #[serde(default)]
    pub utm_campaign: Option<String>,
    /// Upsert keeps configs enabled unless explicitly `false`.
    #[serde(default)]
    pub is_enabled: Option<bool>,
}

/// Wire row of a channel config (`meta.channel_configs` projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelConfigResponse {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub property_id: Uuid,
    pub hotel_id: String,
    /// `NUMERIC(5,2)::text` wire - decimal precision preserved as a string.
    pub rate_multiplier: String,
    pub landing_page_url: String,
    pub utm_source: String,
    pub utm_campaign: String,
    pub is_enabled: bool,
}

/// Envelope of `GET /v1/channels/{channel_id}/configs`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelConfigListResponse {
    pub items: Vec<ChannelConfigResponse>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - ARI sync / full sync (handlers.rs sync_ari/full_sync)
// ═══════════════════════════════════════════════════════════════════════════

/// Result of `POST /v1/channels/{channel_id}/sync` (single-channel ARI sync).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AriSyncResult {
    pub channel_id: Uuid,
    pub property_id: Uuid,
    /// Rate-ad rows inserted/updated in `meta.rate_ads`.
    pub records_pushed: i64,
    /// Multiplier actually applied to the pushed sell prices.
    pub rate_multiplier: f64,
}

/// Per-channel line of the `POST /v1/cron/full-sync` result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelSyncRow {
    pub channel_id: Uuid,
    pub property_id: Uuid,
    pub records_pushed: i64,
}

/// Result of `POST /v1/cron/full-sync` (all active channels, 365-day horizon).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FullSyncResult {
    pub channels_synced: usize,
    pub total_records_pushed: i64,
    pub results: Vec<ChannelSyncRow>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - rate verification (handlers.rs verify_rate_accuracy)
//
// The handler emits two distinct discrepancy shapes; the untagged enum
// keeps the wire exactly one-of-two while pinning both in the schema.
// ═══════════════════════════════════════════════════════════════════════════

/// One rate-accuracy discrepancy between `meta.rate_ads` and the PMS
/// inventory snapshot.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum RateDiscrepancy {
    /// The PMS inventory has no row for the ad's `(room_type, date)` -
    /// carries `source_rate: null`.
    Missing(RateDiscrepancyMissing),
    /// The ad's availability disagrees with the PMS snapshot.
    Mismatch(RateDiscrepancyMismatch),
}

/// Discrepancy variant `no_inventory_row`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RateDiscrepancyMissing {
    pub room_type_code: String,
    pub rate_plan_code: String,
    pub date: chrono::NaiveDate,
    /// Ad-side `NUMERIC(12,2)::text`.
    pub ad_rate: String,
    /// PMS-side rate; always `null` today (the inventory snapshot carries
    /// no rates).
    pub source_rate: Option<String>,
    /// `no_inventory_row`.
    pub reason: String,
}

/// Discrepancy variant `availability_mismatch`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct RateDiscrepancyMismatch {
    pub room_type_code: String,
    pub rate_plan_code: String,
    pub date: chrono::NaiveDate,
    /// Ad-side `NUMERIC(12,2)::text`.
    pub ad_rate: String,
    /// PMS snapshot availability (present in this variant).
    pub source_available: Option<i64>,
    /// Ad-side availability flag that mismatches.
    pub ad_available: bool,
    /// `availability_mismatch`.
    pub reason: String,
}

/// Result of `POST /v1/channels/{channel_id}/verify`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RateVerificationResult {
    pub channel_id: Uuid,
    pub discrepancy_count: usize,
    pub discrepancies: Vec<RateDiscrepancy>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - content push (handlers.rs push_content)
// ═══════════════════════════════════════════════════════════════════════════

/// Room-type projection pushed to the metasearch channel (sourced from the
/// PMS room-type API; `description` is `null` when the PMS has none).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContentRoomType {
    pub room_type_code: String,
    pub room_type_name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub max_occupancy: i64,
}

/// Result of `POST /v1/channels/{channel_id}/content`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContentPushResult {
    pub channel_id: Uuid,
    pub property_id: Uuid,
    pub room_types_pushed: usize,
    pub rooms: Vec<ContentRoomType>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - analytics (handlers.rs get_analytics)
// ═══════════════════════════════════════════════════════════════════════════

/// One daily bucket of the ad-clicks breakdown (last 30 days).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnalyticsDailyRow {
    pub date: chrono::NaiveDate,
    pub clicks: i64,
    /// `NUMERIC(12,4)::text` daily cost.
    pub cost: String,
}

/// Revenue/cost pair of the analytics summary.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnalyticsRevenue {
    /// Revenue is not yet tracked; reported as `"0"`.
    pub revenue: String,
    /// `NUMERIC(12,4)::text` total cost.
    pub cost: String,
}

/// Result of `GET /v1/admin/properties/{property_id}/analytics`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnalyticsResponse {
    pub property_id: Uuid,
    pub clicks: i64,
    /// `NUMERIC(12,4)::text` total cost per click.
    pub total_cost: String,
    pub channels: i64,
    /// Proxied from clicks pending an impressions table.
    pub impressions: i64,
    pub roi: AnalyticsRevenue,
    pub daily: Vec<AnalyticsDailyRow>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - UTM deep links (handlers.rs generate_utm)
// ═══════════════════════════════════════════════════════════════════════════

/// UTM parameter set mirrored into the `deep_link` query parameters
/// (`term`/`content` are always present, empty strings when unset).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UtmParams {
    pub source: String,
    pub medium: String,
    pub campaign: String,
    pub term: String,
    pub content: String,
}

/// Result of `POST /v1/utm/generate`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UtmDeepLink {
    pub deep_link: String,
    pub utm: UtmParams,
}

/// Body of `POST /v1/utm/generate` (all fields optional - each absent field
/// falls back to its handler default).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UtmGenerateRequest {
    /// Booking base URL the link is built on
    /// (default `https://book.example.com`).
    #[serde(default)]
    pub base_url: Option<String>,
    /// Default `google`.
    #[serde(default)]
    pub utm_source: Option<String>,
    /// Default `cpc`.
    #[serde(default)]
    pub utm_medium: Option<String>,
    #[serde(default)]
    pub utm_campaign: Option<String>,
    /// Appended to the link only when non-empty.
    #[serde(default)]
    pub utm_term: Option<String>,
    /// Appended to the link only when non-empty.
    #[serde(default)]
    pub utm_content: Option<String>,
    #[serde(default)]
    pub property_id: Option<String>,
    /// `YYYY-MM-DD`, appended when non-empty.
    #[serde(default)]
    pub checkin: Option<String>,
    /// `YYYY-MM-DD`, appended when non-empty.
    #[serde(default)]
    pub checkout: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - channel credentials (handlers.rs encrypt_credentials)
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `POST /v1/channels/{channel_id}/credentials`. The plaintext api
/// key is consumed by the handler and envelope-encrypted with the platform
/// master key; only the ciphertext (and a channel-row reference) is stored.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CredentialRequest {
    pub api_key: String,
}

/// Result of `POST /v1/channels/{channel_id}/credentials`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CredentialStored {
    pub channel_id: Uuid,
    /// `hotel8://meta/channel/{channel_id}/api_key` - store-by-reference.
    pub secret_ref: String,
    pub stored: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - rate limits (handlers.rs get_rate_limits)
// ═══════════════════════════════════════════════════════════════════════════

/// Local concurrency/rate budget for one sync family.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RateLimitFamily {
    pub max_concurrent: i64,
    pub rate_per_minute: i64,
    pub burst: i64,
}

/// ARI batching budget (size/interval instead of concurrency).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RateLimitAri {
    pub batch_size: i64,
    pub interval_seconds: i64,
}

/// Result of `GET /v1/rate-limits`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RateLimits {
    pub sync: RateLimitFamily,
    pub content: RateLimitFamily,
    pub ari: RateLimitAri,
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests - contract round-trips (H8T-6)
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn channel_round_trip() {
        let c = MetasearchChannel {
            id: Uuid::new_v4(),
            property_id: Uuid::new_v4(),
            channel_code: "GOOGLE".into(),
            channel_name: "Google Hotel Ads".into(),
            is_active: true,
            created_at: chrono::Utc::now(),
        };
        let back: MetasearchChannel =
            serde_json::from_value(serde_json::to_value(&c).unwrap()).unwrap();
        assert_eq!(back.channel_code, "GOOGLE");
        assert!(back.is_active);
    }

    #[test]
    fn rate_advertisement_decimal_date_round_trip() {
        let r = RateAdvertisement {
            channel_id: Uuid::new_v4(),
            room_type_code: "DELX".into(),
            rate_plan_code: "BAR".into(),
            date: chrono::NaiveDate::from_ymd_opt(2026, 9, 10).unwrap(),
            rate_amount: Decimal::new(199_99, 2),
            currency_code: "EUR".into(),
        };
        let back: RateAdvertisement =
            serde_json::from_value(serde_json::to_value(&r).unwrap()).unwrap();
        assert_eq!(back.rate_amount, Decimal::new(199_99, 2));
        assert_eq!(back.date.to_string(), "2026-09-10");
    }

    #[test]
    fn channel_wire_rows_match_handler_projections() {
        let created = ChannelCreatedResponse {
            id: Uuid::new_v4(),
            property_id: Uuid::new_v4(),
            channel_code: "GOOGLE".into(),
            channel_name: "Google Hotel Ads".into(),
            channel_type: "GOOGLE".into(),
            is_active: true,
        };
        let back: ChannelCreatedResponse =
            serde_json::from_value(serde_json::to_value(&created).unwrap()).unwrap();
        assert_eq!(back.channel_code, "GOOGLE");
        assert_eq!(back.channel_type, "GOOGLE");

        // List rows must not carry property_id (the handler SELECT omits it).
        let row = ChannelListRow {
            id: Uuid::new_v4(),
            channel_code: "KAYAK".into(),
            channel_name: "Kayak".into(),
            channel_type: "KAYAK".into(),
            is_active: false,
        };
        let v = serde_json::to_value(&row).unwrap();
        assert!(v.get("property_id").is_none());
        assert_eq!(v["is_active"], false);

        // api_key_ref is always present on the update row (null when unset).
        let updated = ChannelUpdatedResponse {
            id: Uuid::new_v4(),
            channel_code: "GOOGLE".into(),
            channel_name: "Google Hotel Ads".into(),
            channel_type: "GOOGLE".into(),
            is_active: true,
            api_key_ref: None,
        };
        let v = serde_json::to_value(&updated).unwrap();
        assert_eq!(v["api_key_ref"], serde_json::Value::Null);
    }

    #[test]
    fn rate_discrepancy_variants_route_both_ways() {
        let missing = RateDiscrepancyMissing {
            room_type_code: "DELX".into(),
            rate_plan_code: "BAR".into(),
            date: chrono::NaiveDate::from_ymd_opt(2026, 10, 1).unwrap(),
            ad_rate: "0".into(),
            source_rate: None,
            reason: "no_inventory_row".into(),
        };
        let v = serde_json::to_value(&missing).unwrap();
        let back: RateDiscrepancy = serde_json::from_value(v).unwrap();
        match back {
            RateDiscrepancy::Missing(m) => assert!(m.source_rate.is_none()),
            RateDiscrepancy::Mismatch(m) => panic!("misrouted variant: {m:?}"),
        }

        let mismatch = RateDiscrepancyMismatch {
            room_type_code: "SUP".into(),
            rate_plan_code: "BAR".into(),
            date: chrono::NaiveDate::from_ymd_opt(2026, 10, 2).unwrap(),
            ad_rate: "129.00".into(),
            source_available: Some(3),
            ad_available: false,
            reason: "availability_mismatch".into(),
        };
        let v = serde_json::to_value(&mismatch).unwrap();
        let back: RateDiscrepancy = serde_json::from_value(v).unwrap();
        match back {
            RateDiscrepancy::Mismatch(m) => assert_eq!(m.source_available, Some(3)),
            RateDiscrepancy::Missing(m) => panic!("misrouted variant: {m:?}"),
        }
    }
}

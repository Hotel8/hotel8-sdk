// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! distribution-contracts - API types for the Distrib distribution service.
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: Uuid,
    pub property_id: Uuid,
    pub channel_code: String,
    pub channel_name: String,
    pub channel_type: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelCreate {
    pub channel_code: String,
    pub channel_name: String,
    pub channel_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelRateUpdate {
    pub channel_id: Uuid,
    pub room_type_id: Uuid,
    pub rate_plan_code: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub rate_amount: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

// serde field defaults (file scope so `default = ".."` references resolve).
fn default_one() -> i32 {
    1
}

fn default_usd() -> String {
    "USD".into()
}

fn default_ota() -> String {
    "OTA".into()
}

fn default_confirmed() -> String {
    "CONFIRMED".into()
}

fn default_modified() -> String {
    "MODIFIED".into()
}

// ════════════════════════════════════════════════════════════════════════
// Wire shapes (OAPEC-9): the admin/channel API rows the handlers return.
// The legacy Channel/ChannelCreate structs above are the client-engine
// contracts; these pin the wire rows (NUMERIC via `::text` as String).
// ═══════════════════════════════════════════════════════════════════════════

// ── Health ──

/// `GET /v1/health` - liveness + build info (no pool probe on this service).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthDetail {
    pub status: String,
    pub service: String,
    pub version: String,
    pub build: String,
}

// ── Channels ──

/// Body of `POST /v1/channels` (`channel_type` defaults OTA).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelCreateRequest {
    pub property_id: Uuid,
    #[serde(default)]
    pub channel_code: String,
    #[serde(default)]
    pub channel_name: String,
    #[serde(default = "default_ota")]
    pub channel_type: String,
}

/// Created channel (the INSERT RETURNING projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelCreated {
    pub id: Uuid,
    pub property_id: Uuid,
    pub channel_code: String,
    pub channel_name: String,
    pub channel_type: String,
    pub is_active: bool,
}

/// One row of the channels list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelRow {
    pub id: Uuid,
    pub channel_code: String,
    pub channel_name: String,
    pub channel_type: String,
    pub is_active: bool,
}

/// Wrapper list returned by the channels endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelList {
    pub items: Vec<ChannelRow>,
}

/// Body of `PUT /v1/channels/{channel_id}` (partial update).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelUpdateRequest {
    pub channel_name: Option<String>,
    pub channel_type: Option<String>,
    pub is_active: Option<bool>,
    pub api_endpoint: Option<String>,
}

/// Updated channel row (no `property_id` - the query projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelUpdated {
    pub id: Uuid,
    pub channel_code: String,
    pub channel_name: String,
    pub channel_type: String,
    pub is_active: bool,
    pub api_endpoint: Option<String>,
}

// ── Channel-property mapping ──

/// Body of `POST /v1/channel-properties`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelPropertyMapRequest {
    pub channel_id: Uuid,
    pub property_id: Uuid,
    #[serde(default = "default_true")]
    pub is_enabled: bool,
}

/// Mapped channel-property row (the upsert RETURNING projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelPropertyMapped {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub property_id: Uuid,
    pub is_enabled: bool,
}

/// One row of the mapped-properties list (NUMERIC markups as text).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelPropertyRow {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub property_id: Uuid,
    pub is_enabled: bool,
    /// NUMERIC markup percentage as text.
    pub rate_markup_pct: String,
    /// NUMERIC markdown percentage as text.
    pub rate_markdown_pct: String,
    pub cta_enabled: bool,
    pub ctd_enabled: bool,
    pub min_los: Option<i32>,
    pub max_los: Option<i32>,
}

/// Wrapper list returned by the mapped-properties endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelPropertyList {
    pub items: Vec<ChannelPropertyRow>,
}

/// Body of the rate markup/markdown PUT.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RateMarkupRequest {
    /// Decimal strings (defaults "0").
    #[serde(default = "default_zero_str")]
    pub rate_markup_pct: String,
    #[serde(default = "default_zero_str")]
    pub rate_markdown_pct: String,
}

/// Rate markup/markdown update result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RateMarkupResult {
    pub id: Uuid,
    /// NUMERIC percentages as text.
    pub rate_markup_pct: String,
    pub rate_markdown_pct: String,
}

/// Body of the property-controls PUT (all fields optional - partial update).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PropertyControlsRequest {
    pub cta_enabled: Option<bool>,
    pub ctd_enabled: Option<bool>,
    pub min_los: Option<i32>,
    pub max_los: Option<i32>,
}

/// Property-controls update result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PropertyControlsResult {
    pub id: Uuid,
    pub cta_enabled: bool,
    pub ctd_enabled: bool,
    pub min_los: Option<i32>,
    pub max_los: Option<i32>,
}

// ── Room-type/rate-plan mappings ──

/// Body of `POST /v1/room-type-mappings`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoomTypeMapRequest {
    pub channel_property_id: Uuid,
    pub property_room_type_id: Uuid,
    pub channel_room_type_code: String,
}

/// Room-type mapping result (the upsert RETURNING projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoomTypeMapped {
    pub id: Uuid,
    pub channel_property_id: Uuid,
    pub property_room_type_id: Uuid,
    pub channel_room_type_code: String,
}

/// Body of `POST /v1/rate-plan-mappings`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RatePlanMapRequest {
    pub channel_property_id: Uuid,
    pub property_rate_plan_id: Uuid,
    pub channel_rate_plan_code: String,
}

/// Rate-plan mapping result (the upsert RETURNING projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RatePlanMapped {
    pub id: Uuid,
    pub channel_property_id: Uuid,
    pub property_rate_plan_id: Uuid,
    pub channel_rate_plan_code: String,
}

// ── ARI / content / controls push + sync ──

/// Body of the ARI push (`start_date`/`end_date` accepted for API compat -
/// the push uses a rolling 30-day window).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AriPushRequest {
    /// Scopes the push to one property (required for the PMS fetch).
    pub property_id: Option<Uuid>,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
}

/// ARI push result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AriPushResult {
    pub channel_id: Uuid,
    pub records_pushed: usize,
    pub status: String,
}

/// Content push request (property is optional - omit for all mapped).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContentPushRequest {
    pub property_id: Option<Uuid>,
}

/// Content/controls push acknowledgement (shared by both push endpoints).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PushAck {
    pub channel_id: Uuid,
    pub status: String,
}

/// Failed-sync retry result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncRetryResult {
    pub retried_count: usize,
}

// ── Reservation delivery ──

/// Body of `POST /v1/channels/{channel_code}/reservations`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelReservationRequest {
    #[serde(default)]
    pub channel_booking_id: String,
    #[serde(default)]
    pub guest_name: String,
    /// YYYY-MM-DD (defaults 2026-01-01 when absent).
    #[serde(default = "default_arrival")]
    pub arrival_date: String,
    /// YYYY-MM-DD (defaults 2026-01-02 when absent).
    #[serde(default = "default_departure")]
    pub departure_date: String,
    #[serde(default = "default_one")]
    pub num_rooms: i32,
    /// Decimal string (defaults "0").
    #[serde(default = "default_zero_str")]
    pub total_amount: String,
    /// ISO-4217 (defaults USD).
    #[serde(default = "default_usd")]
    pub currency_code: String,
}

/// Reservation-delivery result (booking bound to the PMS reservation).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelReservationDelivered {
    pub booking_id: Uuid,
    pub reservation_id: Uuid,
    pub status: String,
}

/// Body of the channel-reservation update.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelReservationUpdate {
    /// DELIVERED | MODIFIED | CANCELLED (defaults MODIFIED).
    #[serde(default = "default_modified")]
    pub status: String,
}

/// Channel-reservation update result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelReservationUpdateResult {
    pub booking_id: Uuid,
    pub reservation_id: Option<Uuid>,
    pub status: String,
}

/// Body of the reservation-status push.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReservationStatusPush {
    #[serde(default = "default_confirmed")]
    pub status: String,
}

/// Reservation-status push acknowledgement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReservationStatusPushed {
    pub booking_id: Uuid,
    pub status: String,
    pub pushed: bool,
}

// ── Channel-facing reads ──

/// One sellable property of a channel.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelSellableProperty {
    pub property_id: Uuid,
    pub is_enabled: bool,
    pub cta_enabled: bool,
    pub ctd_enabled: bool,
    pub min_los: Option<i32>,
    pub max_los: Option<i32>,
}

/// `GET /v1/channels/{channel_code}/properties`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelSellableProperties {
    pub channel_code: String,
    pub properties: Vec<ChannelSellableProperty>,
}

/// One room type of the channel content read (PMS projection passthrough).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelContentRoomType {
    pub code: serde_json::Value,
    pub name: serde_json::Value,
    pub max_occupancy: serde_json::Value,
    pub bed_type: serde_json::Value,
    pub smoking: serde_json::Value,
}

/// `GET .../properties/{property_code}/content`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelPropertyContent {
    pub channel_code: String,
    pub property_id: Uuid,
    /// Raw property_code path segment (parsed to `property_id` when valid).
    pub property_code: String,
    pub room_types: Vec<ChannelContentRoomType>,
}

/// One ARI line of the channel availability read (PMS projection passthrough).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelAriLine {
    pub room_type_id: serde_json::Value,
    pub date: serde_json::Value,
    pub available: serde_json::Value,
    pub sell_limit: serde_json::Value,
}

/// `GET .../properties/{property_code}/ari`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelAri {
    pub channel_code: String,
    pub property_id: String,
    pub ari: Vec<ChannelAriLine>,
}

/// Booking controls for a property.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelControls {
    pub cta_enabled: bool,
    pub ctd_enabled: bool,
    pub min_los: Option<i32>,
    pub max_los: Option<i32>,
}

/// `GET .../properties/{property_code}/controls`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelControlsResponse {
    pub channel_code: String,
    pub property_id: String,
    pub controls: ChannelControls,
}

/// Channel credential validation result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelCredentialValidation {
    pub channel_code: String,
    pub valid: bool,
}

// ── Sync log ──

/// One sync-log row.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncLogRow {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub property_id: Option<Uuid>,
    /// ARI | CONTENT | CONTROLS.
    pub sync_type: String,
    pub status: String,
    pub attempts: i32,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Wrapper list returned by the sync-log endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncLogList {
    pub items: Vec<SyncLogRow>,
}

// Shared serde defaults across the wire-shape structs (private file scope).
fn default_true() -> bool {
    true
}

fn default_zero_str() -> String {
    "0".into()
}

fn default_arrival() -> String {
    "2026-01-01".into()
}

fn default_departure() -> String {
    "2026-01-02".into()
}

// ════════════════════════════════════════════════════════════════════════
// Tests - contract round-trips (H8T-6)
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn channel_round_trip() {
        let c = Channel {
            id: Uuid::new_v4(),
            property_id: Uuid::new_v4(),
            channel_code: "EXPEDIA".into(),
            channel_name: "Expedia Partner Central".into(),
            channel_type: "OTA".into(),
            is_active: true,
            created_at: chrono::Utc::now(),
        };
        let back: Channel = serde_json::from_value(serde_json::to_value(&c).unwrap()).unwrap();
        assert_eq!(back.channel_code, "EXPEDIA");
        assert_eq!(back.channel_type, "OTA");
    }

    #[test]
    fn channel_create_requires_type() {
        let c: ChannelCreate = serde_json::from_value(serde_json::json!({
            "channel_code": "BOOKING",
            "channel_name": "Booking.com",
            "channel_type": "OTA"
        }))
        .unwrap();
        assert_eq!(c.channel_type, "OTA");
    }

    #[test]
    fn rate_update_date_range_round_trip() {
        let u = ChannelRateUpdate {
            channel_id: Uuid::new_v4(),
            room_type_id: Uuid::new_v4(),
            rate_plan_code: "BAR".into(),
            start_date: chrono::NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
            end_date: chrono::NaiveDate::from_ymd_opt(2026, 9, 30).unwrap(),
            rate_amount: Decimal::new(175_00, 2),
        };
        let back: ChannelRateUpdate =
            serde_json::from_value(serde_json::to_value(&u).unwrap()).unwrap();
        assert_eq!(back.rate_amount, Decimal::new(175_00, 2));
        assert_eq!(back.end_date.to_string(), "2026-09-30");
    }
}

// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! upsell-contracts - API types for the Upsell upsell engine.
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offer {
    pub id: Uuid,
    pub property_id: Uuid,
    pub offer_code: String,
    pub offer_name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub category: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfferCreate {
    pub offer_code: String,
    pub offer_name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsellEligibilityRequest {
    pub reservation_id: Uuid,
    pub room_type_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsellEligibilityResponse {
    pub eligible_offers: Vec<Offer>,
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
pub struct UpsellHealth {
    pub status: String,
    pub service: String,
    /// Build identifier from the shared `sentin8-build-info` facade.
    pub version: String,
    /// Content hash of the build.
    pub build: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - offers CRUD (handlers.rs create/list/update/configure)
//
// `price` stays a JSON string per the `NUMERIC(12,2)::text` wire.
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `POST /v1/offers`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferCreateRequest {
    pub property_id: Uuid,
    /// Offer code (empty string when omitted).
    #[serde(default)]
    pub offer_code: Option<String>,
    #[serde(default)]
    pub offer_name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    /// `NUMERIC(12,2)` amount as a string (default `"0"`).
    #[serde(default)]
    pub price: Option<String>,
    /// Default `UPGRADE`.
    #[serde(default)]
    pub category: Option<String>,
}

/// Wire row of `POST /v1/offers` (creation projection; no description).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferCreatedResponse {
    pub id: Uuid,
    pub property_id: Uuid,
    pub offer_code: String,
    pub offer_name: String,
    /// `NUMERIC(12,2)::text`.
    pub price: String,
    pub status: String,
}

/// Query parameters of `GET /v1/offers`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferListQuery {
    /// Filter by property; absent lists all offers in the environment.
    #[serde(default)]
    pub property_id: Option<Uuid>,
}

/// One item of `GET /v1/offers`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferListRow {
    pub id: Uuid,
    pub offer_code: String,
    pub offer_name: String,
    /// `NUMERIC(12,2)::text`.
    pub price: String,
    pub category: String,
    pub status: String,
}

/// Envelope of `GET /v1/offers`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferListResponse {
    pub items: Vec<OfferListRow>,
}

/// Body of `PUT /v1/offers/{offer_id}` (all fields optional - absent keys
/// keep the stored value via `COALESCE`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferUpdateRequest {
    #[serde(default)]
    pub offer_name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    /// `NUMERIC(12,2)` amount as a string.
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    /// An `upsell.offer_status` label (`DRAFT`/`ACTIVE`/`PAUSED`/`EXPIRED`/`ARCHIVED`).
    #[serde(default)]
    pub status: Option<String>,
}

/// Wire row of `PUT /v1/offers/{offer_id}`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferUpdatedResponse {
    pub id: Uuid,
    pub offer_code: String,
    pub offer_name: String,
    /// `NUMERIC(12,2)::text`.
    pub price: String,
    pub category: String,
    pub status: String,
}

/// Body of `PUT /v1/offers/{offer_id}/configure` (pricing strategy config).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferConfigureRequest {
    /// `FIXED` | `DYNAMIC` (default `FIXED`).
    #[serde(default)]
    pub pricing_strategy: Option<String>,
    /// Room type the upsell moves the guest into.
    #[serde(default)]
    pub upgraded_room_type: Option<String>,
    /// ISO currency (default `USD`).
    #[serde(default)]
    pub currency_code: Option<String>,
    /// `NUMERIC(12,2)` amount as a string (absent keeps the stored price).
    #[serde(default)]
    pub price: Option<String>,
}

/// Wire row of `PUT /v1/offers/{offer_id}/configure` (RETURNING projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferConfiguredResponse {
    pub id: Uuid,
    pub offer_code: String,
    pub offer_name: String,
    /// `NUMERIC(12,2)::text`.
    pub price: String,
    pub pricing_strategy: String,
    #[serde(default)]
    pub upgraded_room_type: Option<String>,
    pub currency_code: String,
    pub status: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - eligibility rules (handlers.rs create_rule/list_rules/delete_rule/test_rule)
//
// `rule_value` is a free-form JSONB predicate payload by design.
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `POST /v1/rules` (one eligibility rule of an offer).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RuleCreateRequest {
    pub offer_id: Uuid,
    /// Rule predicate key, e.g. one of `GET /v1/eligibility/types` codes.
    pub rule_type: String,
    /// Free-form predicate payload (JSONB, default `{}`).
    #[serde(default)]
    pub rule_value: Option<serde_json::Value>,
}

/// Wire row of `POST /v1/rules`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RuleCreatedResponse {
    pub id: Uuid,
    pub offer_id: Uuid,
    pub rule_type: String,
    /// Free-form predicate payload (JSONB).
    pub rule_value: serde_json::Value,
}

/// Query parameters of `GET /v1/rules`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RuleListQuery {
    /// Filter by offer; absent lists up to 100 rules (no other paging).
    #[serde(default)]
    pub offer_id: Option<Uuid>,
}

/// One item of `GET /v1/rules`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RuleListRow {
    pub id: Uuid,
    pub offer_id: Uuid,
    pub rule_type: String,
    /// Free-form predicate payload (JSONB).
    pub rule_value: serde_json::Value,
}

/// Envelope of `GET /v1/rules`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RuleListResponse {
    pub items: Vec<RuleListRow>,
}

/// Body of `POST /v1/rules/test` (all fields optional; a `reservation_id`
/// without a `property_id` is a 400).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RuleTestRequest {
    #[serde(default)]
    pub offer_id: Option<Uuid>,
    #[serde(default)]
    pub reservation_id: Option<Uuid>,
    #[serde(default)]
    pub property_id: Option<Uuid>,
}

/// One dry-run predicate check of `POST /v1/rules/test`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RuleCheckResult {
    /// Rule key evaluated (`reservation_exists` for the PMS lookup probe).
    pub rule_type: String,
    pub passed: bool,
}

/// Result of `POST /v1/rules/test` (dry-run; no state is written).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RuleTestResult {
    pub eligible: bool,
    pub checks: Vec<RuleCheckResult>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - offer generation (handlers.rs generate_offers,
// get_offers_for_reservation, cron_daily_generation)
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `POST /v1/offers/generate`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GenerateOffersRequest {
    pub property_id: Uuid,
    /// Reservation the transaction is attached to (optional seed).
    #[serde(default)]
    pub reservation_id: Option<Uuid>,
}

/// One generated upsell transaction of `POST /v1/offers/generate`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GeneratedOffer {
    pub transaction_id: Uuid,
    pub offer_id: Uuid,
    pub offer_code: String,
    pub offer_name: String,
    /// `NUMERIC(12,2)::text` offer price.
    pub price: String,
}

/// Result of `POST /v1/offers/generate`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GenerateOffersResult {
    pub generated: Vec<GeneratedOffer>,
}

/// Query parameters of `GET /v1/offers/for-reservation`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OffersForReservationQuery {
    pub reservation_id: Uuid,
}

/// One transaction row of `GET /v1/offers/for-reservation` (LEFT JOIN offer
/// columns are `null` when the offer was deleted).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferForReservationRow {
    pub transaction_id: Uuid,
    /// `OFFERED` | `ACCEPTED` | `DECLINED` | `EXPIRED`
    pub status: String,
    #[serde(default)]
    pub offer_code: Option<String>,
    #[serde(default)]
    pub offer_name: Option<String>,
    /// `NUMERIC(12,2)::text` offer price.
    #[serde(default)]
    pub offer_price: Option<String>,
    /// `NUMERIC(12,2)::text` amount actually charged on acceptance.
    #[serde(default)]
    pub price_charged: Option<String>,
}

/// Envelope of `GET /v1/offers/for-reservation` (newest first).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OffersForReservationResponse {
    pub items: Vec<OfferForReservationRow>,
}

/// Body of `POST /v1/cron/daily-generation` (absent `property_id` is a 400).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CronDailyGenerationRequest {
    #[serde(default)]
    pub property_id: Option<Uuid>,
}

/// Result of `POST /v1/cron/daily-generation`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CronDailyGenerationResult {
    pub total_generated: i64,
    /// PMS reservations in the 7-day CONFIRMED/CHECKED_IN window.
    pub reservations_processed: usize,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - accept / decline / status (handlers.rs accept_offer,
// decline_offer, get_offer_status)
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `POST /v1/offers/accept`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AcceptOfferRequest {
    pub transaction_id: Uuid,
    /// `NUMERIC(12,2)` amount as a string (default `"0"`).
    #[serde(default)]
    pub price_charged: Option<String>,
}

/// Result of `POST /v1/offers/accept`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferAcceptedResponse {
    pub transaction_id: Uuid,
    /// `ACCEPTED`.
    pub status: String,
    pub accepted: bool,
}

/// Body of `POST /v1/offers/decline`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeclineOfferRequest {
    pub transaction_id: Uuid,
    /// Free-form decline reason (event payload only).
    #[serde(default)]
    pub reason: Option<String>,
}

/// Result of `POST /v1/offers/decline`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferDeclinedResponse {
    pub transaction_id: Uuid,
    /// `DECLINED`.
    pub status: String,
    pub declined: bool,
}

/// Query parameters of `GET /v1/offers/status`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferStatusQuery {
    pub transaction_id: Uuid,
}

/// Result of `GET /v1/offers/status`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfferStatusResponse {
    pub transaction_id: Uuid,
    /// `OFFERED` | `ACCEPTED` | `DECLINED` | `EXPIRED`
    pub status: String,
    #[serde(default)]
    pub accepted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub declined_at: Option<chrono::DateTime<chrono::Utc>>,
    /// `NUMERIC(12,2)::text` amount charged (null until accepted).
    #[serde(default)]
    pub price_charged: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - analytics / expiry (handlers.rs get_analytics, expire_offers)
// ═══════════════════════════════════════════════════════════════════════════

/// Query parameters of `GET /v1/analytics`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnalyticsQuery {
    /// Filter by property; absent aggregates the environment.
    #[serde(default)]
    pub property_id: Option<Uuid>,
    /// `YYYY-MM-DD` lower bound (default `2020-01-01`).
    #[serde(default)]
    pub start_date: Option<String>,
    /// `YYYY-MM-DD` upper bound (default `2030-12-31`).
    #[serde(default)]
    pub end_date: Option<String>,
}

/// Result of `GET /v1/analytics`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AnalyticsResponse {
    pub total_offers: i64,
    pub accepted: i64,
    pub declined: i64,
    pub expired: i64,
    /// Acceptance rate over decided transactions, formatted `"NN.NN%"`.
    pub acceptance_rate: String,
    /// `NUMERIC(12,2)::text` sum of accepted `price_charged`.
    pub revenue_uplift: String,
}

/// Body of `POST /v1/offers/expire`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExpireOffersRequest {
    /// Filter by property; absent expires environment-wide.
    #[serde(default)]
    pub property_id: Option<Uuid>,
}

/// Result of `POST /v1/offers/expire`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExpireOffersResult {
    pub expired_count: i64,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - eligibility types (handlers.rs list_eligibility_types)
// ═══════════════════════════════════════════════════════════════════════════

/// One supported eligibility rule type.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EligibilityType {
    /// Predicate key (e.g. `GUEST_TIER`).
    pub code: String,
    /// Human-readable description of the predicate.
    pub description: String,
}

/// Result of `GET /v1/eligibility/types`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EligibilityTypesResponse {
    pub types: Vec<EligibilityType>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - PMS integration passthrough
// (handlers.rs fetch_hotel8_pms_data) - PMS projections are free-form
// JSON passthrough by contract; only the envelope is pinned.
// ═══════════════════════════════════════════════════════════════════════════

/// Query parameters of `GET /v1/integration/pms`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PmsIntegrationQuery {
    #[serde(default)]
    pub property_id: Option<Uuid>,
    #[serde(default)]
    pub reservation_id: Option<Uuid>,
}

/// Result of `GET /v1/integration/pms` - the PMS reservation/profile
/// projections and the 30-day inventory rows pass through exactly as the
/// PMS API shaped them.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PmsDataEnvelope {
    /// PMS reservation projection (null when no `reservation_id` was given).
    pub reservation: Option<serde_json::Value>,
    /// 30-day inventory rows (empty when no `property_id` was given).
    pub inventory: Vec<serde_json::Value>,
    /// PMS guest profile projection (null when unresolvable).
    pub profile: Option<serde_json::Value>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - dynamic pricing (handlers.rs compute_dynamic_price)
// ═══════════════════════════════════════════════════════════════════════════

/// Pricing inputs echoed in the result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DynamicPriceFactors {
    pub occupancy_rate: f64,
    pub days_before_arrival: i64,
    /// Guest loyalty tier hint (`NONE` default).
    pub guest_tier: String,
}

/// Body of `POST /v1/integration/func/dynamic-price`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DynamicPriceRequest {
    /// Baseline room price (default `100.0`).
    #[serde(default)]
    pub base_price: Option<f64>,
    /// Occupancy ratio 0..1 (default `0.5`).
    #[serde(default)]
    pub occupancy_rate: Option<f64>,
    /// Lead time in days (default `7`).
    #[serde(default)]
    pub days_before_arrival: Option<i64>,
    /// Guest tier hint (default `NONE`).
    #[serde(default)]
    pub guest_tier: Option<String>,
}

/// Result of `POST /v1/integration/func/dynamic-price` (Func-authoritative
/// when a pricing module is deployed, local rule otherwise).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DynamicPriceResult {
    pub base_price: f64,
    /// Suggested upsell price.
    pub suggested_price: f64,
    pub factors: DynamicPriceFactors,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - Post / Payments / PMS integration ops
// (handlers.rs send_offer_notification, send_acceptance_confirmation,
// process_upgrade_payment, modify_reservation_on_acceptance)
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `POST /v1/integration/post/notify`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NotificationRequest {
    #[serde(default)]
    pub transaction_id: Option<Uuid>,
    /// Delivery channel (default `EMAIL`).
    #[serde(default)]
    pub channel: Option<String>,
}

/// Result of `POST /v1/integration/post/notify` (queued for Post delivery).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NotificationQueued {
    #[serde(default)]
    pub transaction_id: Option<Uuid>,
    pub channel: String,
    /// `queued`.
    pub status: String,
}

/// Body of `POST /v1/integration/post/confirmation`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConfirmationRequest {
    #[serde(default)]
    pub transaction_id: Option<Uuid>,
}

/// Result of `POST /v1/integration/post/confirmation`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConfirmationQueued {
    #[serde(default)]
    pub transaction_id: Option<Uuid>,
    /// `queued`.
    pub status: String,
}

/// Body of `POST /v1/integration/payments/payment`. The token is a
/// single-use payment-method token - never a raw PAN.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpgradePaymentRequest {
    pub transaction_id: Uuid,
    /// Single-use payment-method token (Bill8 PAY-10).
    pub payment_method_token: String,
    /// Charge amount (default `0.0`).
    #[serde(default)]
    pub amount: Option<f64>,
    /// ISO currency (default `USD`).
    #[serde(default)]
    pub currency_code: Option<String>,
}

/// Result of `POST /v1/integration/payments/payment`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpgradePaymentResult {
    pub transaction_id: Uuid,
    /// Bill8:Payments charge id.
    pub payment_id: Uuid,
    /// `PROCESSED`.
    pub status: String,
    pub payment_method_token: String,
}

/// Body of `POST /v1/integration/pms/modify-reservation`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModifyReservationRequest {
    pub transaction_id: Uuid,
    /// Room type the upsell moves the guest into.
    #[serde(default)]
    pub new_room_type_id: Option<Uuid>,
    /// Travelled with the PMS upsell call in a future story; recorded on
    /// the transaction only.
    #[serde(default)]
    pub new_rate: Option<String>,
}

/// Result of `POST /v1/integration/pms/modify-reservation` (`modified` is
/// false when the transaction carries no reservation/property pair).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModifyReservationResult {
    pub transaction_id: Uuid,
    #[serde(default)]
    pub reservation_id: Option<Uuid>,
    pub modified: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - A/B testing (handlers.rs create_ab_test, get_ab_test_results)
// ═══════════════════════════════════════════════════════════════════════════

/// One A/B variant rule to create.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AbTestVariant {
    /// Variant label (default `A`).
    #[serde(default)]
    pub variant_label: Option<String>,
    /// Predicate key (default `GUEST_TIER`).
    #[serde(default)]
    pub rule_type: Option<String>,
    /// Free-form predicate payload (JSONB, default `{}`).
    #[serde(default)]
    pub rule_value: Option<serde_json::Value>,
}

/// Body of `POST /v1/ab-tests`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAbTestRequest {
    pub offer_id: Uuid,
    /// At least one variant (empty array is a 400).
    pub variants: Vec<AbTestVariant>,
}

/// One created variant line of `POST /v1/ab-tests`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AbVariantCreated {
    pub rule_id: Uuid,
    pub variant_label: String,
    pub rule_type: String,
}

/// Result of `POST /v1/ab-tests`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateAbTestResult {
    pub offer_id: Uuid,
    pub variants: Vec<AbVariantCreated>,
}

/// Query parameters of `GET /v1/ab-tests/results`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AbTestResultsQuery {
    /// Filter by offer; absent aggregates per variant across the environment.
    #[serde(default)]
    pub offer_id: Option<Uuid>,
}

/// One variant bucket of `GET /v1/ab-tests/results`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AbTestResultRow {
    pub variant_label: String,
    pub total: i64,
    pub accepted: i64,
    /// Acceptance rate over the variant's transactions, formatted `"NN.NN%"`.
    pub acceptance_rate: String,
    /// `NUMERIC(12,2)::text` accepted revenue of the variant.
    pub revenue: String,
}

/// Result of `GET /v1/ab-tests/results`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AbTestResultsResponse {
    pub variants: Vec<AbTestResultRow>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes - offer rate limits (handlers.rs get_rate_limit_config)
// ═══════════════════════════════════════════════════════════════════════════

/// Per-minute/per-hour budget of one upsell op family.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RateLimitWindow {
    pub max_per_minute: i64,
    pub max_per_hour: i64,
}

/// Result of `GET /v1/rate-limits`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RateLimitConfig {
    pub offer_generation: RateLimitWindow,
    pub offer_acceptance: RateLimitWindow,
    pub rule_testing: RateLimitWindow,
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests - contract round-trips (H8T-6)
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    fn sample_offer(id: Uuid) -> Offer {
        Offer {
            id,
            property_id: Uuid::new_v4(),
            offer_code: "UPG-DLX".into(),
            offer_name: "Deluxe Upgrade".into(),
            description: Some("Upgrade to Deluxe King".into()),
            price: Decimal::new(79_00, 2),
            category: "ROOM_UPGRADE".into(),
            is_active: true,
            created_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn offer_round_trip() {
        let o = sample_offer(Uuid::new_v4());
        let back: Offer = serde_json::from_value(serde_json::to_value(&o).unwrap()).unwrap();
        assert_eq!(back.offer_code, "UPG-DLX");
        assert_eq!(back.price, Decimal::new(79_00, 2));
    }

    #[test]
    fn offer_create_defaults_description_to_none() {
        let c: OfferCreate = serde_json::from_value(serde_json::json!({
            "offer_code": "UPG-SUITE",
            "offer_name": "Suite Upgrade",
            "price": "150.00",
            "category": "ROOM_UPGRADE"
        }))
        .unwrap();
        assert_eq!(c.description, None);
        assert_eq!(c.price, Decimal::new(150_00, 2));
    }

    #[test]
    fn eligibility_response_carries_offers() {
        let r = UpsellEligibilityResponse {
            eligible_offers: vec![sample_offer(Uuid::new_v4())],
        };
        let back: UpsellEligibilityResponse =
            serde_json::from_value(serde_json::to_value(&r).unwrap()).unwrap();
        assert_eq!(back.eligible_offers.len(), 1);
        assert_eq!(back.eligible_offers[0].category, "ROOM_UPGRADE");
    }

    #[test]
    fn rate_limit_config_round_trip() {
        let cfg = RateLimitConfig {
            offer_generation: RateLimitWindow {
                max_per_minute: 100,
                max_per_hour: 5000,
            },
            offer_acceptance: RateLimitWindow {
                max_per_minute: 50,
                max_per_hour: 2000,
            },
            rule_testing: RateLimitWindow {
                max_per_minute: 20,
                max_per_hour: 500,
            },
        };
        let back: RateLimitConfig =
            serde_json::from_value(serde_json::to_value(&cfg).unwrap()).unwrap();
        assert_eq!(back.offer_generation.max_per_minute, 100);
        assert_eq!(back.rule_testing.max_per_hour, 500);
    }

    #[test]
    fn transaction_rows_keep_null_optionals_optional() {
        // for-reservation row: LEFT JOIN optionals serialize as null keys.
        let row = OfferForReservationRow {
            transaction_id: Uuid::new_v4(),
            status: "OFFERED".into(),
            offer_code: None,
            offer_name: None,
            offer_price: None,
            price_charged: None,
        };
        let v = serde_json::to_value(&row).unwrap();
        assert_eq!(v["offer_code"], serde_json::Value::Null);
        let back: OfferForReservationRow = serde_json::from_value(v).unwrap();
        assert_eq!(back.status, "OFFERED");
        assert!(back.offer_code.is_none());

        // status row: accepted_at/declined_at null until transitions happen.
        let st = OfferStatusResponse {
            transaction_id: Uuid::new_v4(),
            status: "OFFERED".into(),
            accepted_at: None,
            declined_at: None,
            price_charged: None,
        };
        let v = serde_json::to_value(&st).unwrap();
        assert_eq!(v["accepted_at"], serde_json::Value::Null);
        assert_eq!(v["price_charged"], serde_json::Value::Null);
    }

    #[test]
    fn eligibility_types_wire_shape() {
        let r = EligibilityTypesResponse {
            types: vec![EligibilityType {
                code: "GUEST_TIER".into(),
                description: "Guest VIP tier".into(),
            }],
        };
        let v = serde_json::to_value(&r).unwrap();
        assert_eq!(v["types"][0]["code"], "GUEST_TIER");
    }
}

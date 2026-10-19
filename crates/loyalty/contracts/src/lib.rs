// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! loyalty-contracts - API types for the Loyalty loyalty program service.
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub id: Uuid,
    pub env_id: Uuid,
    pub program_code: String,
    pub program_name: String,
    pub currency_code: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramCreate {
    pub program_code: String,
    pub program_name: String,
    pub currency_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: Uuid,
    pub program_id: Uuid,
    pub env_id: Uuid,
    pub member_number: String,
    pub tier: String,
    pub points_balance: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberCreate {
    pub program_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointsTransaction {
    pub id: Uuid,
    pub member_id: Uuid,
    pub env_id: Uuid,
    pub transaction_type: String,
    pub points: i64,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AwardPointsRequest {
    pub member_id: Uuid,
    pub points: i64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RedeemPointsRequest {
    pub member_id: Uuid,
    pub points: i64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

// serde field defaults (file scope so default = ".." references resolve).
fn default_pts() -> String {
    "PTS".into()
}

fn default_zero() -> i64 {
    0
}

fn default_base_tier() -> String {
    "BASE".into()
}

fn default_general_cat() -> String {
    "GENERAL".into()
}

fn default_reward_type() -> String {
    "DISCOUNT".into()
}

fn default_validity() -> i32 {
    365
}

fn default_adjust_desc() -> String {
    "Manual adjustment".into()
}

fn default_inactive_months() -> i32 {
    24
}

// Wire shapes (OAPEC-9): the admin/member API rows the handlers return. The
// legacy Program/Member/PointsTransaction structs above are the client-engine
// contracts; these pin the wire rows. Tier benefits stay free-form JSONB.

/// GET /v1/health - liveness + build info (service label is "loyal").
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthDetail {
    pub status: String,
    pub service: String,
    pub version: String,
    pub build: String,
}

/// Body of POST /v1/programs (currency_code defaults PTS).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProgramCreateRequest {
    #[serde(default)]
    pub program_code: String,
    #[serde(default)]
    pub program_name: String,
    #[serde(default = "default_pts")]
    pub currency_code: String,
}

/// Created program (the INSERT RETURNING projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProgramCreated {
    pub id: Uuid,
    pub env_id: Uuid,
    pub program_code: String,
    pub program_name: String,
    pub currency_code: String,
}

/// One row of the programs list (NUMERIC accrual rate as text).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProgramRow {
    pub id: Uuid,
    pub program_code: String,
    pub program_name: String,
    /// LOYALTY_NOW | POINTS_BASED | HYBRID.
    pub program_type: String,
    pub currency_code: String,
    /// NUMERIC points-per-currency as text.
    pub points_per_currency: String,
    pub is_active: bool,
}

/// Wrapper list returned by the programs endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProgramList {
    pub items: Vec<ProgramRow>,
}

/// Body of POST /v1/members.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MemberCreateRequest {
    pub program_id: Uuid,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    pub email: Option<String>,
}

/// Created member (the INSERT RETURNING projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MemberCreated {
    pub id: Uuid,
    pub program_id: Uuid,
    pub member_number: String,
    pub tier: String,
    pub points_balance: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
}

/// Full member detail (GET /v1/members/{member_id}).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MemberDetail {
    pub id: Uuid,
    pub program_id: Uuid,
    pub member_number: String,
    pub tier: String,
    pub points_balance: i64,
    pub lifetime_points: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub enrolled_at: chrono::DateTime<chrono::Utc>,
}

/// One row of the member search list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MemberSearchRow {
    pub id: Uuid,
    pub program_id: Uuid,
    pub member_number: String,
    pub tier: String,
    pub points_balance: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
}

/// Wrapper list returned by the member-search endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MemberList {
    pub items: Vec<MemberSearchRow>,
}

/// Points-award result (the inserted EARN transaction).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PointsAwarded {
    pub id: Uuid,
    pub member_id: Uuid,
    pub points: i64,
    /// Transaction type label ("EARN").
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Points-redeem result (the inserted REDEEM transaction, negative points).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PointsRedeemed {
    pub id: Uuid,
    pub member_id: Uuid,
    pub points: i64,
    /// Transaction type label ("REDEEM").
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Manual points-adjustment body (POST .../adjust).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AdjustPointsRequest {
    #[serde(default = "default_zero")]
    pub points: i64,
    #[serde(default = "default_adjust_desc")]
    pub description: String,
}

/// Points-adjustment result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PointsAdjustmentResult {
    pub id: Uuid,
    pub member_id: Uuid,
    pub points: i64,
    /// Transaction type label ("ADJUST").
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Body of POST /v1/points/calculate (stay-points accrual).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PointsCalculationRequest {
    pub reservation_id: Option<Uuid>,
    pub member_id: Uuid,
    /// Decimal-string eligible spend (defaults "0").
    #[serde(default = "default_zero_str")]
    pub eligible_spend: String,
}

/// Stay-points accrual result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PointsCalculation {
    pub transaction_id: Uuid,
    pub member_id: Uuid,
    pub points_earned: i64,
    pub tier: String,
    pub multiplier: f64,
}

/// One transaction row of the member history.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PointsTransactionRow {
    pub id: Uuid,
    /// Transaction type label (EARN | REDEEM | ADJUST | EXPIRE).
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub points: i64,
    pub description: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Wrapper list of the member transaction history.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MemberTransactions {
    pub items: Vec<PointsTransactionRow>,
}

/// Body of POST /v1/tiers (benefits is a free-form JSONB bag by design).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TierConfigureRequest {
    pub program_id: Uuid,
    /// Tier label (defaults BASE).
    #[serde(default = "default_base_tier")]
    pub tier: String,
    #[serde(default = "default_zero")]
    pub min_points: i64,
    /// Free-form benefits bag (defaults {}), stored as provided.
    #[serde(default = "empty_object")]
    pub benefits: serde_json::Value,
}

/// Configured tier rule (benefits as stored).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TierRuleRow {
    pub id: Uuid,
    pub program_id: Uuid,
    pub tier: String,
    pub min_points: i64,
    /// Free-form benefits bag as stored.
    pub benefits: serde_json::Value,
}

/// Wrapper list returned by the tiers endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TierList {
    pub items: Vec<TierRuleRow>,
}

/// Query of the tiers endpoint (optional program filter).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TierListRequest {
    pub program_id: Option<Uuid>,
}

/// Body of POST /v1/programs/{program_id}/rewards.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RewardCreateRequest {
    pub program_id: Uuid,
    #[serde(default)]
    pub item_code: String,
    #[serde(default)]
    pub item_name: String,
    pub description: Option<String>,
    #[serde(default = "default_zero")]
    pub points_cost: i64,
    #[serde(default = "default_general_cat")]
    pub category: String,
    #[serde(default = "default_reward_type")]
    pub reward_type: String,
    #[serde(default = "default_validity")]
    pub voucher_validity_days: i32,
}

/// Created reward (the INSERT RETURNING projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RewardCreated {
    pub id: Uuid,
    pub program_id: Uuid,
    pub item_code: String,
    pub item_name: String,
    pub points_cost: i64,
    pub category: String,
    pub reward_type: String,
    pub voucher_validity_days: i32,
    pub is_active: bool,
}

/// One row of the rewards list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RewardRow {
    pub id: Uuid,
    pub item_code: String,
    pub item_name: String,
    pub description: Option<String>,
    pub points_cost: i64,
    pub category: String,
    pub reward_type: String,
    pub is_active: bool,
}

/// Wrapper list returned by the rewards endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RewardList {
    pub items: Vec<RewardRow>,
}

/// Body of PUT .../rewards/{reward_id} (partial update).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RewardUpdateRequest {
    pub item_name: Option<String>,
    pub points_cost: Option<i64>,
    pub is_active: Option<bool>,
}

/// Reward-update result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RewardUpdated {
    pub id: Uuid,
    pub item_code: String,
    pub item_name: String,
    pub points_cost: i64,
    pub is_active: bool,
}

/// One reward of the member redeemable catalog with affordability flag.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MemberRewardView {
    pub id: Uuid,
    pub item_code: String,
    pub item_name: String,
    pub description: Option<String>,
    pub points_cost: i64,
    pub category: String,
    pub reward_type: String,
    /// True when the member balance covers the cost.
    pub redeemable: bool,
}

/// GET /v1/members/{member_id}/rewards.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MemberRewardList {
    pub member_id: Uuid,
    pub points_balance: i64,
    pub rewards: Vec<MemberRewardView>,
}

/// Body of POST /v1/rewards/redeem.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RedeemRewardRequest {
    pub member_id: Uuid,
    pub catalog_item_id: Uuid,
}

/// Reward redemption (voucher issued).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RedemptionIssued {
    pub redemption_id: Uuid,
    pub voucher_code: String,
    pub member_id: Uuid,
    pub item_name: String,
    pub points_spent: i64,
    pub remaining_balance: i64,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// Body of POST /v1/members/link-guest.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LinkedGuestProfile {
    pub member_id: Uuid,
    pub guest_profile_id: Uuid,
    pub property_id: Uuid,
}

/// Guest-link result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MemberGuestLink {
    pub member_id: Uuid,
    pub guest_profile_id: Uuid,
    pub linked: bool,
}

/// Body of POST /v1/cron/expire-points.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExpirePointsRequest {
    #[serde(default = "default_inactive_months")]
    pub inactivity_months: i32,
}

/// Points-expiry run result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExpiryResult {
    pub expired_count: usize,
    pub inactivity_months: i32,
}

/// Tier-review run result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TierReviewResult {
    pub upgraded: usize,
    pub downgraded: usize,
}

fn empty_object() -> serde_json::Value {
    serde_json::json!({})
}

fn default_zero_str() -> String {
    "0".into()
}
// ═══════════════════════════════════════════════════════════════════════════
// Tests - contract round-trips (H8T-6)
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_round_trip() {
        let p = Program {
            id: Uuid::new_v4(),
            env_id: Uuid::new_v4(),
            program_code: "STARS".into(),
            program_name: "Sentin8 Stars".into(),
            currency_code: "PTS".into(),
            is_active: true,
            created_at: chrono::Utc::now(),
        };
        let back: Program = serde_json::from_value(serde_json::to_value(&p).unwrap()).unwrap();
        assert_eq!(back.program_code, "STARS");
        assert!(back.is_active);
    }

    #[test]
    fn program_create_defaults_currency_to_none() {
        let c: ProgramCreate = serde_json::from_value(serde_json::json!({
            "program_code": "NIGHTS",
            "program_name": "Sentin8 Nights"
        }))
        .unwrap();
        assert_eq!(c.currency_code, None);
    }

    #[test]
    fn member_round_trip_with_optional_email() {
        let m = Member {
            id: Uuid::new_v4(),
            program_id: Uuid::new_v4(),
            env_id: Uuid::new_v4(),
            member_number: "M-000001".into(),
            tier: "GOLD".into(),
            points_balance: 12_500,
            first_name: "Ada".into(),
            last_name: "Lovelace".into(),
            email: Some("ada@example.test".into()),
            created_at: chrono::Utc::now(),
        };
        let back: Member = serde_json::from_value(serde_json::to_value(&m).unwrap()).unwrap();
        assert_eq!(back.points_balance, 12_500);
        assert_eq!(back.tier, "GOLD");
    }

    #[test]
    fn points_transaction_records_earn_and_redeem() {
        for (t_type, points) in [("EARN", 500i64), ("REDEEM", -200i64)] {
            let t = PointsTransaction {
                id: Uuid::new_v4(),
                member_id: Uuid::new_v4(),
                env_id: Uuid::new_v4(),
                transaction_type: t_type.into(),
                points,
                description: None,
                created_at: chrono::Utc::now(),
            };
            let back: PointsTransaction =
                serde_json::from_value(serde_json::to_value(&t).unwrap()).unwrap();
            assert_eq!(back.transaction_type, t_type);
            assert_eq!(back.points, points);
            assert!(back.description.is_none());
        }
    }

    #[test]
    fn award_and_redeem_requests_round_trip() {
        let a: AwardPointsRequest = serde_json::from_value(serde_json::json!({
            "member_id": Uuid::new_v4().to_string(),
            "points": 100
        }))
        .unwrap();
        assert_eq!(a.points, 100);
        assert_eq!(a.description, None);
        let r: RedeemPointsRequest = serde_json::from_value(serde_json::json!({
            "member_id": Uuid::new_v4().to_string(),
            "points": 50,
            "description": "Free night"
        }))
        .unwrap();
        assert_eq!(r.points, 50);
        assert_eq!(r.description.as_deref(), Some("Free night"));
    }
}

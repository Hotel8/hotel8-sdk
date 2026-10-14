// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! loyalty-contracts — API types for the Loyalty loyalty program service.
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwardPointsRequest {
    pub member_id: Uuid,
    pub points: i64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
// ═══════════════════════════════════════════════════════════════════════════
// Tests — contract round-trips (H8T-6)
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

// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! distribution-contracts — API types for the Distrib distribution service.
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
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
// ═══════════════════════════════════════════════════════════════════════════
// Tests — contract round-trips (H8T-6)
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

// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! meta-contracts — API types for the Meta metasearch connector.
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

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
// Tests — contract round-trips (H8T-6)
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
        let back: MetasearchChannel = serde_json::from_value(serde_json::to_value(&c).unwrap()).unwrap();
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
        let back: RateAdvertisement = serde_json::from_value(serde_json::to_value(&r).unwrap()).unwrap();
        assert_eq!(back.rate_amount, Decimal::new(199_99, 2));
        assert_eq!(back.date.to_string(), "2026-09-10");
    }
}

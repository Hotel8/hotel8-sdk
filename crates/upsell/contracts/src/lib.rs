// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! upsell-contracts — API types for the Upsell upsell engine.
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

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
// Tests — contract round-trips (H8T-6)
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
}

// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! pos-contracts — API types for the H8POS point-of-sale service.
//!
//! Covers outlets, menus, orders, payments, and kitchen display system integration.
//! See `hotel8/design/sentin8-hotel8-pos-technical-design.md` for the full design.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Outlets ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outlet {
    pub id: Uuid,
    pub property_id: Uuid,
    pub outlet_code: String,
    pub outlet_name: String,
    pub outlet_type: String,
    pub currency_code: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutletCreate {
    pub outlet_code: String,
    pub outlet_name: String,
    pub outlet_type: String,
    pub currency_code: Option<String>,
}

// ── Menu Items ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub item_code: String,
    pub item_name: String,
    pub category: String,
    pub price: Decimal,
    pub tax_rate: Decimal,
    pub is_active: bool,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItemCreate {
    pub item_code: String,
    pub item_name: String,
    pub category: String,
    pub price: Decimal,
    pub tax_rate: Option<Decimal>,
    pub description: Option<String>,
}

// ── Orders ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub order_number: String,
    pub table_number: Option<String>,
    pub status: String,
    pub items: Vec<OrderItem>,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub total: Decimal,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub menu_item_id: Uuid,
    pub item_name: String,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub line_total: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreate {
    pub table_number: Option<String>,
    pub items: Vec<OrderItemCreate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemCreate {
    pub menu_item_id: Uuid,
    pub quantity: i32,
}

// ── Health ──

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

    fn sample_outlet(id: Uuid) -> Outlet {
        Outlet {
            id,
            property_id: Uuid::new_v4(),
            outlet_code: "REST01".into(),
            outlet_name: "Main Restaurant".into(),
            outlet_type: "RESTAURANT".into(),
            currency_code: "EUR".into(),
            is_active: true,
            created_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn outlet_round_trip() {
        let o = sample_outlet(Uuid::new_v4());
        let back: Outlet = serde_json::from_value(serde_json::to_value(&o).unwrap()).unwrap();
        assert_eq!(back.outlet_code, "REST01");
        assert!(back.is_active);
    }

    #[test]
    fn outlet_create_defaults_currency_to_none() {
        let c: OutletCreate = serde_json::from_value(serde_json::json!({
            "outlet_code": "BAR01",
            "outlet_name": "Lobby Bar",
            "outlet_type": "BAR"
        }))
        .unwrap();
        assert_eq!(c.currency_code, None);
    }

    #[test]
    fn menu_item_with_decimal_price_round_trip() {
        let m = MenuItem {
            id: Uuid::new_v4(),
            outlet_id: Uuid::new_v4(),
            item_code: "BGR-01".into(),
            item_name: "Signature Burger".into(),
            category: "MAINS".into(),
            price: Decimal::new(18_50, 2),
            tax_rate: Decimal::new(10, 2),
            is_active: true,
            description: None,
            created_at: chrono::Utc::now(),
        };
        let back: MenuItem = serde_json::from_value(serde_json::to_value(&m).unwrap()).unwrap();
        assert_eq!(back.price, Decimal::new(18_50, 2));
        assert_eq!(back.tax_rate, Decimal::new(10, 2));
        assert!(back.description.is_none());
    }

    #[test]
    fn menu_item_create_defaults_tax_rate_to_none() {
        let c: MenuItemCreate = serde_json::from_value(serde_json::json!({
            "item_code": "CKT-01",
            "item_name": "Cheesecake",
            "category": "DESSERT",
            "price": "8.50"
        }))
        .unwrap();
        assert_eq!(c.price, Decimal::new(8_50, 2));
        assert_eq!(c.tax_rate, None);
    }

    #[test]
    fn order_with_items_round_trip() {
        let o = Order {
            id: Uuid::new_v4(),
            outlet_id: Uuid::new_v4(),
            order_number: "CHK-1".into(),
            table_number: Some("12".into()),
            status: "SENT".into(),
            items: vec![OrderItem {
                menu_item_id: Uuid::new_v4(),
                item_name: "Signature Burger".into(),
                quantity: 2,
                unit_price: Decimal::new(18_50, 2),
                line_total: Decimal::new(37_00, 2),
            }],
            subtotal: Decimal::new(37_00, 2),
            tax_total: Decimal::new(3_70, 2),
            total: Decimal::new(40_70, 2),
            created_at: chrono::Utc::now(),
        };
        let back: Order = serde_json::from_value(serde_json::to_value(&o).unwrap()).unwrap();
        assert_eq!(back.items.len(), 1);
        assert_eq!(back.items[0].line_total, Decimal::new(37_00, 2));
        assert_eq!(back.total, Decimal::new(40_70, 2));
    }

    #[test]
    fn health_response_round_trip() {
        let h = HealthResponse {
            status: "ok".into(),
            service: "pos".into(),
        };
        let back: HealthResponse =
            serde_json::from_value(serde_json::to_value(&h).unwrap()).unwrap();
        assert_eq!(back.service, "pos");
    }
}

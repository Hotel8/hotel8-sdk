// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! pos-contracts - API types for the H8POS point-of-sale service.
//!
//! Covers outlets, menus, orders, payments, and kitchen display system integration.
//! See `hotel8/design/sentin8-hotel8-pos-technical-design.md` for the full design.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// ── Outlets ──

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

// serde field defaults (declared at file scope so `default = ".."` references resolve).
fn default_zero_str() -> String {
    "0".into()
}

fn default_one() -> i32 {
    1
}

fn default_cash_ptype() -> String {
    "CASH".into()
}

fn default_today() -> String {
    "today".into()
}
// (default_schedule_start/default_schedule_end/default_csv_format/etc. are
// declared at the file bottom next to their sibling defaults)

// ═══════════════════════════════════════════════════════════════════════════
// Wire shapes (OAPEC-9): the admin/guest API rows the handlers return.
// Legacy Outlet/MenuItem structs above are the client-engine contracts;
// these pin the wire rows (NUMERIC via `::text` as String). ToSchema on every
// member so `docs/openapi.json` response schemas are real.
// ═══════════════════════════════════════════════════════════════════════════

// ── Health & outlets wire ──

/// `GET /v1/health` - liveness + build info + pool probe.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthDetail {
    pub status: String,
    pub service: String,
    pub version: String,
    pub build: String,
    /// Pool connectivity probe ("connected" | "disconnected").
    pub database: String,
}

/// Created outlet (the INSERT RETURNING projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OutletCreated {
    pub id: Uuid,
    pub property_id: Uuid,
    pub outlet_code: String,
    pub outlet_name: String,
    pub outlet_type: String,
    pub currency_code: String,
}

/// Body of `POST /v1/outlets` (`outlet_type` defaults RESTAURANT,
/// `currency_code` defaults USD).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OutletCreateRequest {
    pub property_id: Uuid,
    #[serde(default)]
    pub outlet_code: String,
    #[serde(default)]
    pub outlet_name: String,
    /// RESTAURANT | BAR | ROOM_SERVICE | ... (defaults RESTAURANT).
    #[serde(default = "default_outlet_type")]
    pub outlet_type: String,
    /// ISO-4217 (defaults USD).
    #[serde(default = "default_usd_code")]
    pub currency_code: String,
}

/// Wrapper list returned by the outlets endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OutletList {
    pub items: Vec<OutletRow>,
}

/// One outlet row of the outlets list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OutletRow {
    pub id: Uuid,
    pub property_id: Uuid,
    pub outlet_code: String,
    pub outlet_name: String,
    pub outlet_type: String,
    pub currency_code: String,
    pub is_active: bool,
}

// ── Menu items wire ──

/// Created menu item (NUMERIC price/tax_rate as text).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MenuItemCreated {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub item_code: String,
    pub item_name: String,
    /// NUMERIC price as text.
    pub price: String,
    /// NUMERIC tax rate as text.
    pub tax_rate: String,
}

/// One menu-item row of the items list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MenuItemRow {
    pub id: Uuid,
    pub item_code: String,
    pub item_name: String,
    pub description: Option<String>,
    /// NUMERIC price as text.
    pub price: String,
    /// NUMERIC tax rate as text.
    pub tax_rate: String,
    pub is_active: bool,
}

/// Wrapper list returned by the menu-items endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MenuItemList {
    pub items: Vec<MenuItemRow>,
}

/// Body of `POST /v1/outlets/{outlet_id}/menu-items`
/// (price/tax_rate are decimal strings, defaults "0"; the response carries
/// the inserted ids as `MenuItemCreated`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MenuItemCreateRequest {
    #[serde(default)]
    pub item_code: String,
    #[serde(default)]
    pub item_name: String,
    pub category_id: Option<Uuid>,
    #[serde(default = "default_zero_str")]
    pub price: String,
    #[serde(default = "default_zero_str")]
    pub tax_rate: String,
    pub description: Option<String>,
}

// ── Tables wire ──

/// Created/updated table row (the INSERT/UPDATE RETURNING projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PosTable {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub service_area_id: Uuid,
    pub table_number: String,
    pub table_name: Option<String>,
    pub capacity: i32,
    /// AVAILABLE | OCCUPIED | RESERVED | DIRTY | OUT_OF_SERVICE.
    pub status: String,
    pub current_check_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// One row of the tables list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PosTableListView {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub service_area_id: Uuid,
    pub table_number: String,
    pub table_name: Option<String>,
    pub capacity: i32,
    /// AVAILABLE | OCCUPIED | RESERVED | DIRTY | OUT_OF_SERVICE.
    pub status: String,
    pub current_check_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Wrapper list returned by the tables endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PosTableList {
    pub items: Vec<PosTableListView>,
}

/// Body of `POST /v1/outlets/{outlet_id}/tables`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TableCreateRequest {
    pub service_area_id: Uuid,
    pub table_number: String,
    pub table_name: Option<String>,
    #[serde(default = "default_table_capacity")]
    pub capacity: i32,
}

/// Body of `PUT /v1/outlets/{outlet_id}/tables/{table_id}` (partial update).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TableUpdateRequest {
    pub table_name: Option<String>,
    pub capacity: Option<i32>,
    /// AVAILABLE | OCCUPIED | RESERVED | DIRTY | OUT_OF_SERVICE.
    pub status: Option<String>,
}

/// Deletion acknowledgement returned by table DELETE.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PosTableDeleted {
    pub deleted: bool,
    pub table_id: Uuid,
    pub outlet_id: Uuid,
}

/// One source table attached to a combine result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CombinedSourceTable {
    pub id: Uuid,
    pub table_number: String,
    pub table_name: Option<String>,
    pub capacity: i32,
    pub status: String,
}

/// Body of `POST /v1/outlets/{outlet_id}/tables/combine`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CombineTablesRequest {
    pub table_ids: Vec<Uuid>,
    pub combined_name: String,
}

/// Combine result (the new combined table + the source rows).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CombineTablesResult {
    pub combined_table_id: Uuid,
    pub outlet_id: Uuid,
    pub service_area_id: Uuid,
    pub table_number: String,
    pub table_name: Option<String>,
    pub capacity: i32,
    pub status: String,
    pub current_check_id: Option<Uuid>,
    pub source_tables: Vec<CombinedSourceTable>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Body of `POST /v1/outlets/{outlet_id}/tables/separate`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SeparateTablesRequest {
    pub table_id: Uuid,
}

/// Separate result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SeparateTablesResult {
    pub separated: bool,
    pub outlet_id: Uuid,
    pub combined_table_id: Uuid,
    pub table_number: String,
}

/// Body of `PUT /v1/outlets/{outlet_id}/tables/{table_id}/status`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TableStatusChange {
    /// AVAILABLE | OCCUPIED | RESERVED | DIRTY | OUT_OF_SERVICE.
    pub status: String,
    /// Only honored when `status` is OCCUPIED.
    pub current_check_id: Option<Uuid>,
}

/// One table on the floor plan.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FloorPlanTable {
    pub id: Uuid,
    pub table_number: String,
    pub table_name: Option<String>,
    pub capacity: i32,
    pub status: String,
    pub current_check_id: Option<Uuid>,
    /// Seat position x (null - `pos.tables` has no position columns).
    pub x: Option<serde_json::Value>,
    /// Seat position y (null - `pos.tables` has no position columns).
    pub y: Option<serde_json::Value>,
}

/// One service area of the floor plan.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FloorPlanArea {
    pub id: Uuid,
    pub area_code: String,
    pub area_name: String,
    pub is_active: bool,
    pub tables: Vec<FloorPlanTable>,
}

/// `GET /v1/outlets/{outlet_id}/tables/floor-plan`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FloorPlan {
    pub outlet_id: Uuid,
    pub service_areas: Vec<FloorPlanArea>,
}

// ── Config: service areas / categories / modifiers ──

/// Created/updated service area row.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceAreaRow {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub area_code: String,
    pub area_name: String,
    pub is_active: bool,
}

/// Service-area row in the list view (query projection without outlet id).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceAreaRowListView {
    pub id: Uuid,
    pub area_code: String,
    pub area_name: String,
    pub is_active: bool,
}

/// Wrapper list returned by the service-areas endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceAreaList {
    pub items: Vec<ServiceAreaRowListView>,
}

/// Body of `POST /v1/outlets/{outlet_id}/service-areas`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceAreaCreateRequest {
    #[serde(default)]
    pub area_code: String,
    #[serde(default)]
    pub area_name: String,
}

/// Body of `PUT /v1/outlets/{outlet_id}/service-areas/{area_id}` (partial).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceAreaUpdateRequest {
    pub area_name: Option<String>,
    pub is_active: Option<bool>,
}

/// Created/updated menu category row.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MenuCategoryRow {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub category_code: String,
    pub category_name: String,
    pub sort_order: i32,
    pub is_active: bool,
}

/// Wrapper list returned by the menu categories endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MenuCategoryList {
    pub items: Vec<MenuCategoryRow>,
}

/// Body of `POST /v1/outlets/{outlet_id}/menu/categories`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MenuCategoryCreateRequest {
    #[serde(default)]
    pub category_code: String,
    #[serde(default)]
    pub category_name: String,
    #[serde(default)]
    pub sort_order: i32,
}

/// Body of `PUT /v1/outlets/{outlet_id}/menu/categories/{category_id}` (partial).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MenuCategoryUpdateRequest {
    pub category_name: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

/// Created modifier group.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModifierGroupCreated {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub item_id: Uuid,
    pub group_name: String,
    pub is_required: bool,
    pub min_selections: i32,
    pub max_selections: i32,
}

/// One modifier group of the groups list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModifierGroupView {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub item_id: Uuid,
    pub group_name: String,
    pub is_required: bool,
    pub min_selections: i32,
    pub max_selections: i32,
}

/// Wrapper list returned by the modifier-groups endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModifierGroupList {
    pub items: Vec<ModifierGroupView>,
}

/// Body of `POST .../menu-items/{item_id}/modifier-groups`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModifierGroupCreateRequest {
    #[serde(default)]
    pub group_name: String,
    #[serde(default)]
    pub is_required: bool,
    #[serde(default)]
    pub min_selections: i32,
    #[serde(default = "default_one")]
    pub max_selections: i32,
}

/// Created modifier.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModifierCreated {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub item_id: Uuid,
    pub group_id: Uuid,
    pub modifier_name: String,
    /// NUMERIC price adjustment as text.
    pub price_adjustment: String,
    pub is_active: bool,
}

/// One modifier of the modifiers list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModifierView {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub item_id: Uuid,
    pub group_id: Uuid,
    pub modifier_name: String,
    /// NUMERIC price adjustment as text.
    pub price_adjustment: String,
    pub is_active: bool,
}

/// Wrapper list returned by the modifiers endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModifierList {
    pub items: Vec<ModifierView>,
}

/// Body of `POST .../modifier-groups/{group_id}/modifiers`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModifierCreateRequest {
    #[serde(default)]
    pub modifier_name: String,
    #[serde(default = "default_zero_str")]
    pub price_adjustment: String,
}

/// Item-availability (86'ing) update result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ItemAvailabilityResult {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub item_code: String,
    pub item_name: String,
    pub is_active: bool,
    pub reason: Option<String>,
}

/// Body of `PUT .../menu-items/{item_id}/availability`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ItemAvailabilityRequest {
    pub is_active: bool,
    pub reason: Option<String>,
}

/// One updated price line of the bulk update.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BulkPriceUpdated {
    pub item_id: Uuid,
    pub item_code: String,
    /// NUMERIC price as text.
    pub price: String,
}

/// One price line of the bulk-update body.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BulkPriceUpdateEntry {
    pub item_id: Uuid,
    /// Decimal string.
    pub price: String,
}

/// Body of `PUT /v1/outlets/{outlet_id}/menu-items/prices`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BulkPriceUpdateRequest {
    pub updates: Vec<BulkPriceUpdateEntry>,
}

/// Bulk price-update result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BulkPriceUpdateResult {
    pub updated: Vec<BulkPriceUpdated>,
}

// ── Tax groups / printer groups / payment methods ──

/// Created/updated tax group row.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TaxGroupRow {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub tax_code: String,
    pub tax_name: String,
    /// NUMERIC tax rate as text.
    pub tax_rate: String,
    pub is_active: bool,
}

/// Wrapper list returned by the tax-groups endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TaxGroupList {
    pub items: Vec<TaxGroupRow>,
}

/// Body of `POST /v1/outlets/{outlet_id}/tax-groups`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TaxGroupCreateRequest {
    #[serde(default)]
    pub tax_code: String,
    #[serde(default)]
    pub tax_name: String,
    #[serde(default = "default_zero_str")]
    pub tax_rate: String,
}

/// Body of `PUT /v1/outlets/{outlet_id}/tax-groups/{tax_id}` (partial).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TaxGroupUpdateRequest {
    pub tax_name: Option<String>,
    pub tax_rate: Option<String>,
    pub is_active: Option<bool>,
}

/// Created/updated printer group row.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PrinterGroupRow {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub printer_code: String,
    pub printer_name: String,
    pub station_code: Option<String>,
    pub is_active: bool,
}

/// Wrapper list returned by the printer-groups endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PrinterGroupList {
    pub items: Vec<PrinterGroupRow>,
}

/// Body of `POST /v1/outlets/{outlet_id}/printer-groups`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PrinterGroupCreateRequest {
    #[serde(default)]
    pub printer_code: String,
    #[serde(default)]
    pub printer_name: String,
    pub station_code: Option<String>,
}

/// Body of `PUT /v1/outlets/{outlet_id}/printer-groups/{printer_id}` (partial).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PrinterGroupUpdateRequest {
    pub printer_name: Option<String>,
    pub station_code: Option<String>,
    pub is_active: Option<bool>,
}

/// Body of `PUT /v1/outlets/{outlet_id}/payment-methods`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentMethodUpdate {
    pub methods: Vec<PaymentMethodEntry>,
}

/// Payment-method configuration for an outlet (`GET`/`PUT` response).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentMethodList {
    pub outlet_id: Uuid,
    pub methods: Vec<PaymentMethodEntry>,
}

/// One payment-method entry of the update body.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentMethodEntry {
    /// CASH | CREDIT_CARD | DEBIT_CARD | ROOM_CHARGE | GIFT_CARD | MOBILE_WALLET | COMP.
    pub r#type: String,
    pub is_enabled: bool,
}

// ── Orders (checks) wire ──

/// Created check.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CheckCreated {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub check_number: String,
    pub status: String,
}

/// Body of `POST /v1/outlets/{outlet_id}/checks`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CheckCreateRequest {
    pub table_id: Option<Uuid>,
    pub guest_name: Option<String>,
    #[serde(default = "default_guest_count")]
    pub guest_count: i32,
    pub room_number: Option<String>,
}

/// One order line of a check detail / added order.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OrderLine {
    pub id: Uuid,
    pub menu_item_id: Uuid,
    pub quantity: i32,
    /// NUMERIC unit price as text.
    pub unit_price: String,
    /// NUMERIC line total as text.
    pub line_total: String,
    pub status: String,
}

/// Check detail (`GET /v1/outlets/{outlet_id}/checks/{check_id}`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CheckDetail {
    pub id: Uuid,
    pub outlet_id: Uuid,
    pub check_number: String,
    pub status: String,
    pub guest_name: Option<String>,
    pub guest_count: i32,
    /// NUMERIC amounts as text.
    pub subtotal: String,
    pub tax_total: String,
    pub total: String,
    pub room_number: Option<String>,
    pub opened_at: chrono::DateTime<chrono::Utc>,
    pub orders: Vec<OrderLine>,
}

/// One check row of the search list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CheckSummary {
    pub id: Uuid,
    pub check_number: String,
    pub status: String,
    pub guest_name: Option<String>,
    pub guest_count: i32,
    pub subtotal: String,
    pub total: String,
    pub table_id: Option<Uuid>,
    pub opened_at: chrono::DateTime<chrono::Utc>,
}

/// Wrapper list returned by the checks search endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CheckList {
    pub items: Vec<CheckSummary>,
}

/// Added order line (the INSERT RETURNING projection).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AddedOrderLine {
    pub id: Uuid,
    pub check_id: Uuid,
    pub menu_item_id: Uuid,
    pub item_name: String,
    pub quantity: i32,
    /// NUMERIC line total as text.
    pub line_total: String,
    pub status: String,
}

/// Body of `POST .../checks/{check_id}/orders`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AddOrderItemRequest {
    pub menu_item_id: Uuid,
    #[serde(default = "default_one")]
    pub quantity: i32,
    /// Free-form modifiers bag (stored JSONB as provided).
    #[serde(default)]
    pub modifiers: serde_json::Value,
    pub special_instructions: Option<String>,
}

/// Check-transfer result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransferCheckResult {
    pub check_id: Uuid,
    pub new_table_id: Uuid,
    pub transferred: bool,
}

/// Body of `POST .../checks/{check_id}/transfer`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransferCheckRequest {
    pub new_table_id: Uuid,
}

/// One split check produced by a split operation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SplitPart {
    pub check_id: Uuid,
    pub check_number: String,
    /// NUMERIC split amount as text.
    pub amount: String,
}

/// Body of `POST .../checks/{check_id}/split`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SplitCheckRequest {
    /// "equal" (only the equal split is implemented).
    #[serde(default = "default_equal_split")]
    pub split_type: String,
    #[serde(default = "default_two_splits")]
    pub num_splits: i32,
}

/// Equal-split result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SplitCheckResult {
    pub original_check_id: Uuid,
    pub split_type: String,
    pub splits: Vec<SplitPart>,
}

/// Check hold/recall acknowledgement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CheckStateChange {
    pub check_id: Uuid,
    /// HELD | OPEN after the transition.
    pub status: String,
}

/// Seat-assignment acknowledgement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SeatAssigned {
    pub order_id: Uuid,
    pub seat_number: i32,
    pub assigned: bool,
}

/// Body of `POST .../orders/{order_id}/seat`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AssignSeatRequest {
    #[serde(default = "default_one")]
    pub seat_number: i32,
}

/// Order-item void result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OrderItemVoidResult {
    pub order_id: Uuid,
    pub voided: bool,
    pub reason: String,
    /// NUMERIC refunded amount as text.
    pub amount_refunded: String,
}

/// Body of `POST /v1/outlets/{outlet_id}/orders/void`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VoidOrderItemRequest {
    pub order_id: Uuid,
    #[serde(default = "default_voided_reason")]
    pub reason: String,
    #[serde(default)]
    pub manager_approved: bool,
}

/// Check-void result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CheckVoidResult {
    pub check_id: Uuid,
    pub voided: bool,
    pub reason: String,
}

/// Body of `POST .../checks/{check_id}/void`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VoidCheckRequest {
    pub reason: String,
    #[serde(default)]
    pub manager_approved: bool,
}

// ── Payments wire ──

/// Posted payment row.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostedPayment {
    pub id: Uuid,
    pub check_id: Uuid,
    pub outlet_id: Uuid,
    pub payment_type: String,
    /// NUMERIC amounts as text.
    pub amount: String,
    pub tip_amount: String,
    pub reference: Option<String>,
}

/// Body of `POST /v1/outlets/{outlet_id}/checks/{check_id}/payments`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostPaymentRequest {
    /// CASH | CREDIT_CARD | DEBIT_CARD | ROOM_CHARGE | GIFT_CARD | MOBILE_WALLET | COMP.
    #[serde(default = "default_cash_ptype")]
    pub payment_type: String,
    /// Decimal string.
    #[serde(default = "default_zero_str")]
    pub amount: String,
    /// Decimal string.
    #[serde(default = "default_zero_str")]
    pub tip_amount: String,
    pub reference: Option<String>,
    pub room_number: Option<String>,
}

/// Card-tokenized payment-processor result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CardPaymentProcessed {
    pub payment_id: Uuid,
    pub check_id: Uuid,
    pub amount: String,
    pub payment_type: String,
    pub card_token: String,
    pub status: String,
}

/// Body of `POST .../payments-process` (bill8 card token).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CardPaymentRequest {
    pub card_token: String,
    /// Decimal string.
    #[serde(default = "default_zero_str")]
    pub amount: String,
    /// Decimal string.
    #[serde(default = "default_zero_str")]
    pub tip_amount: String,
}

/// One leg of a split payment.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SplitPaymentLeg {
    pub payment_id: Uuid,
    pub payment_type: String,
    pub amount: String,
}

/// One payment method/amount of the split-payment body.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SplitPaymentLegInput {
    /// CASH | CREDIT_CARD | DEBIT_CARD | ROOM_CHARGE | GIFT_CARD | MOBILE_WALLET | COMP.
    #[serde(default = "default_cash_ptype")]
    pub payment_type: String,
    /// Decimal string.
    #[serde(default = "default_zero_str")]
    pub amount: String,
    /// Decimal string.
    #[serde(default = "default_zero_str")]
    pub tip_amount: String,
    pub reference: Option<String>,
}

/// Body of `POST .../split-payment`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SplitPaymentRequest {
    pub payments: Vec<SplitPaymentLegInput>,
}

/// Split-payment result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SplitPaymentResult {
    pub check_id: Uuid,
    pub payments: Vec<SplitPaymentLeg>,
}

/// Discount-application result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DiscountApplied {
    pub payment_id: Uuid,
    pub check_id: Uuid,
    pub discount_type: String,
    pub discount_amount: String,
    pub new_total: String,
}

/// Body of `POST .../discount`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApplyDiscountRequest {
    /// PERCENTAGE | DOLLAR | EMPLOYEE_MEAL | MANAGER_COMP.
    pub discount_type: String,
    /// Decimal string (percentage value for PERCENTAGE).
    #[serde(default = "default_zero_str")]
    pub value: String,
    #[serde(default)]
    pub manager_approved: bool,
}

/// Room-charge (folio posting) result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoomChargeResult {
    pub payment_id: Uuid,
    pub check_id: Uuid,
    pub room_number: String,
    pub amount: String,
    pub payment_type: String,
    pub status: String,
}

/// Body of `POST /v1/payments/room-charge`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoomChargeRequest {
    pub check_id: Uuid,
    pub room_number: String,
    pub reservation_id: Option<Uuid>,
}

/// Guest lookup by room (PMS in-house reservation match).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuestRoomLookup {
    pub reservation_id: serde_json::Value,
    pub guest_name: serde_json::Value,
    pub room_number: serde_json::Value,
    pub guest_profile_id: serde_json::Value,
}

/// Body of `POST .../redeem-package`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RedeemPackageRequest {
    pub package_code: String,
    pub reservation_id: Uuid,
}

/// Package redemption result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PackageRedeemed {
    pub payment_id: Uuid,
    pub check_id: Uuid,
    pub package_code: String,
    pub redeemed: bool,
    pub amount_charged: String,
}

// ── Reports wire ──

/// `GET /v1/outlets/{outlet_id}/reports/sales`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DailySalesSummary {
    pub outlet_id: Uuid,
    pub date: String,
    pub total_checks: i64,
    pub paid_checks: i64,
    pub voided_checks: i64,
    /// NUMERIC totals as text.
    pub gross_sales: String,
    pub net_sales: String,
    pub tax_collected: String,
    pub total_tips: String,
}

/// One item line of the item-sales report.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ItemSalesRow {
    pub menu_item_id: Uuid,
    pub item_name: String,
    pub item_code: String,
    pub total_quantity: i64,
    pub total_revenue: String,
    pub order_count: i64,
}

/// `GET /v1/outlets/{outlet_id}/reports/items`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ItemSalesReport {
    pub outlet_id: Uuid,
    pub start_date: String,
    pub end_date: String,
    pub items: Vec<ItemSalesRow>,
}

/// One labor line of the labor report.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LaborRow {
    pub role: String,
    pub headcount: i32,
    pub hours: f64,
    pub labor_cost: String,
}

/// `GET /v1/outlets/{outlet_id}/reports/labor`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LaborReport {
    pub outlet_id: Uuid,
    pub date: String,
    pub labor: Vec<LaborRow>,
    pub total_labor_cost: String,
    pub note: String,
}

/// One payment-type line of the end-of-day report.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EodPaymentLine {
    pub payment_type: String,
    pub count: i64,
    pub total_amount: String,
    pub total_tips: String,
}

/// `GET /v1/outlets/{outlet_id}/reports/end-of-day`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EndOfDayReport {
    pub outlet_id: Uuid,
    pub date: String,
    pub total_checks: i64,
    pub gross_sales: String,
    pub tax_collected: String,
    pub payment_breakdown: Vec<EodPaymentLine>,
}

/// Report-export acknowledgement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExportReportQueued {
    pub export_id: Uuid,
    pub outlet_id: Uuid,
    pub report_type: String,
    pub format: String,
    pub date: String,
    pub status: String,
    pub download_url: String,
    pub message: String,
}

/// Query of the export endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExportReportQuery {
    #[serde(default = "default_csv_format")]
    pub format: String,
    #[serde(default = "default_today")]
    pub date: String,
}

// ── Permissions / offline / kitchen wire ──

/// One role line of the permission matrix.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RolePermissionRow {
    pub role: String,
    pub permissions: Vec<String>,
    pub can_void: bool,
    pub can_comp: bool,
    pub can_discount: bool,
    pub void_threshold: String,
    pub comp_threshold: String,
    pub discount_threshold: String,
}

/// `GET /v1/permissions` - the H8POS role permission matrix.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PosPermissions {
    pub roles: Vec<RolePermissionRow>,
}

/// Manager-approval result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ManagerApproval {
    pub approval_id: Uuid,
    pub action_type: String,
    pub amount: String,
    pub reference_id: String,
    pub approved: bool,
    pub approved_by: String,
}

/// Body of `POST /v1/manager-approve`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ManagerApproveRequest {
    pub action_type: String,
    /// Decimal string.
    #[serde(default = "default_zero_str")]
    pub amount: String,
    #[serde(default)]
    pub reference_id: String,
    #[serde(default)]
    pub manager_id: String,
    /// Verified against Sentin8:Auth (required, non-empty).
    #[serde(default)]
    pub pin: String,
}

/// Offline status of the terminal.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfflineStatus {
    pub is_online: bool,
    pub queued_operations: i32,
    pub last_sync_at: chrono::DateTime<chrono::Utc>,
    pub sync_status: String,
}

/// One replayed offline operation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SyncedOperation {
    pub r#type: String,
    /// SYNCED | SKIPPED.
    pub status: String,
}

/// One queued offline operation of the sync body.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfflineOperationInput {
    /// check_create | order_add | payment_post (others are skipped).
    pub r#type: String,
    /// Operation payload (opaque to the sync surface).
    #[serde(default)]
    pub data: serde_json::Value,
}

/// Body of `POST /v1/offline-sync`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfflineSyncRequest {
    pub operations: Vec<OfflineOperationInput>,
}

/// Offline-sync result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OfflineSyncResult {
    pub synced: usize,
    pub operations: Vec<SyncedOperation>,
    pub synced_at: chrono::DateTime<chrono::Utc>,
}

/// Kitchen-routing acknowledgement.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct KitchenRouted {
    pub routing_id: Uuid,
    pub order_id: Uuid,
    pub check_id: Uuid,
    pub item_name: String,
    pub quantity: i32,
    pub routed_to: String,
    pub status: String,
}

// ── Guest ordering wire ──

/// One menu row of the guest menu.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuestMenuItem {
    /// Menu-item id (serialized as text).
    pub id: String,
    pub name: String,
    /// Empty string when the item has no description.
    pub description: String,
    /// NUMERIC price as text.
    pub price: String,
    /// Empty string when uncategorized.
    pub category: String,
}

/// `GET /v1/guest/{property_id}/menu`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuestMenu {
    pub outlet_id: Uuid,
    pub items: Vec<GuestMenuItem>,
}

/// One ordered line of a guest order.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuestOrderLine {
    pub id: Uuid,
    pub menu_item_id: Uuid,
    pub name: String,
    pub quantity: i32,
    pub unit_price: String,
}

/// One line of the guest-order body.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuestOrderLineInput {
    pub menu_item_id: Uuid,
    #[serde(default = "default_one")]
    pub quantity: i32,
}

/// Body of `POST /v1/guest/{property_id}/orders`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuestOrderCreateRequest {
    pub outlet_id: Uuid,
    pub reservation_id: Uuid,
    pub room_number: Option<String>,
    pub items: Vec<GuestOrderLineInput>,
}

/// Created guest (room-service) order.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuestOrderCreated {
    pub check_id: Uuid,
    pub check_number: String,
    pub status: String,
    pub items: Vec<GuestOrderLine>,
}

/// One line of the guest order read-back.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuestOrderLineView {
    /// Order-line id (serialized as text).
    pub id: String,
    pub name: String,
    pub quantity: i32,
    pub unit_price: String,
    pub line_total: String,
    pub status: String,
}

/// `GET /v1/guest/{property_id}/orders/{check_id}` - order read-back.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuestOrderStatus {
    /// Check id (serialized as text).
    pub check_id: String,
    pub check_number: String,
    pub status: String,
    pub subtotal: String,
    pub tax_total: String,
    pub total: String,
    pub room_number: Option<String>,
    /// Reservation id (serialized as text) or null.
    pub reservation_id: Option<String>,
    pub items: Vec<GuestOrderLineView>,
}

fn default_outlet_type() -> String {
    "RESTAURANT".into()
}

fn default_usd_code() -> String {
    "USD".into()
}

fn default_table_capacity() -> i32 {
    4
}

fn default_voided_reason() -> String {
    "voided".into()
}

fn default_guest_count() -> i32 {
    1
}

fn default_equal_split() -> String {
    "equal".into()
}

fn default_two_splits() -> i32 {
    2
}

fn default_csv_format() -> String {
    "csv".into()
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests - contract round-trips (H8T-6)
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

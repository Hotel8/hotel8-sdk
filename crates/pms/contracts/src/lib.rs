// SPDX-FileCopyrightText: Copyright Logically Magic Inc. <hello@logicallymagic.com>
// SPDX-FileCopyrightText: Copyright Hamid Jahanzad <Hamid@Jahanzad.com>
// SPDX-FileCopyrightText: Copyright Contributors to the Hotel8:SDK
// SPDX-License-Identifier: Apache-2.0

//! # pms-contracts
//!
//! Rust types for the PMS Property Management System HTTP API.
//!
//! Covers all PMS modules: Reservations, Front Desk, Housekeeping, Cashiering,
//! Rate Management, Availability, Inventory, Blocks, Events, Profiles/CRM,
//! Back Office, and List of Values (LOV).
//!
//! See `hotel8/design/sentin8-hotel8-pms-technical-design.md` for the full design.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════
// Property
// ═══════════════════════════════════════════════════════════════════════════

/// A hotel property (maps to a Sentin8 environment).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Property {
    pub id: Uuid,
    pub env_id: Uuid,
    pub property_code: String,
    pub property_name: String,
    pub timezone: String,
    pub currency_code: String,
    pub languages: Vec<String>,
    pub address: Option<Address>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PropertyCreate {
    pub property_code: String,
    pub property_name: String,
    pub timezone: Option<String>,
    pub currency_code: Option<String>,
    pub languages: Option<Vec<String>>,
    pub address: Option<Address>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Address {
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Room & Room Type
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoomType {
    pub id: Uuid,
    pub property_id: Uuid,
    pub room_type_code: String,
    pub room_type_name: String,
    pub description: Option<String>,
    pub max_occupancy: i32,
    pub bed_type: Option<String>,
    pub is_smoking: bool,
    pub features: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoomTypeCreate {
    pub room_type_code: String,
    pub room_type_name: String,
    pub description: Option<String>,
    pub max_occupancy: i32,
    pub bed_type: Option<String>,
    pub is_smoking: bool,
    pub features: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FrontOfficeStatus {
    Vacant,
    Occupied,
    DueOut,
    Departed,
    Stayover,
}

impl std::fmt::Display for FrontOfficeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrontOfficeStatus::Vacant => write!(f, "VACANT"),
            FrontOfficeStatus::Occupied => write!(f, "OCCUPIED"),
            FrontOfficeStatus::DueOut => write!(f, "DUE_OUT"),
            FrontOfficeStatus::Departed => write!(f, "DEPARTED"),
            FrontOfficeStatus::Stayover => write!(f, "STAYOVER"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HousekeepingStatus {
    Clean,
    Dirty,
    Inspected,
    Pickup,
    OutOfOrder,
    OutOfService,
}

impl std::fmt::Display for HousekeepingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HousekeepingStatus::Clean => write!(f, "CLEAN"),
            HousekeepingStatus::Dirty => write!(f, "DIRTY"),
            HousekeepingStatus::Inspected => write!(f, "INSPECTED"),
            HousekeepingStatus::Pickup => write!(f, "PICKUP"),
            HousekeepingStatus::OutOfOrder => write!(f, "OUT_OF_ORDER"),
            HousekeepingStatus::OutOfService => write!(f, "OUT_OF_SERVICE"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Room {
    pub id: Uuid,
    pub property_id: Uuid,
    pub room_type_id: Uuid,
    pub room_number: String,
    pub floor: Option<String>,
    pub building: Option<String>,
    pub front_office_status: FrontOfficeStatus,
    pub housekeeping_status: HousekeepingStatus,
    pub is_smoking: bool,
    pub max_occupancy: i32,
    pub bed_type: Option<String>,
    pub features: Vec<String>,
    pub current_reservation_id: Option<Uuid>,
    pub next_reservation_id: Option<Uuid>,
    pub last_cleaned_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_inspected_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoomCreate {
    pub room_type_id: Uuid,
    pub room_number: String,
    pub floor: Option<String>,
    pub building: Option<String>,
    pub is_smoking: Option<bool>,
    pub max_occupancy: Option<i32>,
    pub bed_type: Option<String>,
    pub features: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoomStatusUpdate {
    pub front_office_status: Option<FrontOfficeStatus>,
    pub housekeeping_status: Option<HousekeepingStatus>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Inventory
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub property_id: Uuid,
    pub room_type_id: Uuid,
    pub date: chrono::NaiveDate,
    pub physical_rooms: i32,
    pub sell_limit: i32,
    pub sold_rooms: i32,
    pub available_rooms: i32,
    pub out_of_order: i32,
    pub out_of_service: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellLimitUpdate {
    pub room_type_id: Uuid,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub sell_limit: i32,
    pub reason: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Rate Management
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RatePlan {
    pub id: Uuid,
    pub property_id: Uuid,
    pub rate_plan_code: String,
    pub rate_plan_name: String,
    pub rate_category: Option<String>,
    pub rate_class: Option<String>,
    pub description: Option<String>,
    pub currency_code: String,
    pub market_code: Option<String>,
    pub source_code: Option<String>,
    pub is_active: bool,
    pub is_negotiated: bool,
    pub min_los: Option<i32>,
    pub max_los: Option<i32>,
    pub min_advance_booking: Option<i32>,
    pub max_advance_booking: Option<i32>,
    pub cancellation_policy: Option<String>,
    pub deposit_policy: Option<String>,
    pub guarantee_policy: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RatePlanCreate {
    pub rate_plan_code: String,
    pub rate_plan_name: String,
    pub rate_category: Option<String>,
    pub rate_class: Option<String>,
    pub description: Option<String>,
    pub currency_code: String,
    pub market_code: Option<String>,
    pub source_code: Option<String>,
    pub is_active: Option<bool>,
    pub is_negotiated: Option<bool>,
    pub min_los: Option<i32>,
    pub max_los: Option<i32>,
    pub min_advance_booking: Option<i32>,
    pub max_advance_booking: Option<i32>,
    pub cancellation_policy: Option<String>,
    pub deposit_policy: Option<String>,
    pub guarantee_policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateSchedule {
    pub id: Uuid,
    pub rate_plan_id: Uuid,
    pub room_type_id: Uuid,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub days_of_week: Vec<String>,
    pub rate_amount: rust_decimal::Decimal,
    pub rate_amount_single: Option<rust_decimal::Decimal>,
    pub rate_amount_double: Option<rust_decimal::Decimal>,
    pub rate_amount_extra_adult: Option<rust_decimal::Decimal>,
    pub rate_amount_extra_child: Option<rust_decimal::Decimal>,
    pub package_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateScheduleCreate {
    pub room_type_id: Uuid,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub days_of_week: Option<Vec<String>>,
    pub rate_amount: rust_decimal::Decimal,
    pub rate_amount_single: Option<rust_decimal::Decimal>,
    pub rate_amount_double: Option<rust_decimal::Decimal>,
    pub rate_amount_extra_adult: Option<rust_decimal::Decimal>,
    pub rate_amount_extra_child: Option<rust_decimal::Decimal>,
    pub package_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub package_code: String,
    pub package_name: String,
    pub quantity: i32,
    pub price: rust_decimal::Decimal,
    pub per_stay: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageCreate {
    pub package_code: String,
    pub package_name: String,
    pub quantity: Option<i32>,
    pub price: rust_decimal::Decimal,
    pub per_stay: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Availability
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityRequest {
    pub arrival_date: chrono::NaiveDate,
    pub departure_date: chrono::NaiveDate,
    pub number_of_adults: i32,
    pub number_of_children: Option<i32>,
    pub number_of_rooms: Option<i32>,
    pub room_type: Option<String>,
    pub rate_plan_code: Option<String>,
    pub promo_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityResponse {
    pub property_code: String,
    pub env_id: Uuid,
    pub arrival_date: chrono::NaiveDate,
    pub departure_date: chrono::NaiveDate,
    pub number_of_nights: i32,
    pub room_types: Vec<AvailableRoomType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableRoomType {
    pub room_type: String,
    pub room_type_description: String,
    pub available_rooms: i32,
    pub total_rooms: i32,
    pub rate_plans: Vec<AvailableRatePlan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableRatePlan {
    pub rate_plan_code: String,
    pub rate_plan_name: String,
    pub total_amount: rust_decimal::Decimal,
    pub average_nightly_rate: rust_decimal::Decimal,
    pub currency_code: String,
    pub is_available: bool,
    pub restrictions: Option<RateRestrictions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateRestrictions {
    pub closed_to_arrival: bool,
    pub closed_to_departure: bool,
    pub min_los: i32,
    pub max_los: i32,
}

// ═══════════════════════════════════════════════════════════════════════════
// Guest Profile (CRM)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProfileType {
    Guest,
    Company,
    TravelAgent,
    Source,
    Group,
    Contact,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VipStatus {
    None,
    Vip,
    Vvip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestProfile {
    pub id: Uuid,
    pub property_id: Uuid,
    pub profile_type: ProfileType,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub preferred_name: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub nationality: Option<String>,
    pub primary_language: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub mobile_phone: Option<String>,
    pub address: Option<Address>,
    pub passport_number: Option<String>,
    pub passport_country: Option<String>,
    pub passport_expiry_date: Option<chrono::NaiveDate>,
    pub vip_status: VipStatus,
    pub communication_consent: bool,
    pub preferences: serde_json::Value,
    pub loyalty_programs: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestProfileCreate {
    pub profile_type: ProfileType,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub preferred_name: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub nationality: Option<String>,
    pub primary_language: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub mobile_phone: Option<String>,
    pub address: Option<Address>,
    pub passport_number: Option<String>,
    pub passport_country: Option<String>,
    pub passport_expiry_date: Option<chrono::NaiveDate>,
    pub vip_status: Option<VipStatus>,
    pub communication_consent: Option<bool>,
    pub preferences: Option<serde_json::Value>,
    pub loyalty_programs: Option<serde_json::Value>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Reservations
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReservationStatus {
    Reserved,
    DueIn,
    CheckedIn,
    CheckedOut,
    Cancelled,
    NoShow,
    Waitlist,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    pub id: Uuid,
    pub property_id: Uuid,
    pub confirmation_number: String,
    pub status: ReservationStatus,
    pub arrival_date: chrono::NaiveDate,
    pub departure_date: chrono::NaiveDate,
    pub number_of_nights: i32,
    pub number_of_rooms: i32,
    pub number_of_adults: i32,
    pub number_of_children: i32,
    pub room_type_id: Uuid,
    pub room_id: Option<Uuid>,
    pub rate_plan_id: Uuid,
    pub rate_amount: rust_decimal::Decimal,
    pub currency_code: String,
    pub guest_profile_id: Option<Uuid>,
    pub company_profile_id: Option<Uuid>,
    pub travel_agent_profile_id: Option<Uuid>,
    pub source_code: Option<String>,
    pub market_code: Option<String>,
    pub payment_method: Option<String>,
    pub guarantee_code: Option<String>,
    pub credit_card_token: Option<String>,
    pub packages: serde_json::Value,
    pub comments: serde_json::Value,
    pub special_requests: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationCreate {
    pub arrival_date: chrono::NaiveDate,
    pub departure_date: chrono::NaiveDate,
    pub number_of_rooms: Option<i32>,
    pub number_of_adults: i32,
    pub number_of_children: Option<i32>,
    pub room_type_id: Uuid,
    pub rate_plan_id: Uuid,
    pub guest_profile_id: Option<Uuid>,
    pub company_profile_id: Option<Uuid>,
    pub travel_agent_profile_id: Option<Uuid>,
    pub source_code: Option<String>,
    pub market_code: Option<String>,
    pub payment_method: Option<String>,
    pub guarantee_code: Option<String>,
    pub credit_card_token: Option<String>,
    pub packages: Option<serde_json::Value>,
    pub comments: Option<serde_json::Value>,
    pub special_requests: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationSearch {
    pub last_name: Option<String>,
    pub confirmation_number: Option<String>,
    pub arrival_date: Option<chrono::NaiveDate>,
    pub departure_date: Option<chrono::NaiveDate>,
    pub status: Option<ReservationStatus>,
    pub room_number: Option<String>,
    pub company_name: Option<String>,
    pub block_code: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Folio (Cashiering)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FolioStatus {
    Open,
    Active,
    Settled,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folio {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub folio_number: Option<String>,
    pub status: FolioStatus,
    pub balance: rust_decimal::Decimal,
    pub currency_code: String,
    pub charges: Vec<FolioCharge>,
    pub payments: Vec<FolioPayment>,
    pub taxes: Vec<FolioTax>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolioCharge {
    pub id: Uuid,
    pub transaction_code: String,
    pub description: String,
    pub amount: rust_decimal::Decimal,
    pub quantity: i32,
    pub posted_at: chrono::DateTime<chrono::Utc>,
    pub posted_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolioPayment {
    pub id: Uuid,
    pub payment_method: String,
    pub amount: rust_decimal::Decimal,
    pub reference_number: Option<String>,
    pub posted_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolioTax {
    pub tax_code: String,
    pub tax_description: String,
    pub amount: rust_decimal::Decimal,
    pub taxable_amount: rust_decimal::Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostChargeRequest {
    pub transaction_code: String,
    pub amount: rust_decimal::Decimal,
    pub quantity: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPaymentRequest {
    pub payment_method: String,
    pub amount: rust_decimal::Decimal,
    pub credit_card_token: Option<String>,
    pub reference_number: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Blocks & Events
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockStatus {
    Enquiry,
    Tentative,
    Definite,
    Actual,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: Uuid,
    pub property_id: Uuid,
    pub block_code: String,
    pub block_name: String,
    pub status: BlockStatus,
    pub block_type: Option<String>,
    pub arrival_date: chrono::NaiveDate,
    pub departure_date: chrono::NaiveDate,
    pub number_of_rooms: i32,
    pub pickup_rooms: i32,
    pub rate_plan_id: Option<Uuid>,
    pub company_profile_id: Option<Uuid>,
    pub travel_agent_profile_id: Option<Uuid>,
    pub contact_profile_id: Option<Uuid>,
    pub posting_master_reservation_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockCreate {
    pub block_code: String,
    pub block_name: String,
    pub status: Option<BlockStatus>,
    pub block_type: Option<String>,
    pub arrival_date: chrono::NaiveDate,
    pub departure_date: chrono::NaiveDate,
    pub number_of_rooms: i32,
    pub rate_plan_id: Option<Uuid>,
    pub company_profile_id: Option<Uuid>,
    pub travel_agent_profile_id: Option<Uuid>,
    pub contact_profile_id: Option<Uuid>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventStatus {
    Enquiry,
    Tentative,
    Definite,
    Actual,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub block_id: Uuid,
    pub event_name: String,
    pub event_type: Option<String>,
    pub status: EventStatus,
    pub start_date_time: chrono::DateTime<chrono::Utc>,
    pub end_date_time: chrono::DateTime<chrono::Utc>,
    pub function_space: Option<String>,
    pub setup_style: Option<String>,
    pub expected_attendees: i32,
    pub guaranteed_attendees: Option<i32>,
    pub rental_code: Option<String>,
    pub notes: Option<String>,
    pub resources: Vec<EventResource>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCreate {
    pub block_id: Uuid,
    pub event_name: String,
    pub event_type: Option<String>,
    pub status: Option<EventStatus>,
    pub start_date_time: chrono::DateTime<chrono::Utc>,
    pub end_date_time: chrono::DateTime<chrono::Utc>,
    pub function_space: Option<String>,
    pub setup_style: Option<String>,
    pub expected_attendees: i32,
    pub guaranteed_attendees: Option<i32>,
    pub rental_code: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResource {
    pub resource_code: String,
    pub resource_name: String,
    pub quantity: i32,
    pub price: rust_decimal::Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// List of Values (LOV)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LovEntry {
    pub code: String,
    pub description: String,
    pub display_sequence: i32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LovCreate {
    pub code: String,
    pub description: String,
    pub display_sequence: Option<i32>,
    pub is_active: Option<bool>,
}

/// Liveness/build contract (`GET /v1/health`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub build: String,
    /// Pool connectivity probe ("connected" | "disconnected").
    pub database: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// Availability search (the handler wire shapes)
//
// The legacy `AvailabilityResponse` family above is the booking-engine
// contract; these structs pin the SHOPPING-side search shapes the PMS
// availability handlers actually return (property-scoped, org-scoped,
// corporate rate-mix, packages, guarantee policies). ToSchema on every
// member so the `docs/openapi.json` response schemas are real (OAPEC §2
// rule 10 - the `serde_json::Value` body placeholder is retired).
// ═══════════════════════════════════════════════════════════════════════════

/// One quoted rate plan inside a room-type availability result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RatePlanQuote {
    pub rate_plan_id: Uuid,
    pub rate_plan_code: String,
    pub rate_plan_name: String,
    pub currency_code: String,
    pub rate_amount: String,
    /// `true` only on the corporate rate-mix: a negotiated plan linked to
    /// the requesting company. Absent (`None`, skipped) on public quotes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negotiated: Option<bool>,
}

/// One room type with its sellable inventory and the quoted rate plans whose
/// restrictions admit the stay.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoomTypeAvailability {
    pub room_type_id: Uuid,
    pub room_type_code: String,
    pub room_type_name: String,
    pub max_occupancy: i32,
    pub bed_type: Option<String>,
    pub available_rooms: i32,
    pub rate_plans: Vec<RatePlanQuote>,
}

/// Property-scoped availability search response
/// (`GET /v1/properties/{property_id}/availability`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PropertyAvailability {
    pub property_id: Uuid,
    pub arrival_date: String,
    pub departure_date: String,
    pub number_of_nights: i64,
    pub room_types: Vec<RoomTypeAvailability>,
}

/// Corporate rate-mix availability: the property-scoped response plus the
/// company profile the negotiated rate plans belong to
/// (`POST /v1/properties/{property_id}/corporate-availability`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CorporateAvailability {
    pub property_id: Uuid,
    pub company_profile_id: Uuid,
    pub arrival_date: String,
    pub departure_date: String,
    pub number_of_nights: i64,
    pub room_types: Vec<RoomTypeAvailability>,
}

/// One room type in the org-wide availability roll-up (no rate quoting).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OrgRoomTypeAvailability {
    pub room_type_code: String,
    pub room_type_name: String,
    pub available_rooms: i32,
}

/// One property in the org-wide availability roll-up.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OrgPropertyAvailability {
    pub property_id: Uuid,
    pub property_code: String,
    pub property_name: String,
    pub city: Option<String>,
    pub room_types: Vec<OrgRoomTypeAvailability>,
}

/// Org-wide availability search across every property of the environment
/// (`GET /v1/availability`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OrgAvailability {
    pub env_id: Uuid,
    pub arrival_date: String,
    pub departure_date: String,
    pub properties: Vec<OrgPropertyAvailability>,
}

/// A sellable package summary (`GET /v1/properties/{property_id}/packages`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PackageSummary {
    pub package_code: String,
    pub package_name: String,
}

/// Wrapper list returned by the packages endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PackageList {
    pub items: Vec<PackageSummary>,
}

/// A guarantee policy option (`GET /v1/properties/{property_id}/guarantee-policies`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuaranteePolicy {
    pub code: String,
    pub description: String,
    pub is_active: bool,
}

/// Wrapper list returned by the guarantee-policies endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuaranteePolicyList {
    pub items: Vec<GuaranteePolicy>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Guest-app surfaces (eGuest / eKiosk wire shapes)
//
// Digital-key lifecycle, folio-by-room settlement view, boarding pass, stay
// feedback, saved payment methods, and reservation↔guest linking. ToSchema
// on every member so the `docs/openapi.json` response schemas are real
// (OAPEC §2 rule 10 - the `serde_json::Value` body placeholder is retired).
// ═══════════════════════════════════════════════════════════════════════════

/// One posted folio charge.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FolioChargeView {
    pub id: Uuid,
    pub charge_code: String,
    pub description: String,
    /// Numeric amount serialized as text (NUMERIC via `::text`).
    pub amount: String,
    pub quantity: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// One posted folio payment.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FolioPaymentView {
    pub id: Uuid,
    pub payment_code: String,
    pub amount: String,
    pub reference: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// One digital key.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DigitalKeyView {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub room_number: Option<String>,
    pub key_type: String,
    pub status: String,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub activated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub revoked_at: Option<chrono::DateTime<chrono::Utc>>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub issued_count: i32,
}

/// Wrapper list returned by the digital-keys endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DigitalKeyList {
    pub items: Vec<DigitalKeyView>,
}

/// Body of `POST /v1/properties/{property_id}/digital-keys` (key issuance).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IssueDigitalKeyRequest {
    pub reservation_id: Uuid,
    #[serde(default = "default_physical_key")]
    pub key_type: String,
    /// Keys to have ready for the stay (clamped server-side to 1..=8).
    #[serde(default = "default_one")]
    pub count: i32,
}

/// Minted/reused key reference.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IssuedKeyRef {
    pub id: Uuid,
}

/// Wrapper list of minted/reused key references.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct IssuedKeyList {
    pub items: Vec<IssuedKeyRef>,
}

/// Key state transition result (UPDATE ... RETURNING of the touched row).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DigitalKeyTransitionResult {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub room_number: Option<String>,
    pub status: String,
}

/// Folio settlement view for the in-house stay in a room.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RoomFolioView {
    pub id: Option<Uuid>,
    pub reservation_id: Uuid,
    pub room_number: String,
    pub status: String,
    pub charges: Vec<FolioChargeView>,
    pub payments: Vec<FolioPaymentView>,
    /// charges − payments, formatted with 2 decimals.
    pub balance: String,
}

/// Boarding pass - the check-in artifact the kiosk completion step renders.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BoardingPass {
    pub reservation_id: Uuid,
    pub confirmation_number: String,
    pub guest_name: String,
    pub property_name: Option<String>,
    pub room_number: Option<String>,
    pub arrival_date: chrono::NaiveDate,
    pub departure_date: chrono::NaiveDate,
    pub num_adults: i32,
    pub key_count: i64,
    pub issued_at: chrono::DateTime<chrono::Utc>,
}

/// Body of `POST /v1/properties/{property_id}/feedback`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SubmitFeedbackRequest {
    pub reservation_id: Uuid,
    /// 1..=5.
    pub rating: i32,
    pub comment: Option<String>,
}

/// A submitted stay feedback row.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FeedbackView {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Feedback lookup response (all-`None` except the id when not yet rated).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FeedbackLookup {
    pub reservation_id: Uuid,
    pub rating: Option<i32>,
    pub comment: Option<String>,
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A saved bill8 payment-method token reference (never PAN/secrets).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuestPaymentMethod {
    pub id: Uuid,
    pub label: String,
    pub payment_method_id: String,
    pub is_default: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Wrapper list of the guest's saved payment methods.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuestPaymentMethodList {
    pub items: Vec<GuestPaymentMethod>,
}

/// Body of `POST /v1/properties/{property_id}/payment-methods`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateGuestPaymentMethodRequest {
    /// bill8 token reference (required, non-empty).
    pub payment_method_id: String,
    /// Required, non-empty.
    pub label: String,
    #[serde(default)]
    pub is_default: bool,
}

/// Body of `POST /v1/properties/{property_id}/reservations/{res_id}/link-guest`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LinkGuestRequest {
    /// Optional explicit target profile; resolved from the session when absent.
    pub target_profile_id: Option<Uuid>,
}

/// Guest-linking result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LinkGuestResult {
    pub linked: bool,
    pub already_linked: bool,
    pub reservation_id: Uuid,
    pub guest_profile_id: Uuid,
    pub booker_profile_id: Option<Uuid>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Housekeeping wire shapes
//
// Rooms board, task CRUD, and the daily forecast, as the handlers return
// them. The legacy `HousekeepingStatus` enum above is the typed DB wire;
// the handler board carries the raw status strings.
// ═══════════════════════════════════════════════════════════════════════════

/// One row on the housekeeping rooms board.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HkBoardRow {
    pub id: Uuid,
    pub room_number: String,
    pub floor: String,
    pub building: String,
    pub housekeeping_status: String,
    pub front_office_status: String,
    pub last_cleaned_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_inspected_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Wrapper list returned by the housekeeping board endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HkBoardList {
    pub items: Vec<HkBoardRow>,
}

/// Body of `PUT /v1/properties/{property_id}/housekeeping/{room_number}`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HkStatusChangeRequest {
    /// DIRTY | CLEAN | INSPECTED (defaults CLEAN).
    #[serde(default = "default_clean")]
    pub status: String,
}

/// Housekeeping status change result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HkStatusResult {
    pub room_number: String,
    pub housekeeping_status: String,
}

/// Body of `POST /v1/properties/{property_id}/housekeeping/tasks`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HkTaskCreateRequest {
    pub room_number: Option<String>,
    #[serde(default = "default_clean_task_type")]
    pub task_type: String,
    #[serde(default = "default_normal")]
    pub priority: String,
    pub assigned_to: Option<String>,
    pub notes: Option<String>,
}

/// One created housekeeping task.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HkTaskCreated {
    pub id: Uuid,
    pub property_id: Uuid,
    pub room_number: Option<String>,
    pub task_type: String,
    pub status: String,
    pub priority: String,
    pub assigned_to: Option<String>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// One housekeeping task as listed.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HkTaskView {
    pub id: Uuid,
    pub room_number: Option<String>,
    pub task_type: String,
    pub status: String,
    pub priority: String,
    pub assigned_to: Option<String>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Wrapper list of housekeeping tasks.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HkTaskList {
    pub items: Vec<HkTaskView>,
}

/// Body of `PUT /v1/properties/{property_id}/housekeeping/tasks/{task_id}`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HkTaskUpdateRequest {
    #[serde(default = "default_open")]
    pub status: String,
    pub assigned_to: Option<String>,
}

/// Housekeeping task update result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HkTaskUpdateResult {
    pub id: Uuid,
    pub status: String,
    pub assigned_to: Option<String>,
}

/// Daily housekeeping forecast for the property.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HousekeepingForecast {
    pub date: chrono::NaiveDate,
    pub expected_arrivals: i32,
    pub stayovers: i32,
    pub expected_departures: i32,
    pub dirty_rooms: i32,
    pub open_tasks: i32,
}

// ═══════════════════════════════════════════════════════════════════════════
// List of Values (LOV) wire shapes
// ═══════════════════════════════════════════════════════════════════════════

/// One LOV row as the handlers return it (property-scoped catalog entry).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LovRow {
    pub id: Uuid,
    pub lov_type: String,
    pub lov_code: String,
    pub lov_value: String,
    pub is_active: bool,
    pub sort_order: i32,
}

/// Wrapper list returned by the LOV endpoints (full or per-type).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LovList {
    pub items: Vec<LovRow>,
}

/// Body of `POST /v1/properties/{property_id}/lov`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LovCreateRequest {
    #[serde(default)]
    pub lov_type: String,
    #[serde(default)]
    pub lov_code: String,
    #[serde(default)]
    pub lov_value: String,
    #[serde(default = "default_true")]
    pub is_active: bool,
    #[serde(default)]
    pub sort_order: i32,
}

/// One LOV entry in the per-type listing (id/code/value only).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LovEntryView {
    pub id: Uuid,
    pub lov_code: String,
    pub lov_value: String,
}

/// Wrapper list of per-type LOV entries.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LovEntryList {
    pub items: Vec<LovEntryView>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Reservation wire shapes (reservations.rs handlers)
//
// The legacy `Reservation` struct above is the booking-engine contract; these
// structs pin the admin-API wire rows the reservation handlers actually
// return (`SELECT *` of `pms.reservations` - confirmation-centric, no
// computed rate). ToSchema on every member so the `docs/openapi.json`
// response schemas are real (OAPEC §2 rule 10 - the `serde_json::Value`
// body placeholder is retired).
// ═══════════════════════════════════════════════════════════════════════════

/// Wire shape of a reservation row as returned by the PMS API.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReservationResponse {
    pub id: Uuid,
    pub property_id: Uuid,
    pub confirmation_number: String,
    pub guest_profile_id: Option<Uuid>,
    pub room_type_id: Option<Uuid>,
    pub rate_plan_id: Option<Uuid>,
    pub arrival_date: chrono::NaiveDate,
    pub departure_date: chrono::NaiveDate,
    pub num_adults: i32,
    pub num_children: i32,
    pub num_rooms: i32,
    pub status: String,
    pub guarantee_type: String,
    pub guarantee_code: Option<String>,
    pub special_requests: Option<String>,
    pub package_code: Option<String>,
    pub market_code: Option<String>,
    pub source_code: Option<String>,
    pub channel_code: Option<String>,
    pub assigned_room_number: Option<String>,
    pub checked_in_at: Option<chrono::DateTime<chrono::Utc>>,
    pub checked_out_at: Option<chrono::DateTime<chrono::Utc>>,
    pub cancelled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Wrapper list returned by the reservations search endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReservationList {
    pub items: Vec<ReservationResponse>,
}

/// Body of `POST /v1/properties/{property_id}/reservations`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReservationCreateRequest {
    pub guest_profile_id: Option<Uuid>,
    pub room_type_id: Option<Uuid>,
    pub rate_plan_id: Option<Uuid>,
    pub arrival_date: String,
    pub departure_date: String,
    #[serde(default = "default_one")]
    pub num_adults: i32,
    #[serde(default)]
    pub num_children: i32,
    #[serde(default = "default_one")]
    pub num_rooms: i32,
    #[serde(default = "default_ccg")]
    pub guarantee_type: String,
    pub guarantee_code: Option<String>,
    pub special_requests: Option<String>,
    pub package_code: Option<String>,
    pub market_code: Option<String>,
    pub source_code: Option<String>,
    pub channel_code: Option<String>,
}

/// Reservation-split result (original + the newly minted sibling).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SplitResult {
    pub original_reservation_id: Uuid,
    pub new_reservation_id: Uuid,
    pub new_confirmation_number: String,
}

/// Body of `POST .../reservations/{res_id}/upsell` (room-type/rate-plan swap).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpsellRequest {
    pub new_room_type_id: Option<Uuid>,
    pub new_rate_plan_id: Option<Uuid>,
}

/// Upsell result echoes the updated confirmation.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpsellResult {
    pub reservation_id: Uuid,
    pub confirmation_number: String,
    pub status: String,
}

/// Guest snapshot attached to the arrival instructions.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InstructionProfile {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub loyalty_number: Option<String>,
}

/// Package summary attached to the arrival instructions.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InstructionPackage {
    pub package_name: String,
    pub price: String,
}

/// A reservation comment.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InstructionComment {
    pub id: Uuid,
    pub comment_type: String,
    pub content: String,
    pub author: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// `GET .../reservations/{res_id}/instructions` - arrival packet.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ArrivalInstructions {
    pub reservation: ReservationResponse,
    pub profile: Option<InstructionProfile>,
    pub package: Option<InstructionPackage>,
    pub comments: Vec<InstructionComment>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Front-desk wire shapes (front_desk.rs handlers)
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `POST .../reservations/{res_id}/check-in` (room assignment).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CheckInRequest {
    pub room_number: Option<String>,
}

/// Body of `POST .../reservations/{res_id}/check-out` (folio settlement hint).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CheckOutRequest {
    pub payment_method: Option<String>,
}

/// Arrival front-desk move result (shared by check-in / check-out).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FrontDeskMoveResult {
    pub reservation_id: Uuid,
    pub status: String,
    pub room_number: Option<String>,
}

/// Body of `POST .../reservations/{res_id}/pre-check-in`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PreCheckInRequest {
    pub arrival_time: Option<String>,
    pub special_requests: Option<String>,
}

/// Pre-registration result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PreCheckInResult {
    pub reservation_id: Uuid,
    pub confirmation_number: String,
    pub status: String,
}

/// One guest message thread entry.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MessageView {
    pub id: Uuid,
    pub direction: String,
    pub message_type: String,
    pub content: String,
    pub sender: Option<String>,
    pub is_read: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Wrapper list of guest messages.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MessageList {
    pub items: Vec<MessageView>,
}

/// Body of `POST .../reservations/{res_id}/messages`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SendMessageRequest {
    #[serde(default)]
    pub content: String,
    #[serde(default = "default_message_type")]
    pub message_type: String,
    pub sender: Option<String>,
}

/// Sent message (is_read is false by construction; omitted in the POST reply).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SentMessage {
    pub id: Uuid,
    pub direction: String,
    pub message_type: String,
    pub content: String,
    pub sender: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// One service request attached to a stay.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceRequestView {
    pub id: Uuid,
    pub request_type: String,
    pub description: Option<String>,
    pub status: String,
    pub assigned_to: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Wrapper list of service requests.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceRequestList {
    pub items: Vec<ServiceRequestView>,
}

/// Body of `POST .../reservations/{res_id}/service-requests`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateServiceRequestRequest {
    #[serde(default = "default_housekeeping")]
    pub request_type: String,
    pub description: Option<String>,
}

/// Created service request (echoes the owning reservation).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceRequestCreated {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub request_type: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Cashiering wire shapes (cashiering.rs handlers)
//
// The legacy `PostChargeRequest`/`PostPaymentRequest` structs above carry
// `Decimal` amounts for the booking-engine client; the admin-API wire
// carries decimal amounts as strings (NUMERIC via `::text`), so the request
// bodies below are the wire-faithful variants. Charge/payment row views
// reuse `FolioChargeView`/`FolioPaymentView` from the guest-app section.
// ═══════════════════════════════════════════════════════════════════════════

/// Guest folio with charges and payments.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FolioResponse {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub property_id: Uuid,
    pub status: String,
    pub closed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub payment_method: Option<String>,
    pub charges: Vec<FolioChargeView>,
    pub payments: Vec<FolioPaymentView>,
}

/// Body of `POST .../charges` (amount is a decimal string).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChargePostRequest {
    #[serde(default = "default_room_code")]
    pub charge_code: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default = "default_one")]
    pub quantity: i32,
}

/// The inserted folio-charge row.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostedCharge {
    pub id: Uuid,
    pub folio_id: Uuid,
    pub charge_code: String,
    pub description: String,
    pub amount: String,
    pub quantity: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Body of `POST .../payments` (amount is a decimal string).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentPostRequest {
    #[serde(default = "default_cash_code")]
    pub payment_code: String,
    #[serde(default)]
    pub amount: String,
    pub reference: Option<String>,
}

/// The inserted folio-payment row.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostedPayment {
    pub id: Uuid,
    pub folio_id: Uuid,
    pub payment_code: String,
    pub amount: String,
    pub reference: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Folio-generation result (the printable folio number).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FolioGenerated {
    pub reservation_id: Uuid,
    pub folio_number: String,
    pub status: String,
}

/// Body of `POST .../charges/dispute` (the disputed charge is rebated).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DisputeChargeRequest {
    pub charge_id: Uuid,
    #[serde(default)]
    pub reason: String,
    #[serde(default = "default_zero_str")]
    pub rebate_amount: String,
}

/// Charge-dispute result (the posted rebate row).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DisputeResult {
    pub rebate_id: Uuid,
    pub original_charge_id: Uuid,
    pub reason: String,
    /// Negative rebate amount as posted (NUMERIC as text).
    pub rebate_amount: String,
}

/// One transaction code; `id` is `None` when served from system defaults.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransactionCode {
    pub id: Option<Uuid>,
    pub code: String,
    pub description: String,
    pub category: String,
    pub is_active: bool,
}

/// Wrapper list of transaction codes, with the serving branch.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TransactionCodeList {
    pub items: Vec<TransactionCode>,
    /// "property" | "defaults"
    pub source: String,
}

/// Body of `POST .../vouchers/validate`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VoucherValidateRequest {
    #[serde(default)]
    pub voucher_number: String,
}

/// Gift-voucher validation (all detail fields `None` when not found).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VoucherValidation {
    pub valid: bool,
    pub voucher_number: String,
    pub balance: Option<String>,
    pub currency_code: Option<String>,
    pub status: Option<String>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error: Option<String>,
}

/// Body of `POST .../vouchers/redeem` (amount is a decimal string).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VoucherRedeemRequest {
    #[serde(default)]
    pub voucher_number: String,
    #[serde(default = "default_zero_str")]
    pub amount: String,
}

/// Voucher redemption result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VoucherRedeemResult {
    pub voucher_number: String,
    pub redeemed_amount: String,
    pub remaining_balance: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// Rooms & inventory wire shapes (rooms.rs handlers)
//
// The typed room/room-type CRUD already returns the `RoomType`/`Room`
// contracts; these structs pin the inventory board and the sellable-item
// CRUD rows the remaining handlers return.
// ═══════════════════════════════════════════════════════════════════════════

/// One inventory row as the inventory board returns it.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InventoryRow {
    pub property_id: Uuid,
    pub room_type_id: Uuid,
    pub date: chrono::NaiveDate,
    pub physical_rooms: i32,
    pub sell_limit: i32,
    pub sold_rooms: i32,
    pub available_rooms: i32,
    pub out_of_order: i32,
    pub out_of_service: i32,
}

/// Wrapper list returned by the inventory endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InventoryList {
    pub items: Vec<InventoryRow>,
}

/// Body of `PUT .../inventory/sell-limits`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SellLimitUpdateRequest {
    pub room_type_id: Uuid,
    #[serde(default = "default_schedule_start")]
    pub start_date: String,
    #[serde(default = "default_schedule_end")]
    pub end_date: String,
    #[serde(default)]
    pub sell_limit: i32,
    pub reason: Option<String>,
}

/// Sell-limit update result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SellLimitResult {
    pub status: String,
    pub room_type_id: Uuid,
    pub sell_limit: i32,
}

/// Body of `POST .../items` (prices are decimal strings).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ItemCreateRequest {
    pub item_code: String,
    pub item_name: String,
    #[serde(default = "default_general")]
    pub category: String,
    #[serde(default = "default_zero_str")]
    pub unit_price: String,
    #[serde(default = "default_zero_str")]
    pub unit_cost: String,
    #[serde(default)]
    pub stock_qty: i32,
    #[serde(default)]
    pub reorder_point: i32,
    #[serde(default = "default_true")]
    pub is_sellable: bool,
}

/// A sellable/stock item (NUMERIC amounts as text).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ItemView {
    pub id: Uuid,
    pub item_code: String,
    pub item_name: String,
    pub category: String,
    pub unit_price: String,
    pub unit_cost: String,
    pub stock_qty: i32,
    pub reorder_point: i32,
    pub is_sellable: bool,
}

/// Wrapper list returned by the items endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ItemList {
    pub items: Vec<ItemView>,
}

/// Body of `PUT .../items/{item_id}` (all fields optional - partial update).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ItemUpdateRequest {
    pub item_name: Option<String>,
    pub unit_price: Option<String>,
    pub unit_cost: Option<String>,
    pub stock_qty: Option<i32>,
    pub reorder_point: Option<i32>,
    pub is_sellable: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Front-office wire shapes (rooms.rs in-house board)
// ═══════════════════════════════════════════════════════════════════════════

/// One in-house stay on the front-desk board.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InHouseStay {
    pub id: Uuid,
    pub confirmation_number: String,
    pub guest_profile_id: Option<Uuid>,
    pub room_type_id: Option<Uuid>,
    /// Assigned room (from `assigned_room_number`).
    pub room_number: Option<String>,
    pub arrival_date: chrono::NaiveDate,
    pub departure_date: chrono::NaiveDate,
    pub num_adults: i32,
    pub num_children: i32,
}

/// Wrapper list returned by the in-house endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InHouseList {
    pub items: Vec<InHouseStay>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Back-office wire shapes (back_office.rs handlers)
//
// Reports, property config, the End-of-Day pipeline, the async job family,
// external-system delivery config, and property module toggles, as the
// handlers return them. `get_async_result` stays a free-form passthrough
// (per-module stored document) and is deliberately not typed here.
// ═══════════════════════════════════════════════════════════════════════════

/// One day of the occupancy report.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OccupancyDay {
    pub date: chrono::NaiveDate,
    pub reservations: i64,
    pub guests: i64,
}

/// `GET /v1/properties/{property_id}/reports/occupancy`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OccupancyReport {
    pub items: Vec<OccupancyDay>,
}

/// One revenue line of the revenue report (NUMERIC total as text).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RevenueLine {
    pub charge_code: String,
    pub total: String,
}

/// `GET /v1/properties/{property_id}/reports/revenue`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RevenueReport {
    pub items: Vec<RevenueLine>,
}

/// `GET /v1/properties/{property_id}/config` (languages JSONB as stored).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PropertyConfig {
    pub id: Uuid,
    pub property_code: String,
    pub property_name: String,
    pub timezone: String,
    pub currency_code: String,
    /// Languages JSONB as stored (array of BCP-47 codes).
    pub languages: serde_json::Value,
}

/// Summary block attached to the `generate_reports` EOD step.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EodReportSummary {
    pub business_date: chrono::NaiveDate,
    pub occupied_rooms: i32,
    pub total_rooms: i32,
}

/// One executed End-of-Day step (optional payload varies per step name).
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct EodStep {
    pub step: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unbalanced_folios: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges_posted: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows_affected: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_summary: Option<EodReportSummary>,
}

/// `POST /v1/properties/{property_id}/end-of-day`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EndOfDayResult {
    pub eod_id: Uuid,
    pub property_id: Uuid,
    pub business_date: chrono::NaiveDate,
    pub status: String,
    pub steps: Vec<EodStep>,
}

/// `POST .../end-of-day/{eod_id}/retry` (resume from the failed step).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EodRetryResult {
    pub eod_id: Uuid,
    pub property_id: Uuid,
    pub business_date: chrono::NaiveDate,
    pub status: String,
    pub resumed_from: String,
    pub steps: Vec<EodStep>,
}

/// `GET /v1/properties/{property_id}/reports/reservation-summary`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReservationSummaryReport {
    pub property_id: Uuid,
    pub start_date: String,
    pub end_date: String,
    pub total_reservations: i32,
    pub in_house: i32,
    pub checked_out: i32,
    pub cancelled: i32,
    pub active_reservations: i32,
    pub total_guests: i32,
}

/// Body of `POST /v1/properties/{property_id}/async` (module job launch).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AsyncJobRequest {
    #[serde(default = "default_inventory_module")]
    pub module: String,
    #[serde(default = "default_bulk_update_op")]
    pub operation: String,
}

/// `GET /v1/async/status/{job_id}`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AsyncJobStatus {
    pub id: Uuid,
    pub module: String,
    pub operation: String,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Body of `POST .../async/inventory`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InventoryAsyncRequest {
    #[serde(default = "default_bulk_sell_limit")]
    pub operation: String,
}

/// Body of `POST .../async/cashiering`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CashieringAsyncRequest {
    #[serde(default = "default_bulk_folio")]
    pub operation: String,
}

/// Body of `POST .../async/crm`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CrmAsyncRequest {
    #[serde(default = "default_profile_merge")]
    pub operation: String,
}

/// Body of `POST .../async/par`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ParAsyncRequest {
    #[serde(default = "default_par_report")]
    pub operation: String,
}

/// One external-system delivery configuration (webhook + retry policy).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeliveryConfig {
    pub system: String,
    pub webhook_url: String,
    pub retry_count: i32,
    pub retry_delay_seconds: i32,
    pub is_enabled: bool,
}

/// `GET /v1/properties/{property_id}/external-systems`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExternalSystemConfig {
    pub property_id: Uuid,
    pub delivery_configs: Vec<DeliveryConfig>,
}

/// Mutation acknowledgement (property + "updated") shared by the
/// external-system and module-config PUTs.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateAcknowledgement {
    pub property_id: Uuid,
    pub status: String,
}

/// One property module toggle.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PropertyModule {
    pub module: String,
    pub is_enabled: bool,
}

/// `GET /v1/properties/{property_id}/modules` (also the PUT body).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PropertyModuleList {
    pub items: Vec<PropertyModule>,
}

/// One failed/stuck End-of-Day run.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EodException {
    pub id: Uuid,
    pub business_date: chrono::NaiveDate,
    pub status: String,
    pub step: String,
    /// Stored error payload from the failed step (free-form).
    pub error_details: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// `GET /v1/properties/{property_id}/end-of-day/exceptions`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EodExceptionList {
    pub property_id: Uuid,
    pub exceptions: Vec<EodException>,
}

/// One guest line of the guest-history report (NUMERIC revenue as text).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuestHistoryRow {
    pub profile_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub vip_status: String,
    pub stay_count: i64,
    pub total_revenue: String,
}

/// `GET /v1/properties/{property_id}/reports/guest-history`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GuestHistoryReport {
    pub property_id: Uuid,
    pub guests: Vec<GuestHistoryRow>,
}

/// One task line of the housekeeping-analysis report.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HousekeepingAnalysisRow {
    pub id: Uuid,
    pub task_type: String,
    pub status: String,
    pub priority: Option<String>,
    pub assigned_to: Option<String>,
    pub room_number: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub turnaround_seconds: Option<i32>,
}

/// `GET .../reports/housekeeping-analysis`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HousekeepingAnalysisReport {
    pub property_id: Uuid,
    pub tasks: Vec<HousekeepingAnalysisRow>,
}

/// One block line of the sales & catering report (NUMERIC rate as text).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SalesCateringRow {
    pub id: Uuid,
    pub block_code: String,
    pub block_name: String,
    pub status: String,
    pub block_type: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub negotiated_rate: String,
    pub event_count: i64,
    pub total_capacity: i64,
}

/// `GET .../reports/sales-catering`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SalesCateringReport {
    pub property_id: Uuid,
    pub blocks: Vec<SalesCateringRow>,
}

/// One channel line of the channel-performance report (revenue as text).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelPerformanceRow {
    pub source_code: String,
    pub reservation_count: i64,
    pub active_count: i64,
    pub revenue: String,
}

/// `GET .../reports/channel-performance`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChannelPerformanceReport {
    pub property_id: Uuid,
    pub start_date: String,
    pub end_date: String,
    pub channels: Vec<ChannelPerformanceRow>,
}

/// One segment line of the market-segment report (revenue as text).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MarketSegmentRow {
    pub market_code: String,
    pub reservation_count: i64,
    pub revenue: String,
    pub avg_party_size: i32,
}

/// `GET .../reports/market-segment`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MarketSegmentReport {
    pub property_id: Uuid,
    pub start_date: String,
    pub end_date: String,
    pub segments: Vec<MarketSegmentRow>,
}

/// One rate-plan line of the rate-analysis report (NUMERIC amounts as text).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RateAnalysisRow {
    pub rate_plan_code: String,
    pub rate_plan_name: String,
    pub rate_category: String,
    pub reservation_count: i64,
    pub total_revenue: String,
    pub adr: String,
}

/// `GET .../reports/rate-analysis`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RateAnalysisReport {
    pub property_id: Uuid,
    pub rate_plans: Vec<RateAnalysisRow>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Blocks & events wire shapes (blocks.rs / events.rs handlers)
//
// The legacy `Block`/`BlockCreate`/`Event`/`EventCreate` structs above are
// the booking-engine contracts; these pin the admin-API wire rows.
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `POST /v1/properties/{property_id}/blocks`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BlockCreateRequest {
    #[serde(default)]
    pub block_name: String,
    pub room_type_id: Option<Uuid>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    #[serde(default)]
    pub blocked_rooms: i32,
    #[serde(default = "default_released_until")]
    pub released_until: String,
    #[serde(default = "default_allotment")]
    pub block_type: String,
    pub negotiated_rate: Option<String>,
    pub cutoff_date: Option<String>,
}

/// Created block (provisional until pickup).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BlockCreated {
    pub id: Uuid,
    pub block_code: String,
    pub status: String,
}

/// One row of the blocks list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BlockSummary {
    pub id: Uuid,
    pub block_code: String,
    pub block_name: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub blocked_rooms: i32,
    pub picked_up: i32,
    pub released_until: String,
    pub status: String,
}

/// Wrapper list returned by the blocks endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BlockList {
    pub items: Vec<BlockSummary>,
}

/// Body of the block status transition.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BlockStatusChange {
    #[serde(default = "default_provisional")]
    pub status: String,
}

/// Legal next statuses for a block.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BlockNextStatuses {
    pub current_status: String,
    pub next_statuses: Vec<String>,
}

/// Created posting master (a master reservation for group billing).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostingMasterCreated {
    pub posting_master_id: Uuid,
    pub block_id: Uuid,
    pub confirmation_number: String,
}

/// One bookable function space.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FunctionSpace {
    pub id: Uuid,
    pub space_code: String,
    pub space_name: String,
    pub capacity: i32,
    /// Setup styles stored as a JSONB array of codes.
    #[serde(default)]
    pub setup_styles: Vec<String>,
}

/// Wrapper list returned by the function-spaces endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FunctionSpaceList {
    pub items: Vec<FunctionSpace>,
}

/// One registered event resource (`cost` is a decimal string).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventResourceRow {
    pub id: Uuid,
    pub resource_type: String,
    pub resource_name: String,
    pub quantity: i32,
    pub cost: String,
    pub notes: Option<String>,
}

/// Wrapper list returned by the event-resources endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventResourceList {
    pub items: Vec<EventResourceRow>,
}

/// One resource line in the PUT `.../events/{event_id}/resources` body.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventResourceInput {
    #[serde(default = "default_av_resource")]
    pub resource_type: String,
    #[serde(default)]
    pub resource_name: String,
    #[serde(default = "default_one")]
    pub quantity: i32,
    #[serde(default = "default_zero_str")]
    pub cost: String,
    pub notes: Option<String>,
}

/// Body of `PUT .../events/{event_id}/resources`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventResourceUpdate {
    pub resources: Vec<EventResourceInput>,
}

/// A resource line registered by the PUT.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventResourceAdded {
    pub id: Uuid,
    pub resource_type: String,
    pub resource_name: String,
}

/// PUT event-resources result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventResourceUpdateResult {
    pub event_id: Uuid,
    pub added: Vec<EventResourceAdded>,
}

/// Body of `POST /v1/properties/{property_id}/events`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventCreateRequest {
    #[serde(default)]
    pub event_name: String,
    #[serde(default = "default_meeting")]
    pub event_type: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    #[serde(default)]
    pub expected_attendees: i32,
    pub banquet_order: Option<String>,
    pub contact_name: Option<String>,
    pub contact_phone: Option<String>,
    pub notes: Option<String>,
}

/// Created event (tentative until confirmed).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventCreated {
    pub id: Uuid,
    pub event_code: String,
    pub status: String,
}

/// One row of the events list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventSummary {
    pub id: Uuid,
    pub event_code: String,
    pub event_name: String,
    pub event_type: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub status: String,
    pub expected_attendees: i32,
}

/// Wrapper list returned by the events endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EventSummaryList {
    pub items: Vec<EventSummary>,
}

// ═══════════════════════════════════════════════════════════════════════════
// GraphQL envelope (graphql.rs)
//
// `data` stays `serde_json::Value` deliberately: a GraphQL payload is
// query-shaped by definition; only the envelope is fixed.
// ═══════════════════════════════════════════════════════════════════════════

/// A GraphQL error (message-only, current implementation).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GraphQlError {
    pub message: String,
}

/// GraphQL-style response envelope.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GraphQlResponse {
    /// Result data (null on error-only responses).
    pub data: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GraphQlError>>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Integration wire shapes (integration.rs handlers)
//
// Outbound platform-service integrations (Payments, Ledger, Storage, Post,
// Flow, Func) as the handlers surface them.
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `POST .../reservations/{res_id}/payments-process`
/// (bill8 Payments token - never raw card data).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentProcessRequest {
    /// bill8 Payments token (PCI: no raw card data crosses PMS).
    #[serde(default)]
    pub payment_method_token: String,
    #[serde(default)]
    pub amount: f64,
    #[serde(default = "default_usd")]
    pub currency: String,
}

/// Payment-authorization result (the immutable Payments charge id as token).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaymentProcessResult {
    pub reservation_id: Uuid,
    pub token: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub pci_compliant: bool,
}

/// Body of `POST .../reservations/{res_id}/ledger-post`
/// (balance-id legs; `amount` is a decimal string).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LedgerPostRequest {
    /// Optional pre-resolved debit balance (alias: debit_account).
    #[serde(alias = "debit_account")]
    pub debit_balance_id: Option<Uuid>,
    /// Optional pre-resolved credit balance (alias: credit_account).
    #[serde(alias = "credit_account")]
    pub credit_balance_id: Option<Uuid>,
    /// Required; decimal string.
    pub amount: String,
    #[serde(default = "default_cad")]
    pub currency: String,
}

/// Ledger posting result (the immutable balance-transaction id).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LedgerPostResult {
    pub posting_id: Uuid,
    pub posting_ref: String,
    pub debit_balance_id: Uuid,
    pub credit_balance_id: Uuid,
    pub amount: String,
    pub status: String,
}

/// Body of `POST .../images` (base64 content; empty = metadata-only call).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UploadImageRequest {
    #[serde(default = "default_property_image")]
    pub image_type: String,
    /// Base64 of the bytes (empty = metadata-only call).
    #[serde(default)]
    pub content_base64: String,
    #[serde(default = "default_jpeg")]
    pub content_type: String,
}

/// Image upload result (the Storage object key).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UploadImageResult {
    pub property_id: Uuid,
    pub object_key: String,
    pub image_type: String,
    pub status: String,
}

/// Body of `POST .../notifications`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SendNotificationRequest {
    #[serde(default = "default_email_channel")]
    pub channel: String,
    #[serde(default = "default_confirmation")]
    pub template: String,
    #[serde(default)]
    pub recipient: String,
}

/// Notification-enqueue result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SendNotificationResult {
    pub message_id: String,
    pub channel: String,
    pub template: String,
    pub recipient: String,
    pub status: String,
}

/// Per-operation rate-limit windows enforced at Sentin8:BFF.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RateLimits {
    pub search_availability: String,
    pub create_reservation: String,
    pub post_charge: String,
}

/// `GET .../rate-limits`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RateLimitConfig {
    pub rate_limits: RateLimits,
    pub enforced_by: String,
}

/// One local audit-journal entry (read-back of the service's own journal).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuditLogEntry {
    pub id: Uuid,
    pub entry: String,
    pub action: String,
    pub target_resource_type: Option<String>,
    pub target_resource_id: Option<String>,
    pub actor: Option<String>,
    pub severity: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Wrapper list returned by the audit-log endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuditLogList {
    pub items: Vec<AuditLogEntry>,
}

/// PCI posture checks evaluated by the handler.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PciChecks {
    pub no_raw_card_data_stored: bool,
    pub all_payments_via_bill8_payments_tokens: bool,
    pub no_card_numbers_in_logs: bool,
    pub no_card_numbers_in_database: bool,
}

/// `GET .../pci-compliance`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PciCompliance {
    pub property_id: Uuid,
    pub pci_compliant: bool,
    pub checks: PciChecks,
}

/// Body of `POST .../flow/workflows`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FlowWorkflowStartRequest {
    #[serde(default = "default_group_booking")]
    pub workflow_type: String,
}

/// Flow workflow start result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FlowWorkflowStarted {
    pub workflow_id: Uuid,
    pub property_id: Uuid,
    pub workflow_type: String,
    pub status: String,
    pub message: String,
}

/// Body of `POST .../func/invoke`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FuncInvokeRequest {
    #[serde(default = "default_rate_calculator")]
    pub module_name: String,
}

/// Func module execution payload.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FuncInvokePayload {
    pub computed: bool,
    pub note: String,
}

/// Func invocation result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FuncInvokeResult {
    pub invocation_id: Uuid,
    pub property_id: Uuid,
    pub module_name: String,
    pub status: String,
    pub result: FuncInvokePayload,
}

/// Body of `POST .../post/emails`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostEmailRequest {
    #[serde(default = "default_confirmation")]
    pub email_type: String,
    #[serde(default)]
    pub to: String,
}

/// Post email enqueue result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostEmailQueued {
    pub message_id: Uuid,
    pub property_id: Uuid,
    pub email_type: String,
    pub to: String,
    pub status: String,
    pub message: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// Rate-management wire shapes (rate_plans.rs handlers)
//
// The legacy `RateSchedule`/`RateScheduleCreate`/`Package`/`PackageCreate`
// structs above are the booking-engine contracts; these pin the
// schedule/package/negotiated/promo wire rows.
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `POST .../rate-plans/{plan_id}/schedules` (amounts decimal strings).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ScheduleCreateRequest {
    pub room_type_id: Uuid,
    /// YYYY-MM-DD (default 2020-01-01 when absent).
    #[serde(default = "default_schedule_start")]
    pub start_date: String,
    /// YYYY-MM-DD (default 2030-12-31 when absent).
    #[serde(default = "default_schedule_end")]
    pub end_date: String,
    #[serde(default = "default_zero_str")]
    pub rate_amount: String,
    pub rate_amount_single: Option<String>,
    pub rate_amount_double: Option<String>,
    pub rate_amount_extra_adult: Option<String>,
    pub rate_amount_extra_child: Option<String>,
    #[serde(default = "default_full_week")]
    pub days_of_week: Vec<String>,
    pub package_code: Option<String>,
}

/// Created rate schedule (deduped per plan/room-type/window).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ScheduleCreated {
    pub id: Uuid,
    pub rate_plan_id: Uuid,
    pub room_type_id: Uuid,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    /// NUMERIC amount as text.
    pub rate_amount: String,
}

/// One rate-schedule row of the schedules list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ScheduleView {
    pub id: Uuid,
    pub room_type_id: Uuid,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    /// Day codes the schedule rate applies to (["MON", ...]).
    #[serde(default)]
    pub days_of_week: Vec<String>,
    pub rate_amount: String,
    pub rate_amount_single: Option<String>,
    pub rate_amount_double: Option<String>,
    pub package_code: Option<String>,
}

/// Wrapper list returned by the schedules endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ScheduleList {
    pub items: Vec<ScheduleView>,
}

/// Body of `POST /v1/properties/{property_id}/packages`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PackageCreateRequest {
    #[serde(default)]
    pub package_code: String,
    #[serde(default)]
    pub package_name: String,
    #[serde(default = "default_one")]
    pub quantity: i32,
    #[serde(default = "default_zero_str")]
    pub price: String,
    #[serde(default)]
    pub per_stay: bool,
}

/// Created package (DELIBERATELY DISTINCT from the unused legacy
/// `PackageCreate` - the admin wire rows differ).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PackageCreated {
    pub id: Uuid,
    pub package_code: String,
    pub package_name: String,
    pub quantity: i32,
    /// NUMERIC price as text.
    pub price: String,
    pub per_stay: bool,
}

/// One package row of the packages list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PackageDetailView {
    pub id: Uuid,
    pub package_code: String,
    pub package_name: String,
    pub quantity: i32,
    /// NUMERIC price as text.
    pub price: String,
    pub per_stay: bool,
    pub is_active: bool,
}

/// Wrapper list returned by the packages endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PackageDetailList {
    pub items: Vec<PackageDetailView>,
}

/// One negotiated rate plan (corporate / government / AAA).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NegotiatedRatePlan {
    pub id: Uuid,
    pub rate_plan_code: String,
    pub rate_plan_name: String,
    pub rate_category: String,
    pub currency_code: String,
    pub is_active: bool,
    pub market_code: Option<String>,
    pub source_code: Option<String>,
    pub description: Option<String>,
}

/// Wrapper list returned by the negotiated-rate-plans endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NegotiatedRatePlanList {
    pub items: Vec<NegotiatedRatePlan>,
}

/// Body of `POST .../negotiated-rate-plans/{rate_plan_id}/link-company`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LinkCompanyRequest {
    pub company_profile_id: Uuid,
}

/// Negotiated-rate↔company link result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LinkCompanyResult {
    pub linked: bool,
    pub rate_plan_id: Uuid,
    pub company_profile_id: Uuid,
}

/// Body of `POST /v1/properties/{property_id}/promo/validate`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PromoValidateRequest {
    /// Required; the PROMO rate_plan_code.
    pub promo_code: String,
    /// Optional stay window for restriction warnings.
    pub check_in: Option<String>,
    /// Optional stay window for restriction warnings.
    pub check_out: Option<String>,
}

/// Promo-code validation (not-found carries only `reason`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PromoValidation {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub rate_plan_id: Option<Uuid>,
    pub rate_plan_code: Option<String>,
    pub rate_plan_name: Option<String>,
    pub currency_code: Option<String>,
    /// Restriction warnings (MinLOS/MaxLOS/MinAdvance) when a stay window was supplied.
    pub warnings: Option<Vec<String>>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Profiles/CRM wire shapes (profiles.rs handlers)
//
// `preferences` fields stay `serde_json::Value` deliberately: the CRM
// attribute bag is free-form by design.
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `POST /v1/properties/{property_id}/profiles`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProfileCreateRequest {
    #[serde(default = "default_guest_type")]
    pub profile_type: String,
    #[serde(default = "default_none_vip")]
    pub vip_status: String,
    #[serde(default)]
    pub first_name: String,
    #[serde(default)]
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    /// Free-form address (stored as TEXT in `guest_profiles`).
    pub address: Option<String>,
    pub company_name: Option<String>,
    pub loyalty_number: Option<String>,
    pub preferred_language: Option<String>,
    /// CRM attribute bag (free-form by design).
    pub preferences: Option<serde_json::Value>,
}

/// Created profile (the INSERT RETURNING row).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProfileCreated {
    pub id: Uuid,
    pub profile_type: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// One row of the profiles list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProfileSummary {
    pub id: Uuid,
    pub profile_type: String,
    pub vip_status: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company_name: Option<String>,
    pub loyalty_number: Option<String>,
}

/// Wrapper list returned by the profiles endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProfileList {
    pub items: Vec<ProfileSummary>,
}

/// Full profile detail (the `SELECT *` row).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProfileDetail {
    pub id: Uuid,
    pub property_id: Uuid,
    pub profile_type: String,
    pub vip_status: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub company_name: Option<String>,
    pub loyalty_number: Option<String>,
    pub preferred_language: Option<String>,
    /// CRM attribute bag (free-form by design).
    pub preferences: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Body of `POST .../profiles/{profile_id}/preferences`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AddPreferenceRequest {
    #[serde(default = "default_room_pref")]
    pub preference_type: String,
    #[serde(default)]
    pub preference_code: String,
    pub preference_value: Option<String>,
}

/// One guest preference row.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PreferenceView {
    pub id: Uuid,
    pub preference_type: String,
    pub preference_code: String,
    pub preference_value: Option<String>,
}

/// Wrapper list returned by the preferences endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PreferenceList {
    pub items: Vec<PreferenceView>,
}

/// Body of `POST .../profiles/{profile_id}/loyalty`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LinkLoyaltyRequest {
    #[serde(default)]
    pub loyalty_number: String,
    pub program_code: Option<String>,
}

/// Loyalty-link result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoyaltyLinkResult {
    pub profile_id: Uuid,
    pub loyalty_number: String,
}

/// One stay of the stay-history list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StayHistoryRow {
    pub id: Uuid,
    pub confirmation_number: String,
    pub arrival_date: chrono::NaiveDate,
    pub departure_date: chrono::NaiveDate,
    pub status: String,
    pub num_adults: i32,
    pub num_children: i32,
    pub room_number: Option<String>,
}

/// Wrapper list returned by the stay-history endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StayHistoryList {
    pub items: Vec<StayHistoryRow>,
}

/// Body of `POST /v1/properties/{property_id}/profiles/merge`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MergeProfilesRequest {
    pub source_profile_id: Uuid,
    pub target_profile_id: Uuid,
}

/// Profile-merge result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MergeProfilesResult {
    pub merged: bool,
    pub source_profile_id: Uuid,
    pub target_profile_id: Uuid,
    pub reservations_moved: u64,
}

/// Body of `PUT .../profiles/{profile_id}/loyalty-number`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SetLoyaltyNumberRequest {
    pub loyalty_number: String,
}

/// Loyalty-number set result.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoyaltyNumberSet {
    pub profile_id: Uuid,
    pub loyalty_number: Option<String>,
    pub linked: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// Staff & corporate wire shapes (staff_corporate.rs handlers)
// ═══════════════════════════════════════════════════════════════════════════

/// Body of `PUT .../reservations/{res_id}` (partial edit; all optional).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReservationUpdateRequest {
    pub arrival_date: Option<String>,
    pub departure_date: Option<String>,
    pub num_adults: Option<i32>,
    pub num_children: Option<i32>,
    pub num_rooms: Option<i32>,
    pub room_type_id: Option<Uuid>,
    pub rate_plan_id: Option<Uuid>,
    pub special_requests: Option<String>,
}

/// Walk-in request body (`arrival_date`/`departure_date` are required).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WalkInRequest {
    pub room_type_id: Uuid,
    pub arrival_date: String,
    pub departure_date: String,
    #[serde(default = "default_walk_in_last_name")]
    pub last_name: String,
    #[serde(default)]
    pub first_name: String,
    #[serde(default = "default_one")]
    pub num_rooms: i32,
}

/// Walk-in booking result (also drives the kiosk confirmation screen).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WalkInCreated {
    pub id: Uuid,
    pub confirmation_number: String,
    pub guest_profile_id: Uuid,
    pub room_type_code: String,
    pub arrival_date: chrono::NaiveDate,
    pub departure_date: chrono::NaiveDate,
    pub status: String,
}

/// A corporate credit method (POSTPAY / BILL_TO_COMPANY / PENDING / REJECTED).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompanyPaymentMethod {
    pub id: Uuid,
    pub company_profile_id: Uuid,
    pub label: String,
    pub payment_method_id: Option<String>,
    pub approval: String,
    /// NUMERIC credit limit as text (None when unapproved).
    pub credit_limit: Option<String>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Wrapper list returned by the company payment-methods endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompanyPaymentMethodList {
    pub items: Vec<CompanyPaymentMethod>,
}

/// Body of `POST .../companies/{profile_id}/payment-methods`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompanyPaymentMethodInput {
    #[serde(default = "default_pending")]
    pub approval: String,
    #[serde(default = "default_corporate_label")]
    pub label: String,
    pub payment_method_id: Option<String>,
    /// Decimal string; empty = no limit.
    pub credit_limit: Option<String>,
}

/// One traveler binding on the company roster.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompanyUserView {
    pub id: Uuid,
    pub company_profile_id: Uuid,
    pub traveler_profile_id: Uuid,
    pub traveler_name: String,
    pub email: Option<String>,
    pub role_at_company: String,
    pub is_default: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Wrapper list returned by the company users endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompanyUserList {
    pub items: Vec<CompanyUserView>,
}

/// Body of `POST .../companies/{profile_id}/users`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompanyUserInput {
    pub traveler_profile_id: Uuid,
    #[serde(default = "default_traveler_role")]
    pub role_at_company: String,
    #[serde(default)]
    pub is_default: bool,
}

/// Company-user bind result (the upsert RETURNING row).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompanyUserBound {
    pub id: Uuid,
    pub company_profile_id: Uuid,
    pub traveler_profile_id: Uuid,
    pub role_at_company: String,
    pub is_default: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// One company binding of the by-traveler roster.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TravelerCompany {
    pub company_profile_id: Uuid,
    pub company_name: String,
    pub role_at_company: String,
    pub is_default: bool,
}

/// Wrapper list returned by the companies-by-traveler endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TravelerCompanyList {
    pub items: Vec<TravelerCompany>,
}

/// Kiosk terminal mode configuration (source of truth for eKiosk bootstrap).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct KioskConfig {
    pub property_id: Uuid,
    pub enabled_modes: Vec<String>,
    pub addon_package_codes: Vec<String>,
    pub max_keys_per_stay: i32,
    pub lockdown: bool,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Body of `PUT .../kiosk-config` (defaults applied per-key when absent).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct KioskConfigUpdate {
    /// Defaults [CHECK_IN, CHECK_OUT, COLLECT_KEY] when empty.
    #[serde(default)]
    pub enabled_modes: Vec<String>,
    #[serde(default)]
    pub addon_package_codes: Vec<String>,
    #[serde(default = "default_max_keys")]
    pub max_keys_per_stay: i32,
    #[serde(default)]
    pub lockdown: bool,
}

/// First-admin provisioning status (no IdP + no bound ADMIN user).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AdminProvisioningStatus {
    pub company_profile_id: Uuid,
    pub requires_provisioning: bool,
    pub has_idp: bool,
    pub admin_count: i64,
}

/// Body of `PUT .../companies/{profile_id}/admin-provisioning`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProvisionAdminRequest {
    pub email: String,
    #[serde(default)]
    pub first_name: String,
    #[serde(default = "default_admin_last_name")]
    pub last_name: String,
}

/// Admin provisioning result (`traveler_profile_id` is `None` when already
/// provisioned).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AdminProvisioningResult {
    pub company_profile_id: Uuid,
    pub traveler_profile_id: Option<Uuid>,
    pub already_provisioned: bool,
    pub requires_provisioning: bool,
}

// serde field defaults shared by the wire-shape sections above.
fn default_one() -> i32 {
    1
}

// ── Back-office defaults ──
fn default_inventory_module() -> String {
    "INVENTORY".into()
}
fn default_bulk_update_op() -> String {
    "BULK_UPDATE".into()
}
fn default_bulk_sell_limit() -> String {
    "BULK_SELL_LIMIT_UPDATE".into()
}
fn default_bulk_folio() -> String {
    "BULK_FOLIO_POSTING".into()
}
fn default_profile_merge() -> String {
    "PROFILE_MERGE".into()
}
fn default_par_report() -> String {
    "PAR_REPORT_GENERATION".into()
}

// ── Blocks & events defaults ──
fn default_released_until() -> String {
    "0".into()
}
fn default_allotment() -> String {
    "ALLOTMENT".into()
}
fn default_provisional() -> String {
    "PROVISIONAL".into()
}
fn default_av_resource() -> String {
    "AV".into()
}
fn default_meeting() -> String {
    "MEETING".into()
}

// ── Integration defaults ──
fn default_usd() -> String {
    "USD".into()
}
fn default_cad() -> String {
    "CAD".into()
}
fn default_property_image() -> String {
    "PROPERTY".into()
}
fn default_jpeg() -> String {
    "image/jpeg".into()
}
fn default_email_channel() -> String {
    "EMAIL".into()
}
fn default_confirmation() -> String {
    "CONFIRMATION".into()
}
fn default_group_booking() -> String {
    "GROUP_BOOKING".into()
}
fn default_rate_calculator() -> String {
    "rate_calculator".into()
}

// ── Rate-management defaults ──
fn default_full_week() -> Vec<String> {
    vec![
        "MON".to_string(),
        "TUE".to_string(),
        "WED".to_string(),
        "THU".to_string(),
        "FRI".to_string(),
        "SAT".to_string(),
        "SUN".to_string(),
    ]
}

// ── Profiles defaults ──
fn default_guest_type() -> String {
    "GUEST".into()
}
fn default_none_vip() -> String {
    "NONE".into()
}
fn default_room_pref() -> String {
    "ROOM".into()
}

// ── Staff & corporate defaults ──
fn default_walk_in_last_name() -> String {
    "Walk-in".into()
}
fn default_pending() -> String {
    "PENDING".into()
}
fn default_corporate_label() -> String {
    "Corporate account".into()
}
fn default_traveler_role() -> String {
    "TRAVELER".into()
}
fn default_max_keys() -> i32 {
    4
}
fn default_admin_last_name() -> String {
    "Admin".into()
}
fn default_true() -> bool {
    true
}
fn default_ccg() -> String {
    "CCG".into()
}
fn default_cash_code() -> String {
    "CASH".into()
}
fn default_room_code() -> String {
    "ROOM".into()
}
fn default_zero_str() -> String {
    "0".into()
}
fn default_message_type() -> String {
    "MESSAGE".into()
}
fn default_housekeeping() -> String {
    "HOUSEKEEPING".into()
}
fn default_general() -> String {
    "GENERAL".into()
}
fn default_schedule_start() -> String {
    "2020-01-01".into()
}
fn default_schedule_end() -> String {
    "2030-12-31".into()
}
fn default_physical_key() -> String {
    "PHYSICAL".into()
}
fn default_clean() -> String {
    "CLEAN".into()
}
fn default_clean_task_type() -> String {
    "CLEAN".into()
}
fn default_normal() -> String {
    "NORMAL".into()
}
fn default_open() -> String {
    "OPEN".into()
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests - contract round-trips + wire format (H8T-2)
//
// These tests pin the JSON wire format of every status enum to the exact
// `CREATE TYPE hotel8.pms_* AS ENUM (...)` labels in services/pms/sql/init.sql
// (a drift here breaks every RLS query that string-compares statuses), plus
// serde round-trips for the wire-facing structs and Display impls.
//
// See hotel8/design/sentin8-hotel8-rust-test-strategy.md §2.1.
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    /// Every variant serializes to the exact DB enum label
    /// (`hotel8.pms_res_status` in services/pms/sql/init.sql).
    #[test]
    fn reservation_status_wire_format_matches_db_enum() {
        assert_eq!(
            serde_json::to_value(ReservationStatus::CheckedIn).unwrap(),
            "CHECKED_IN"
        );
        assert_eq!(
            serde_json::to_value(ReservationStatus::CheckedOut).unwrap(),
            "CHECKED_OUT"
        );
        assert_eq!(
            serde_json::to_value(ReservationStatus::Cancelled).unwrap(),
            "CANCELLED"
        );
        assert_eq!(
            serde_json::to_value(ReservationStatus::NoShow).unwrap(),
            "NO_SHOW"
        );
        assert_eq!(
            serde_json::to_value(ReservationStatus::Reserved).unwrap(),
            "RESERVED"
        );
        assert_eq!(
            serde_json::to_value(ReservationStatus::DueIn).unwrap(),
            "DUE_IN"
        );
        assert_eq!(
            serde_json::to_value(ReservationStatus::Waitlist).unwrap(),
            "WAITLIST"
        );
        // Deserialization is symmetric
        assert_eq!(
            serde_json::from_value::<ReservationStatus>("CHECKED_IN".into()).unwrap(),
            ReservationStatus::CheckedIn
        );
    }

    /// Front-office status wire format (`hotel8.pms_fo_status`) + Display.
    #[test]
    fn front_office_status_wire_and_display_match_db_labels() {
        for (variant, label) in [
            (FrontOfficeStatus::Vacant, "VACANT"),
            (FrontOfficeStatus::Occupied, "OCCUPIED"),
            (FrontOfficeStatus::DueOut, "DUE_OUT"),
            (FrontOfficeStatus::Departed, "DEPARTED"),
            (FrontOfficeStatus::Stayover, "STAYOVER"),
        ] {
            assert_eq!(
                serde_json::to_value(variant).unwrap(),
                label,
                "wire: {label}"
            );
            assert_eq!(variant.to_string(), label, "display: {label}");
            assert_eq!(
                serde_json::from_value::<FrontOfficeStatus>(label.into()).unwrap(),
                variant
            );
        }
    }

    /// Housekeeping status wire format (`hotel8.pms_hk_status`) + Display.
    #[test]
    fn housekeeping_status_wire_and_display_match_db_labels() {
        for (variant, label) in [
            (HousekeepingStatus::Clean, "CLEAN"),
            (HousekeepingStatus::Dirty, "DIRTY"),
            (HousekeepingStatus::Inspected, "INSPECTED"),
            (HousekeepingStatus::Pickup, "PICKUP"),
            (HousekeepingStatus::OutOfOrder, "OUT_OF_ORDER"),
            (HousekeepingStatus::OutOfService, "OUT_OF_SERVICE"),
        ] {
            assert_eq!(
                serde_json::to_value(variant).unwrap(),
                label,
                "wire: {label}"
            );
            assert_eq!(variant.to_string(), label, "display: {label}");
            assert_eq!(
                serde_json::from_value::<HousekeepingStatus>(label.into()).unwrap(),
                variant
            );
        }
    }

    /// Folio / Block / Event / VIP / ProfileType wire formats.
    #[test]
    fn remaining_status_wire_formats() {
        assert_eq!(serde_json::to_value(FolioStatus::Open).unwrap(), "OPEN");
        assert_eq!(
            serde_json::to_value(FolioStatus::Settled).unwrap(),
            "SETTLED"
        );
        assert_eq!(
            serde_json::to_value(BlockStatus::Definite).unwrap(),
            "DEFINITE"
        );
        assert_eq!(
            serde_json::to_value(EventStatus::Tentative).unwrap(),
            "TENTATIVE"
        );
        assert_eq!(serde_json::to_value(VipStatus::Vvip).unwrap(), "VVIP");
        assert_eq!(
            serde_json::to_value(ProfileType::TravelAgent).unwrap(),
            "TRAVEL_AGENT"
        );
        // Round-trip back
        assert_eq!(
            serde_json::from_value::<ProfileType>("TRAVEL_AGENT".into()).unwrap(),
            ProfileType::TravelAgent
        );
    }

    fn sample_property(id: Uuid) -> Property {
        Property {
            id,
            env_id: Uuid::new_v4(),
            property_code: "HOTEL01".into(),
            property_name: "Sentin8 Grand".into(),
            timezone: "UTC".into(),
            currency_code: "USD".into(),
            languages: vec!["en".into(), "fr".into()],
            address: Some(Address {
                address_line1: Some("1 Rue de la Paix".into()),
                address_line2: None,
                city: Some("Paris".into()),
                state: None,
                postal_code: Some("75002".into()),
                country: Some("FR".into()),
            }),
            phone: Some("+33100000000".into()),
            email: Some("front.desk@sentin8.example".into()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn property_round_trip_preserves_all_fields() {
        let p = sample_property(Uuid::new_v4());
        let v = serde_json::to_value(&p).unwrap();
        let back: Property = serde_json::from_value(v).unwrap();
        assert_eq!(back.id, p.id);
        assert_eq!(back.property_code, "HOTEL01");
        assert_eq!(back.languages.len(), 2);
        let addr = back.address.expect("address present");
        assert_eq!(addr.city.as_deref(), Some("Paris"));
        assert!(addr.state.is_none(), "absent optional stays None");
    }

    #[test]
    fn property_create_defaults_optionals_to_none() {
        let v = serde_json::json!({
            "property_code": "HOTEL02",
            "property_name": "Sentin8 Plaza"
        });
        let c: PropertyCreate = serde_json::from_value(v).unwrap();
        assert_eq!(c.property_code, "HOTEL02");
        assert_eq!(c.currency_code, None);
        assert_eq!(c.languages, None);
        assert!(c.address.is_none());
    }

    #[test]
    fn room_type_round_trip() {
        let rt = RoomType {
            id: Uuid::new_v4(),
            property_id: Uuid::new_v4(),
            room_type_code: "DELX".into(),
            room_type_name: "Deluxe King".into(),
            description: None,
            max_occupancy: 3,
            bed_type: Some("KING".into()),
            is_smoking: false,
            features: vec!["SEA_VIEW".into()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let back: RoomType = serde_json::from_value(serde_json::to_value(&rt).unwrap()).unwrap();
        assert_eq!(back.room_type_code, "DELX");
        assert_eq!(back.max_occupancy, 3);
        assert_eq!(back.features, vec!["SEA_VIEW".to_string()]);
    }

    #[test]
    fn room_type_create_defaults_features_to_none() {
        let c: RoomTypeCreate = serde_json::from_value(serde_json::json!({
            "room_type_code": "STD",
            "room_type_name": "Standard",
            "max_occupancy": 2,
            "is_smoking": false
        }))
        .unwrap();
        assert_eq!(c.features, None);
        assert!(c.bed_type.is_none());
    }

    #[test]
    fn reservation_round_trip_keeps_status_and_nights() {
        let r = Reservation {
            id: Uuid::new_v4(),
            property_id: Uuid::new_v4(),
            confirmation_number: "RES-20260904000000".into(),
            status: ReservationStatus::CheckedIn,
            arrival_date: chrono::NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
            departure_date: chrono::NaiveDate::from_ymd_opt(2026, 9, 5).unwrap(),
            number_of_nights: 4,
            number_of_rooms: 1,
            number_of_adults: 2,
            number_of_children: 0,
            room_type_id: Uuid::new_v4(),
            room_id: Some(Uuid::new_v4()),
            rate_plan_id: Uuid::new_v4(),
            rate_amount: Decimal::new(189_99, 2),
            currency_code: "EUR".into(),
            guest_profile_id: None,
            company_profile_id: None,
            travel_agent_profile_id: None,
            source_code: Some("WEB".into()),
            market_code: None,
            payment_method: Some("CCG".into()),
            guarantee_code: None,
            credit_card_token: None,
            packages: serde_json::json!([]),
            comments: serde_json::json!([]),
            special_requests: Some("High floor".into()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let back: Reservation = serde_json::from_value(serde_json::to_value(&r).unwrap()).unwrap();
        assert_eq!(back.status, ReservationStatus::CheckedIn);
        assert_eq!(back.number_of_nights, 4);
        assert_eq!(back.rate_amount, Decimal::new(189_99, 2));
    }

    #[test]
    fn guest_profile_round_trip_with_vip() {
        let g = GuestProfile {
            id: Uuid::new_v4(),
            property_id: Uuid::new_v4(),
            profile_type: ProfileType::Guest,
            title: Some("Dr".into()),
            first_name: Some("Ada".into()),
            middle_name: None,
            last_name: Some("Lovelace".into()),
            preferred_name: None,
            gender: None,
            date_of_birth: Some(chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()),
            nationality: Some("GB".into()),
            primary_language: None,
            email: Some("ada@example.test".into()),
            phone: None,
            mobile_phone: None,
            address: None,
            passport_number: Some("123456789".into()),
            passport_country: Some("GB".into()),
            passport_expiry_date: None,
            vip_status: VipStatus::Vip,
            communication_consent: true,
            preferences: serde_json::json!({"floor": "high"}),
            loyalty_programs: serde_json::json!([]),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let back: GuestProfile = serde_json::from_value(serde_json::to_value(&g).unwrap()).unwrap();
        assert_eq!(back.vip_status, VipStatus::Vip);
        assert_eq!(back.profile_type, ProfileType::Guest);
        assert!(back.communication_consent);
        assert_eq!(back.preferences["floor"], "high");
    }

    #[test]
    fn folio_charge_decimal_survives_json_round_trip() {
        let charge = FolioCharge {
            id: Uuid::new_v4(),
            transaction_code: "ROOM".into(),
            description: "Night 1".into(),
            amount: Decimal::new(1, 2), // 0.01 - the smallest cent
            quantity: 1,
            posted_at: chrono::Utc::now(),
            posted_by: None,
        };
        let back: FolioCharge =
            serde_json::from_value(serde_json::to_value(&charge).unwrap()).unwrap();
        assert_eq!(
            back.amount,
            Decimal::new(1, 2),
            "no float truncation on 0.01"
        );
        assert_eq!(back.transaction_code, "ROOM");
    }

    #[test]
    fn folio_with_charges_and_payments_round_trip() {
        let now = chrono::Utc::now();
        let f = Folio {
            id: Uuid::new_v4(),
            reservation_id: Uuid::new_v4(),
            folio_number: Some("F-0001".into()),
            status: FolioStatus::Open,
            balance: Decimal::new(25_00, 2),
            currency_code: "USD".into(),
            charges: vec![FolioCharge {
                id: Uuid::new_v4(),
                transaction_code: "REST".into(),
                description: "Dinner".into(),
                amount: Decimal::new(75_00, 2),
                quantity: 2,
                posted_at: now,
                posted_by: Some("mgr".into()),
            }],
            payments: vec![FolioPayment {
                id: Uuid::new_v4(),
                payment_method: "CASH".into(),
                amount: Decimal::new(50_00, 2),
                reference_number: None,
                posted_at: now,
                status: "POSTED".into(),
            }],
            taxes: vec![FolioTax {
                tax_code: "VAT".into(),
                tax_description: "Value Added Tax".into(),
                amount: Decimal::new(8_00, 2),
                taxable_amount: Decimal::new(50_00, 2),
            }],
        };
        let back: Folio = serde_json::from_value(serde_json::to_value(&f).unwrap()).unwrap();
        assert_eq!(back.charges.len(), 1);
        assert_eq!(back.payments.len(), 1);
        assert_eq!(back.taxes[0].taxable_amount, Decimal::new(50_00, 2));
        assert_eq!(back.status, FolioStatus::Open);
    }

    #[test]
    fn inventory_and_sell_limit_round_trip() {
        let inv = Inventory {
            property_id: Uuid::new_v4(),
            room_type_id: Uuid::new_v4(),
            date: chrono::NaiveDate::from_ymd_opt(2026, 9, 10).unwrap(),
            physical_rooms: 100,
            sell_limit: 95,
            sold_rooms: 20,
            available_rooms: 75,
            out_of_order: 3,
            out_of_service: 2,
        };
        let back: Inventory = serde_json::from_value(serde_json::to_value(&inv).unwrap()).unwrap();
        assert_eq!(back.available_rooms, 75);
        assert_eq!(back.out_of_order, 3);
    }

    #[test]
    fn availability_request_defaults_room_count_to_none() {
        let r: AvailabilityRequest = serde_json::from_value(serde_json::json!({
            "arrival_date": "2026-09-01",
            "departure_date": "2026-09-05",
            "number_of_adults": 2
        }))
        .unwrap();
        assert_eq!(r.number_of_rooms, None);
        assert_eq!(r.promo_code, None);
    }

    #[test]
    fn availability_response_with_rate_plans_round_trip() {
        let resp = AvailabilityResponse {
            property_code: "HOTEL01".into(),
            env_id: Uuid::new_v4(),
            arrival_date: chrono::NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
            departure_date: chrono::NaiveDate::from_ymd_opt(2026, 9, 5).unwrap(),
            number_of_nights: 4,
            room_types: vec![AvailableRoomType {
                room_type: "DELX".into(),
                room_type_description: "Deluxe King".into(),
                available_rooms: 3,
                total_rooms: 10,
                rate_plans: vec![AvailableRatePlan {
                    rate_plan_code: "BAR".into(),
                    rate_plan_name: "Best Available".into(),
                    total_amount: Decimal::new(799_96, 2),
                    average_nightly_rate: Decimal::new(199_99, 2),
                    currency_code: "EUR".into(),
                    is_available: true,
                    restrictions: Some(RateRestrictions {
                        closed_to_arrival: false,
                        closed_to_departure: true,
                        min_los: 1,
                        max_los: 30,
                    }),
                }],
            }],
        };
        let back: AvailabilityResponse =
            serde_json::from_value(serde_json::to_value(&resp).unwrap()).unwrap();
        let rt = &back.room_types[0];
        let rp = &rt.rate_plans[0];
        assert_eq!(rp.rate_plan_code, "BAR");
        assert_eq!(rp.total_amount, Decimal::new(799_96, 2));
        let restrictions = rp.restrictions.as_ref().unwrap();
        assert!(restrictions.closed_to_departure);
        assert_eq!(restrictions.min_los, 1);
    }

    #[test]
    fn lov_entry_round_trip() {
        let lov = LovEntry {
            code: "MR".into(),
            description: "Mr.".into(),
            display_sequence: 1,
            is_active: true,
        };
        let back: LovEntry = serde_json::from_value(serde_json::to_value(&lov).unwrap()).unwrap();
        assert_eq!(back.code, "MR");
        assert!(back.is_active);
    }

    #[test]
    fn health_response_round_trip() {
        let h = HealthResponse {
            status: "ok".into(),
            service: "pms".into(),
            version: "0.1.0".into(),
            build: "deadbeef".into(),
            database: "connected".into(),
        };
        let back: HealthResponse =
            serde_json::from_value(serde_json::to_value(&h).unwrap()).unwrap();
        assert_eq!(back.build, "deadbeef");
        assert_eq!(back.database, "connected");
    }
}

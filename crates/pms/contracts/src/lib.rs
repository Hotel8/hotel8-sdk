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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomTypeCreate {
    pub room_type_code: String,
    pub room_type_name: String,
    pub description: Option<String>,
    pub max_occupancy: i32,
    pub bed_type: Option<String>,
    pub is_smoking: bool,
    pub features: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

// ═══════════════════════════════════════════════════════════════════════════
// Health
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub build: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests — contract round-trips + wire format (H8T-2)
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
            amount: Decimal::new(1, 2), // 0.01 — the smallest cent
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
        };
        let back: HealthResponse =
            serde_json::from_value(serde_json::to_value(&h).unwrap()).unwrap();
        assert_eq!(back.build, "deadbeef");
    }
}

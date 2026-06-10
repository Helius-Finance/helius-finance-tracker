//! Snapshot tests for [`helius::AppError`] structured variant `Display` output.
//!
//! These tests are the regression gate for Phase 4 (structured errors).  They
//! assert that every new typed variant produces a string that is byte-identical
//! to what the previous string-carrying helpers (`invalid_ref`, `duplicate`,
//! `AppError::Validation(format!(...))`) produced before the refactor.
//!
//! If a display string is ever changed for a good reason, update the expected
//! literal here intentionally — that documents the decision.

use helius::error::{AppError, EntityKind};

// ── NotFoundEntity ────────────────────────────────────────────────────────────

#[test]
fn not_found_account() {
    let err = AppError::invalid_ref(EntityKind::Account, "Checking");
    assert_eq!(err.to_string(), "account `Checking` was not found");
}

#[test]
fn not_found_category() {
    let err = AppError::invalid_ref(EntityKind::Category, "Food");
    assert_eq!(err.to_string(), "category `Food` was not found");
}

#[test]
fn not_found_scenario() {
    let err = AppError::invalid_ref(EntityKind::Scenario, "base");
    assert_eq!(err.to_string(), "scenario `base` was not found");
}

#[test]
fn not_found_goal() {
    let err = AppError::NotFoundEntity {
        kind: EntityKind::Goal,
        value: "Emergency fund".to_string(),
    };
    assert_eq!(err.to_string(), "goal `Emergency fund` was not found");
}

#[test]
fn not_found_recurring_rule() {
    let err = AppError::NotFoundEntity {
        kind: EntityKind::RecurringRule,
        value: "salary".to_string(),
    };
    assert_eq!(err.to_string(), "recurring rule `salary` was not found");
}

#[test]
fn not_found_planning_item() {
    let err = AppError::NotFoundEntity {
        kind: EntityKind::PlanningItem,
        value: "42".to_string(),
    };
    assert_eq!(err.to_string(), "planning item `42` was not found");
}

#[test]
fn not_found_transaction() {
    let err = AppError::NotFoundEntity {
        kind: EntityKind::Transaction,
        value: "99".to_string(),
    };
    assert_eq!(err.to_string(), "transaction `99` was not found");
}

// ── DuplicateEntity ───────────────────────────────────────────────────────────

#[test]
fn duplicate_account() {
    let err = AppError::duplicate(EntityKind::Account, "Savings");
    assert_eq!(err.to_string(), "account `Savings` already exists");
}

#[test]
fn duplicate_category() {
    let err = AppError::duplicate(EntityKind::Category, "Travel");
    assert_eq!(err.to_string(), "category `Travel` already exists");
}

#[test]
fn duplicate_scenario() {
    let err = AppError::duplicate(EntityKind::Scenario, "optimistic");
    assert_eq!(err.to_string(), "scenario `optimistic` already exists");
}

#[test]
fn duplicate_goal() {
    let err = AppError::duplicate(EntityKind::Goal, "Vacation fund");
    assert_eq!(err.to_string(), "goal `Vacation fund` already exists");
}

#[test]
fn duplicate_recurring_rule() {
    let err = AppError::duplicate(EntityKind::RecurringRule, "rent");
    assert_eq!(err.to_string(), "recurring rule `rent` already exists");
}

// ── FieldValidation ───────────────────────────────────────────────────────────
// Each assertion mirrors a message that `amount.rs` previously produced as
// `AppError::Validation("<field> <reason>".to_string())`.

#[test]
fn field_validation_amount_must_be_positive() {
    let err = AppError::field_validation("amount", "must be positive");
    assert_eq!(err.to_string(), "amount must be positive");
}

#[test]
fn field_validation_amount_cannot_be_zero() {
    let err = AppError::field_validation("amount", "cannot be zero");
    assert_eq!(err.to_string(), "amount cannot be zero");
}

#[test]
fn field_validation_amount_cannot_be_empty() {
    let err = AppError::field_validation("amount", "cannot be empty");
    assert_eq!(err.to_string(), "amount cannot be empty");
}

#[test]
fn field_validation_amount_too_large() {
    let err = AppError::field_validation("amount", "is too large to fit in 64-bit cents");
    assert_eq!(
        err.to_string(),
        "amount is too large to fit in 64-bit cents"
    );
}

#[test]
fn field_validation_amount_decimal_places() {
    let err = AppError::field_validation("amount", "can use at most two decimal places");
    assert_eq!(err.to_string(), "amount can use at most two decimal places");
}

#[test]
fn field_validation_amount_decimal_point() {
    let err = AppError::field_validation("amount", "can contain at most one decimal point");
    assert_eq!(
        err.to_string(),
        "amount can contain at most one decimal point"
    );
}

#[test]
fn field_validation_amount_digits_only() {
    let err = AppError::field_validation(
        "amount",
        "must contain only digits and an optional decimal point",
    );
    assert_eq!(
        err.to_string(),
        "amount must contain only digits and an optional decimal point"
    );
}

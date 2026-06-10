//! Account service: wraps the account-domain methods of [`crate::db::Db`]
//! behind typed request structs.
//!
//! Frontends build a request, call the matching service method, and render
//! the response. Orchestration that is common to every frontend (e.g. the
//! "edit requires at least one field change" rule) lives here so CLI and
//! TUI cannot drift.

use crate::db::Db;
use crate::error::AppError;
use crate::model::{Account, AccountKind};

/// Request for [`AccountService::add`].
#[derive(Clone, Debug)]
pub struct AddAccountRequest {
    pub name: String,
    pub kind: AccountKind,
    pub opening_balance_cents: i64,
    pub opened_on: String,
}

/// Request for [`AccountService::edit`]. Any field left `None` keeps its
/// current value. At least one field must be `Some` — the service enforces
/// that rule uniformly across frontends.
#[derive(Clone, Debug, Default)]
pub struct EditAccountRequest {
    pub reference: String,
    pub name: Option<String>,
    pub kind: Option<AccountKind>,
    pub opening_balance_cents: Option<i64>,
    pub opened_on: Option<String>,
}

/// Request for [`AccountService::delete`].
#[derive(Clone, Debug)]
pub struct DeleteAccountRequest {
    pub reference: String,
}

/// Request for [`AccountService::list`]. Currently empty; kept as a struct
/// so future filters (include archived, filter by kind, …) can be added
/// without breaking callers.
#[derive(Clone, Debug, Default)]
pub struct ListAccountsRequest;

/// Service facade for the account domain.
pub struct AccountService<'a> {
    db: &'a Db,
}

impl<'a> AccountService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    /// Inserts a new account. Returns the freshly assigned row id so the
    /// caller can reference it in status messages or follow-up queries.
    pub fn add(&self, req: AddAccountRequest) -> Result<i64, AppError> {
        self.db.add_account(
            &req.name,
            &req.kind,
            req.opening_balance_cents,
            &req.opened_on,
        )
    }

    /// Updates an existing account. Requires at least one field to change;
    /// frontends should surface the error message verbatim.
    pub fn edit(&self, req: EditAccountRequest) -> Result<i64, AppError> {
        if req.name.is_none()
            && req.kind.is_none()
            && req.opening_balance_cents.is_none()
            && req.opened_on.is_none()
        {
            return Err(AppError::Validation(
                "account edit requires at least one field change".to_string(),
            ));
        }
        self.db.edit_account(
            &req.reference,
            req.name.as_deref(),
            req.kind.as_ref(),
            req.opening_balance_cents,
            req.opened_on.as_deref(),
        )
    }

    /// Archives an account (soft-delete). Returns the row id of the
    /// archived account for status messaging.
    pub fn delete(&self, req: DeleteAccountRequest) -> Result<i64, AppError> {
        self.db.delete_account(&req.reference)
    }

    /// Lists all non-archived accounts, ordered by name (case-insensitive).
    pub fn list(&self, _req: ListAccountsRequest) -> Result<Vec<Account>, AppError> {
        self.db.list_accounts()
    }
}

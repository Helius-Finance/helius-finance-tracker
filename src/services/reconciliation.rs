//! Reconciliation service: wraps the reconciliation-domain methods of
//! [`crate::db::Db`].

use crate::db::Db;
use crate::error::AppError;
use crate::model::ReconciliationRecord;

/// Service facade for the reconciliation domain.
pub struct ReconciliationService<'a> {
    db: &'a Db,
}

impl<'a> ReconciliationService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    /// Creates a new reconciliation for an account. Returns the row id.
    pub fn start(
        &self,
        account: &str,
        statement_ending_on: &str,
        statement_balance_cents: i64,
        transaction_ids: &[i64],
    ) -> Result<i64, AppError> {
        self.db.start_reconciliation(
            account,
            statement_ending_on,
            statement_balance_cents,
            transaction_ids,
        )
    }

    /// Lists reconciliations, optionally filtered to a single account.
    pub fn list(&self, account: Option<&str>) -> Result<Vec<ReconciliationRecord>, AppError> {
        self.db.list_reconciliations(account)
    }

    /// Removes a reconciliation by id.
    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        self.db.delete_reconciliation(id)
    }
}

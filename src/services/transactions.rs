//! Transaction service: wraps the transaction-domain methods of
//! [`crate::db::Db`] behind a consistent facade.
//!
//! The request types for this service are the existing model structs
//! (`NewTransaction`, `UpdateTransaction`, `TransactionFilters`) because they
//! already carry exactly the right shape. Creating parallel request structs
//! would add noise without benefit.

use crate::db::Db;
use crate::error::AppError;
use crate::model::{NewTransaction, TransactionFilters, TransactionRecord, UpdateTransaction};

/// Service facade for the transaction domain.
pub struct TransactionService<'a> {
    db: &'a Db,
}

impl<'a> TransactionService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    /// Inserts a new transaction. Returns the freshly assigned row id.
    pub fn add(&self, transaction: &NewTransaction) -> Result<i64, AppError> {
        self.db.add_transaction(transaction)
    }

    /// Applies a partial update to an existing transaction.
    pub fn edit(&self, patch: &UpdateTransaction) -> Result<(), AppError> {
        self.db.edit_transaction(patch)
    }

    /// Soft-deletes a transaction by id.
    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        self.db.delete_transaction(id)
    }

    /// Restores a previously soft-deleted transaction.
    pub fn restore(&self, id: i64) -> Result<(), AppError> {
        self.db.restore_transaction(id)
    }

    /// Lists transactions matching the given filters.
    pub fn list(&self, filters: &TransactionFilters) -> Result<Vec<TransactionRecord>, AppError> {
        self.db.list_transactions(filters)
    }

    /// Returns up to `limit` most recent non-deleted transactions.
    pub fn recent(&self, limit: usize) -> Result<Vec<TransactionRecord>, AppError> {
        self.db.recent_transactions(limit)
    }
}

use crate::db::Db;
use crate::error::AppError;
use crate::model::{NewTransaction, TransactionFilters, TransactionRecord, UpdateTransaction};

pub struct TransactionService<'a> {
    db: &'a Db,
}

impl<'a> TransactionService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn add(&self, transaction: &NewTransaction) -> Result<i64, AppError> {
        self.db.add_transaction(transaction)
    }

    pub fn edit(&self, patch: &UpdateTransaction) -> Result<(), AppError> {
        self.db.edit_transaction(patch)
    }

    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        self.db.delete_transaction(id)
    }

    pub fn restore(&self, id: i64) -> Result<(), AppError> {
        self.db.restore_transaction(id)
    }

    pub fn list(&self, filters: &TransactionFilters) -> Result<Vec<TransactionRecord>, AppError> {
        self.db.list_transactions(filters)
    }

    pub fn recent(&self, limit: usize) -> Result<Vec<TransactionRecord>, AppError> {
        self.db.recent_transactions(limit)
    }
}

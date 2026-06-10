use crate::db::Db;
use crate::error::AppError;
use crate::model::ReconciliationRecord;

pub struct ReconciliationService<'a> {
    db: &'a Db,
}

impl<'a> ReconciliationService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

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

    pub fn list(&self, account: Option<&str>) -> Result<Vec<ReconciliationRecord>, AppError> {
        self.db.list_reconciliations(account)
    }

    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        self.db.delete_reconciliation(id)
    }
}

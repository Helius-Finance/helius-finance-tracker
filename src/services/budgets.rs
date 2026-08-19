use crate::db::Db;
use crate::error::AppError;
use crate::model::{BudgetRecord, BudgetStatusRecord};

pub struct BudgetService<'a> {
    db: &'a Db,
}

impl<'a> BudgetService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn set(
        &self,
        month: &str,
        category: &str,
        amount_cents: i64,
        account: Option<&str>,
        scenario: Option<&str>,
    ) -> Result<i64, AppError> {
        self.db
            .set_budget(month, category, amount_cents, account, scenario)
    }

    pub fn delete(
        &self,
        month: &str,
        category: &str,
        account: Option<&str>,
        scenario: Option<&str>,
    ) -> Result<(), AppError> {
        self.db
            .delete_budget(month, category, account, scenario)
    }

    pub fn list(
        &self,
        month: Option<&str>,
        scenario: Option<&str>,
    ) -> Result<Vec<BudgetRecord>, AppError> {
        self.db.list_budgets(month, scenario)
    }

    pub fn status(
        &self,
        month: &str,
        scenario: Option<&str>,
    ) -> Result<Vec<BudgetStatusRecord>, AppError> {
        self.db.budget_status(month, scenario)
    }
}

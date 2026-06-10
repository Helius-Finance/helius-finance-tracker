//! Budget service: wraps the budget-domain methods of [`crate::db::Db`].

use crate::db::Db;
use crate::error::AppError;
use crate::model::{BudgetRecord, BudgetStatusRecord};

/// Service facade for the budget domain.
pub struct BudgetService<'a> {
    db: &'a Db,
}

impl<'a> BudgetService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    /// Creates or replaces a budget entry. Returns the row id.
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

    /// Deletes a budget entry for the given month / category / scenario
    /// combination.
    pub fn delete(
        &self,
        month: &str,
        category: &str,
        scenario: Option<&str>,
    ) -> Result<(), AppError> {
        self.db.delete_budget(month, category, scenario)
    }

    /// Lists budget entries, optionally filtered by month and/or scenario.
    pub fn list(
        &self,
        month: Option<&str>,
        scenario: Option<&str>,
    ) -> Result<Vec<BudgetRecord>, AppError> {
        self.db.list_budgets(month, scenario)
    }

    /// Returns the spend-vs-budget status for the given month.
    pub fn status(
        &self,
        month: &str,
        scenario: Option<&str>,
    ) -> Result<Vec<BudgetStatusRecord>, AppError> {
        self.db.budget_status(month, scenario)
    }
}

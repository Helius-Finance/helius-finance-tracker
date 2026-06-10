//! Reporting service: wraps the read-only reporting and forecast methods
//! of [`crate::db::Db`].

use crate::db::Db;
use crate::error::AppError;
use crate::model::{
    BalanceRecord, BalanceTrendPoint, CategorySpendingPoint, ForecastSnapshot,
    MonthlyCashFlowPoint, SummaryRecord, WeeklyBalancePoint,
};

/// Service facade for the reporting domain.
pub struct ReportingService<'a> {
    db: &'a Db,
}

impl<'a> ReportingService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    /// Returns current balances, optionally filtered to a single account.
    pub fn balances(&self, account_ref: Option<&str>) -> Result<Vec<BalanceRecord>, AppError> {
        self.db.balances(account_ref)
    }

    /// Summarises income/expense for a date range, optionally per account.
    pub fn summary(
        &self,
        from: &str,
        to: &str,
        account_ref: Option<&str>,
    ) -> Result<SummaryRecord, AppError> {
        self.db.summary(from, to, account_ref)
    }

    /// Returns monthly income/expense trend for the last `months` months.
    pub fn monthly_cash_flow_trend(
        &self,
        months: usize,
    ) -> Result<Vec<MonthlyCashFlowPoint>, AppError> {
        self.db.monthly_cash_flow_trend(months)
    }

    /// Returns top-N expense categories for a date range.
    pub fn category_spending(
        &self,
        from: &str,
        to: &str,
        limit: usize,
    ) -> Result<Vec<CategorySpendingPoint>, AppError> {
        self.db.category_spending(from, to, limit)
    }

    /// Returns total balance trend over the last `months` months.
    pub fn total_balance_trend(&self, months: usize) -> Result<Vec<BalanceTrendPoint>, AppError> {
        self.db.total_balance_trend(months)
    }

    /// Returns weekly opening balance history for the last `weeks` weeks.
    pub fn weekly_opening_balance_history(
        &self,
        weeks: usize,
    ) -> Result<Vec<WeeklyBalancePoint>, AppError> {
        self.db.weekly_opening_balance_history(weeks)
    }

    /// Runs the cash-flow forecast for up to `days` days, optionally under
    /// a named scenario and/or filtered to a single account.
    pub fn forecast(
        &self,
        scenario: Option<&str>,
        account_ref: Option<&str>,
        days: usize,
    ) -> Result<ForecastSnapshot, AppError> {
        self.db.forecast(scenario, account_ref, days)
    }
}

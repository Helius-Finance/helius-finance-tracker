use crate::db::Db;
use crate::error::AppError;
use crate::model::{
    BalanceRecord, BalanceTrendPoint, CategorySpendingPoint, ForecastSnapshot,
    MonthlyCashFlowPoint, SummaryRecord, WeeklyBalancePoint,
};

pub struct ReportingService<'a> {
    db: &'a Db,
}

impl<'a> ReportingService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn balances(&self, account_ref: Option<&str>) -> Result<Vec<BalanceRecord>, AppError> {
        self.db.balances(account_ref)
    }

    pub fn summary(
        &self,
        from: &str,
        to: &str,
        account_ref: Option<&str>,
    ) -> Result<SummaryRecord, AppError> {
        self.db.summary(from, to, account_ref)
    }

    pub fn monthly_cash_flow_trend(
        &self,
        months: usize,
    ) -> Result<Vec<MonthlyCashFlowPoint>, AppError> {
        self.db.monthly_cash_flow_trend(months)
    }

    pub fn category_spending(
        &self,
        from: &str,
        to: &str,
        limit: usize,
    ) -> Result<Vec<CategorySpendingPoint>, AppError> {
        self.db.category_spending(from, to, limit)
    }

    pub fn total_balance_trend(&self, months: usize) -> Result<Vec<BalanceTrendPoint>, AppError> {
        self.db.total_balance_trend(months)
    }

    pub fn weekly_opening_balance_history(
        &self,
        weeks: usize,
    ) -> Result<Vec<WeeklyBalancePoint>, AppError> {
        self.db.weekly_opening_balance_history(weeks)
    }

    pub fn forecast(
        &self,
        scenario: Option<&str>,
        account_ref: Option<&str>,
        days: usize,
    ) -> Result<ForecastSnapshot, AppError> {
        self.db.forecast(scenario, account_ref, days)
    }
}

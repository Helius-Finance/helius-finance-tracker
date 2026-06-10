//! Recurring-rule service: wraps the recurring-rule domain methods of
//! [`crate::db::Db`].

use crate::db::Db;
use crate::error::AppError;
use crate::model::{
    NewRecurringRule, RecurringOccurrenceRecord, RecurringRuleRecord, UpdateRecurringRule,
};

/// Service facade for the recurring-rule domain.
pub struct RecurringService<'a> {
    db: &'a Db,
}

impl<'a> RecurringService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    /// Creates a new recurring rule. Returns the row id.
    pub fn add(&self, rule: &NewRecurringRule) -> Result<i64, AppError> {
        self.db.add_recurring_rule(rule)
    }

    /// Applies a partial update to an existing recurring rule.
    pub fn edit(&self, patch: &UpdateRecurringRule) -> Result<(), AppError> {
        self.db.edit_recurring_rule(patch)
    }

    /// Lists all non-deleted recurring rules.
    pub fn list(&self) -> Result<Vec<RecurringRuleRecord>, AppError> {
        self.db.list_recurring_rules()
    }

    /// Pauses a recurring rule so it is skipped on the next run.
    pub fn pause(&self, id: i64) -> Result<(), AppError> {
        self.db.pause_recurring_rule(id)
    }

    /// Resumes a previously paused recurring rule.
    pub fn resume(&self, id: i64) -> Result<(), AppError> {
        self.db.resume_recurring_rule(id)
    }

    /// Permanently deletes a recurring rule.
    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        self.db.delete_recurring_rule(id)
    }

    /// Posts all due recurring rules as transactions up to and including
    /// `through`. Returns the number of transactions posted.
    pub fn run_due(&self, through: &str) -> Result<usize, AppError> {
        self.db.run_due_recurring(through)
    }

    /// Lists upcoming/overdue occurrences up to and including `through`.
    pub fn list_due(&self, through: &str) -> Result<Vec<RecurringOccurrenceRecord>, AppError> {
        self.db.list_due_occurrences(through)
    }
}

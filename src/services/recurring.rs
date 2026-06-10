use crate::db::Db;
use crate::error::AppError;
use crate::model::{
    NewRecurringRule, RecurringOccurrenceRecord, RecurringRuleRecord, UpdateRecurringRule,
};

pub struct RecurringService<'a> {
    db: &'a Db,
}

impl<'a> RecurringService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn add(&self, rule: &NewRecurringRule) -> Result<i64, AppError> {
        self.db.add_recurring_rule(rule)
    }

    pub fn edit(&self, patch: &UpdateRecurringRule) -> Result<(), AppError> {
        self.db.edit_recurring_rule(patch)
    }

    pub fn list(&self) -> Result<Vec<RecurringRuleRecord>, AppError> {
        self.db.list_recurring_rules()
    }

    pub fn pause(&self, id: i64) -> Result<(), AppError> {
        self.db.pause_recurring_rule(id)
    }

    pub fn resume(&self, id: i64) -> Result<(), AppError> {
        self.db.resume_recurring_rule(id)
    }

    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        self.db.delete_recurring_rule(id)
    }

    pub fn run_due(&self, through: &str) -> Result<usize, AppError> {
        self.db.run_due_recurring(through)
    }

    pub fn list_due(&self, through: &str) -> Result<Vec<RecurringOccurrenceRecord>, AppError> {
        self.db.list_due_occurrences(through)
    }
}

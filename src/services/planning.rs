//! Planning service: wraps the planning domain methods of [`crate::db::Db`].
//!
//! The planning domain covers three entity types that are closely related:
//! - **Items**: one-off planned transactions attached to a scenario
//! - **Scenarios**: named forecast configurations
//! - **Goals**: savings / balance targets
//!
//! All three are exposed through a single `PlanningService` struct. If any
//! sub-domain grows large enough to warrant its own file, split then.

use crate::db::Db;
use crate::error::AppError;
use crate::model::{
    NewPlanningGoal, NewPlanningItem, NewPlanningScenario, PlanningGoalRecord, PlanningItemRecord,
    PlanningScenarioRecord, UpdatePlanningGoal, UpdatePlanningItem, UpdatePlanningScenario,
};

/// Service facade for the planning domain.
pub struct PlanningService<'a> {
    db: &'a Db,
}

impl<'a> PlanningService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    // ── Planning items ────────────────────────────────────────────────────────

    /// Creates a new planning item. Returns the row id.
    pub fn add_item(&self, item: &NewPlanningItem) -> Result<i64, AppError> {
        self.db.add_planning_item(item)
    }

    /// Applies a partial update to an existing planning item.
    pub fn edit_item(&self, patch: &UpdatePlanningItem) -> Result<(), AppError> {
        self.db.edit_planning_item(patch)
    }

    /// Lists planning items, optionally filtered by scenario and/or date range.
    pub fn list_items(
        &self,
        scenario: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<PlanningItemRecord>, AppError> {
        self.db.list_planning_items(scenario, from, to)
    }

    /// Archives (soft-deletes) a planning item by id.
    pub fn delete_item(&self, id: i64) -> Result<(), AppError> {
        self.db.delete_planning_item(id)
    }

    /// Posts a planning item as a real transaction. Returns the new
    /// transaction id.
    pub fn post_item(&self, id: i64) -> Result<i64, AppError> {
        self.db.post_planning_item(id)
    }

    // ── Scenarios ─────────────────────────────────────────────────────────────

    /// Creates a new planning scenario. Returns the row id.
    pub fn add_scenario(&self, scenario: &NewPlanningScenario) -> Result<i64, AppError> {
        self.db.add_planning_scenario(scenario)
    }

    /// Lists all non-archived scenarios.
    pub fn list_scenarios(&self) -> Result<Vec<PlanningScenarioRecord>, AppError> {
        self.db.list_planning_scenarios()
    }

    /// Applies a partial update to an existing scenario.
    pub fn edit_scenario(&self, patch: &UpdatePlanningScenario) -> Result<(), AppError> {
        self.db.edit_planning_scenario(patch)
    }

    /// Archives a scenario by id.
    pub fn delete_scenario(&self, id: i64) -> Result<(), AppError> {
        self.db.delete_planning_scenario(id)
    }

    // ── Goals ─────────────────────────────────────────────────────────────────

    /// Creates a new planning goal. Returns the row id.
    pub fn add_goal(&self, goal: &NewPlanningGoal) -> Result<i64, AppError> {
        self.db.add_planning_goal(goal)
    }

    /// Lists all non-archived goals.
    pub fn list_goals(&self) -> Result<Vec<PlanningGoalRecord>, AppError> {
        self.db.list_planning_goals()
    }

    /// Applies a partial update to an existing goal.
    pub fn edit_goal(&self, patch: &UpdatePlanningGoal) -> Result<(), AppError> {
        self.db.edit_planning_goal(patch)
    }

    /// Archives a goal by id.
    pub fn delete_goal(&self, id: i64) -> Result<(), AppError> {
        self.db.delete_planning_goal(id)
    }
}

use crate::db::Db;
use crate::error::AppError;
use crate::model::{
    NewPlanningGoal, NewPlanningItem, NewPlanningScenario, PlanningGoalRecord, PlanningItemRecord,
    PlanningScenarioRecord, UpdatePlanningGoal, UpdatePlanningItem, UpdatePlanningScenario,
};

pub struct PlanningService<'a> {
    db: &'a Db,
}

impl<'a> PlanningService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn add_item(&self, item: &NewPlanningItem) -> Result<i64, AppError> {
        self.db.add_planning_item(item)
    }

    pub fn edit_item(&self, patch: &UpdatePlanningItem) -> Result<(), AppError> {
        self.db.edit_planning_item(patch)
    }

    pub fn list_items(
        &self,
        scenario: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Vec<PlanningItemRecord>, AppError> {
        self.db.list_planning_items(scenario, from, to)
    }

    pub fn delete_item(&self, id: i64) -> Result<(), AppError> {
        self.db.delete_planning_item(id)
    }

    pub fn post_item(&self, id: i64) -> Result<i64, AppError> {
        self.db.post_planning_item(id)
    }

    pub fn add_scenario(&self, scenario: &NewPlanningScenario) -> Result<i64, AppError> {
        self.db.add_planning_scenario(scenario)
    }

    pub fn list_scenarios(&self) -> Result<Vec<PlanningScenarioRecord>, AppError> {
        self.db.list_planning_scenarios()
    }

    pub fn edit_scenario(&self, patch: &UpdatePlanningScenario) -> Result<(), AppError> {
        self.db.edit_planning_scenario(patch)
    }

    pub fn delete_scenario(&self, id: i64) -> Result<(), AppError> {
        self.db.delete_planning_scenario(id)
    }

    pub fn add_goal(&self, goal: &NewPlanningGoal) -> Result<i64, AppError> {
        self.db.add_planning_goal(goal)
    }

    pub fn list_goals(&self) -> Result<Vec<PlanningGoalRecord>, AppError> {
        self.db.list_planning_goals()
    }

    pub fn edit_goal(&self, patch: &UpdatePlanningGoal) -> Result<(), AppError> {
        self.db.edit_planning_goal(patch)
    }

    pub fn delete_goal(&self, id: i64) -> Result<(), AppError> {
        self.db.delete_planning_goal(id)
    }
}

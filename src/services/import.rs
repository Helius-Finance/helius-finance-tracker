//! Import service: shared orchestration for CSV and CAMT053 imports.
//!
//! Both the CLI (`helius import csv|camt053`) and the TUI import wizard
//! used to build `CsvImportRequest`/`Camt053ImportRequest` independently
//! and then call `Db::import_transactions` directly. That meant the same
//! "request → resolve plan → import (dry-run) → import (real)" chain was
//! re-implemented in two places. This module consolidates the chain into
//! a single API both frontends call.
//!
//! Three entry points:
//!
//! * [`ImportService::execute`] — one-shot import. Honours the
//!   `dry_run` flag carried by the request. The CLI uses this.
//! * [`ImportService::preview`] — first half of a two-step flow. Forces
//!   `dry_run = true`, returns the resolved plan + the preview rows. The
//!   TUI calls this to populate its review screen.
//! * [`ImportService::commit`] — second half of the two-step flow.
//!   Takes the [`ImportPreview`] returned by `preview`, flips the plan
//!   to `dry_run = false`, and runs the real import.
//!
//! The plan resolvers (`resolve_csv_import_plan`,
//! `resolve_camt053_import_plan`) stay in [`crate::importer`] because
//! they're parsing/validation logic owned by the import domain. This
//! module re-exports the request types so callers can `use
//! helius::services::import::{ImportRequest, CsvImportRequest, ...}`
//! without reaching into `crate::importer` directly.

use crate::db::Db;
use crate::error::AppError;
use crate::importer::{resolve_camt053_import_plan, resolve_csv_import_plan};
use crate::model::{ImportPlan, ImportResult};

pub use crate::importer::{Camt053ImportRequest, CsvImportRequest};

/// Top-level request union. Matches the two import sources Helius
/// supports today; adding a new format is one new variant + one new
/// `ImportPlan` variant in `model.rs`.
#[derive(Clone, Debug)]
pub enum ImportRequest {
    Csv(Box<CsvImportRequest>),
    Camt053(Camt053ImportRequest),
}

/// Output of [`ImportService::preview`] and input to
/// [`ImportService::commit`]. Bundles the resolved plan (so commit
/// doesn't have to re-resolve) with the dry-run preview rows the
/// frontend rendered to the user.
#[derive(Clone, Debug)]
pub struct ImportPreview {
    pub plan: ImportPlan,
    pub result: ImportResult,
}

/// Service facade for the import domain.
pub struct ImportService<'a> {
    db: &'a Db,
}

impl<'a> ImportService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    /// One-shot import. Honours `req`'s own `dry_run` flag verbatim,
    /// matching the CLI's behaviour.
    pub fn execute(&self, req: ImportRequest) -> Result<ImportResult, AppError> {
        let plan = resolve_plan(req)?;
        self.db.import_transactions(&plan)
    }

    /// Two-step step 1. Always forces `dry_run = true` regardless of
    /// what the request carried, so callers cannot accidentally commit
    /// during the preview phase. Returns the plan so the matching
    /// [`commit`](Self::commit) call can re-use it without re-parsing.
    pub fn preview(&self, req: ImportRequest) -> Result<ImportPreview, AppError> {
        let mut plan = resolve_plan(req)?;
        plan.set_dry_run(true);
        let result = self.db.import_transactions(&plan)?;
        Ok(ImportPreview { plan, result })
    }

    /// Two-step step 2. Flips the stored plan to `dry_run = false` and
    /// runs the real import. The caller is responsible for surfacing
    /// the original preview to the user before invoking this.
    pub fn commit(&self, preview: ImportPreview) -> Result<ImportResult, AppError> {
        let mut plan = preview.plan;
        plan.set_dry_run(false);
        self.db.import_transactions(&plan)
    }
}

fn resolve_plan(req: ImportRequest) -> Result<ImportPlan, AppError> {
    match req {
        ImportRequest::Csv(csv) => {
            resolve_csv_import_plan(*csv).map(|plan| ImportPlan::Csv(Box::new(plan)))
        }
        ImportRequest::Camt053(camt) => resolve_camt053_import_plan(camt).map(ImportPlan::Camt053),
    }
}

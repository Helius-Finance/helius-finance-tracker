use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::time::SystemTime;

use crate::db::Db;
use crate::error::AppError;
use crate::importer::{resolve_camt053_import_plan, resolve_csv_import_plan};
use crate::model::{ImportPlan, ImportResult};

pub use crate::importer::{Camt053ImportRequest, CsvImportRequest};

#[derive(Clone, Debug)]
pub enum ImportRequest {
    Csv(Box<CsvImportRequest>),
    Camt053(Camt053ImportRequest),
}

#[derive(Clone, Debug)]
pub struct ImportPreview {
    pub plan: ImportPlan,
    pub result: ImportResult,
    pub fingerprint: FileFingerprint,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileFingerprint {
    len: u64,
    modified: Option<SystemTime>,
    hash: u64,
}

impl FileFingerprint {
    fn of(path: &Path) -> Self {
        let bytes = fs::read(path).unwrap_or_default();
        let mut hasher = DefaultHasher::new();
        bytes.hash(&mut hasher);
        Self {
            len: bytes.len() as u64,
            modified: fs::metadata(path).and_then(|meta| meta.modified()).ok(),
            hash: hasher.finish(),
        }
    }
}

pub struct ImportService<'a> {
    db: &'a Db,
}

impl<'a> ImportService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn execute(&self, req: ImportRequest) -> Result<ImportResult, AppError> {
        let plan = resolve_plan(req)?;
        self.db.import_transactions(&plan)
    }

    pub fn preview(&self, req: ImportRequest) -> Result<ImportPreview, AppError> {
        let mut plan = resolve_plan(req)?;
        plan.set_dry_run(true);
        let fingerprint = FileFingerprint::of(plan.path());
        let result = self.db.import_transactions(&plan)?;
        Ok(ImportPreview {
            plan,
            result,
            fingerprint,
        })
    }

    pub fn commit(&self, preview: ImportPreview) -> Result<ImportResult, AppError> {
        if FileFingerprint::of(preview.plan.path()) != preview.fingerprint {
            return Err(AppError::Validation(
                "import file changed since preview; please preview again".to_string(),
            ));
        }
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

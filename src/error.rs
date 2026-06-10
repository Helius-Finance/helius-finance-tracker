use std::fmt;
use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntityKind {
    Account,
    Category,
    Goal,
    PlanningItem,
    RecurringRule,
    Scenario,
    Transaction,
}

impl fmt::Display for EntityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Account => "account",
            Self::Category => "category",
            Self::Goal => "goal",
            Self::PlanningItem => "planning item",
            Self::RecurringRule => "recurring rule",
            Self::Scenario => "scenario",
            Self::Transaction => "transaction",
        })
    }
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{kind} `{value}` was not found")]
    NotFoundEntity { kind: EntityKind, value: String },

    #[error("{kind} `{value}` already exists")]
    DuplicateEntity { kind: EntityKind, value: String },

    #[error("{field} {reason}")]
    FieldValidation { field: String, reason: String },

    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    AlreadyExists(String),
    #[error("{0}")]
    Config(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Db(#[from] rusqlite::Error),
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    DateParse(#[from] chrono::ParseError),
    #[error("{0}")]
    Http(String),
    #[error(transparent)]
    Clap(#[from] clap::Error),
}

impl AppError {
    pub fn missing_db(path: &Path) -> Self {
        Self::Config(format!(
            "database not initialized at {}; run `helius init` first",
            path.display()
        ))
    }

    pub fn invalid_ref(kind: EntityKind, value: &str) -> Self {
        Self::NotFoundEntity {
            kind,
            value: value.to_string(),
        }
    }

    pub fn duplicate(kind: EntityKind, value: &str) -> Self {
        Self::DuplicateEntity {
            kind,
            value: value.to_string(),
        }
    }

    pub fn field_validation(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::FieldValidation {
            field: field.into(),
            reason: reason.into(),
        }
    }

    pub fn path_message(prefix: &str, path: PathBuf) -> Self {
        Self::Config(format!("{prefix}: {}", path.display()))
    }
}
// SPDX-License-Identifier: AGPL-3.0-only

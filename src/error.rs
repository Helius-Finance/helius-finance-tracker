use std::fmt;
use std::path::{Path, PathBuf};

use thiserror::Error;

/// Named entity domains used in structured error variants.
///
/// Implemented for [`AppError::NotFoundEntity`] and [`AppError::DuplicateEntity`] so
/// that a future GUI can match on the kind rather than parsing the English message.
/// The [`fmt::Display`] impl produces the exact lowercase noun used in user-facing
/// messages (e.g. `Account` → `"account"`), keeping the zero-diff rule satisfied.
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
    // ── Structured variants (GUI-matchable) ─────────────────────────────────
    /// A named entity was looked up by name or id and did not exist.
    ///
    /// Display: `"{kind} \`{value}\` was not found"` — byte-identical to the
    /// string previously produced by [`AppError::invalid_ref`].
    #[error("{kind} `{value}` was not found")]
    NotFoundEntity { kind: EntityKind, value: String },

    /// A unique-constraint violation on a named entity.
    ///
    /// Display: `"{kind} \`{value}\` already exists"` — byte-identical to the
    /// string previously produced by [`AppError::duplicate`].
    #[error("{kind} `{value}` already exists")]
    DuplicateEntity { kind: EntityKind, value: String },

    /// A field-level value failed validation.
    ///
    /// Display: `"{field} {reason}"` — byte-identical to the strings previously
    /// produced by `AppError::Validation(format!("{field} {reason}"))` in
    /// `src/amount.rs`.
    #[error("{field} {reason}")]
    FieldValidation { field: String, reason: String },

    // ── Catch-all string-carrying variants ──────────────────────────────────
    /// Contextual validation errors that don't fit a structured variant (e.g.
    /// "cannot archive account while X still references it").
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    AlreadyExists(String),
    #[error("{0}")]
    Config(String),

    // ── Transparent wrappers ─────────────────────────────────────────────────
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

    /// Look up a named entity by name/ref; returns a typed [`NotFoundEntity`]
    /// error whose `Display` is `"{kind} \`{value}\` was not found"`.
    pub fn invalid_ref(kind: EntityKind, value: &str) -> Self {
        Self::NotFoundEntity {
            kind,
            value: value.to_string(),
        }
    }

    /// Unique-constraint violation on a named entity; returns a typed
    /// [`DuplicateEntity`] error whose `Display` is `"{kind} \`{value}\` already exists"`.
    pub fn duplicate(kind: EntityKind, value: &str) -> Self {
        Self::DuplicateEntity {
            kind,
            value: value.to_string(),
        }
    }

    /// Field-level value validation failure.  `Display` is `"{field} {reason}"`.
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

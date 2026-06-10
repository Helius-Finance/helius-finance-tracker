//! Service layer that sits between the [`crate::db::Db`] and every frontend
//! (CLI, TUI, and any future GUI).
//!
//! Each submodule exposes strongly-typed request / response structs and a
//! service struct that owns a borrow of `Db`. Services are the stable API
//! surface; frontends are thin adapters over them.
//!
//! Layering rules (enforced manually for now; see Phase 6):
//! - Services MUST NOT depend on `clap`, `ratatui`, `console`, `crate::output`,
//!   `crate::ui`, or any other frontend-specific module.
//! - Services MUST NOT write to stdout/stderr directly. They return data or
//!   [`crate::error::AppError`]; the caller renders.
//! - One-way dependency: frontends depend on services; services depend on the
//!   db and model; nothing depends on frontends.

pub mod accounts;
pub mod budgets;
pub mod categories;
pub mod import;
pub mod planning;
pub mod reconciliation;
pub mod recurring;
pub mod reporting;
pub mod transactions;

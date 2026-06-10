//! Integration tests for [`helius::services::import::ImportService`].
//!
//! Mirrors the pattern established by `tests/services_accounts.rs`:
//! exercise the service directly against a fresh on-disk SQLite
//! database. These tests cover behaviour the CLI integration tests in
//! `tests/cli.rs` cannot — namely the TUI-style two-step preview/commit
//! flow that goes through `ImportService::preview` + `commit` rather
//! than a single `helius import csv` invocation.

use std::path::PathBuf;

use helius::services::accounts::{AccountService, AddAccountRequest};
use helius::services::import::{
    Camt053ImportRequest, CsvImportRequest, ImportRequest, ImportService,
};
use helius::{AccountKind, Db, TransactionFilters};
use tempfile::TempDir;

fn fresh_db() -> (TempDir, Db) {
    let temp_dir = TempDir::new().expect("tempdir");
    let path = temp_dir.path().join("tracker.db");
    let db = Db::open_for_init(&path).expect("open_for_init");
    db.init("EUR").expect("init");
    drop(db);
    let db = Db::open_existing(&path).expect("open_existing");
    (temp_dir, db)
}

fn seed_checking(db: &Db) {
    AccountService::new(db)
        .add(AddAccountRequest {
            name: "Checking".to_string(),
            kind: AccountKind::Checking,
            opening_balance_cents: 0,
            opened_on: "2026-01-01".to_string(),
        })
        .expect("seed checking");
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("import")
        .join(name)
}

fn alpha_request(dry_run: bool) -> CsvImportRequest {
    CsvImportRequest {
        path: fixture_path("alpha-bank-gr.csv"),
        account: "Checking".to_string(),
        preset_id: Some("alpha-bank-gr".to_string()),
        date_column: None,
        amount_column: None,
        debit_column: None,
        credit_column: None,
        description_column: None,
        category_column: None,
        category: None,
        income_category: None,
        expense_category: None,
        payee_column: None,
        note_column: None,
        type_column: None,
        default_kind: None,
        date_format: None,
        delimiter: None,
        dry_run,
        allow_duplicates: false,
    }
}

fn camt_request(file: &str, dry_run: bool) -> Camt053ImportRequest {
    Camt053ImportRequest {
        path: fixture_path(file),
        account: "Checking".to_string(),
        income_category: None,
        expense_category: None,
        dry_run,
        allow_duplicates: false,
    }
}

fn count_transactions(db: &Db) -> usize {
    db.list_transactions(&TransactionFilters {
        from: None,
        to: None,
        account: None,
        category: None,
        search: None,
        limit: None,
        include_deleted: false,
    })
    .expect("list_transactions")
    .len()
}

#[test]
fn preview_then_commit_persists_rows_once() {
    let (_guard, db) = fresh_db();
    seed_checking(&db);
    let service = ImportService::new(&db);

    let preview = service
        .preview(ImportRequest::Csv(Box::new(alpha_request(false))))
        .expect("preview");
    assert!(
        preview.result.dry_run,
        "preview must report dry_run=true regardless of request"
    );
    assert_eq!(preview.result.imported_count, 2);
    assert_eq!(count_transactions(&db), 0, "preview must not persist rows");

    let result = service.commit(preview).expect("commit");
    assert!(!result.dry_run);
    assert_eq!(result.imported_count, 2);
    assert_eq!(
        count_transactions(&db),
        2,
        "commit must persist exactly the previewed rows"
    );
}

#[test]
fn preview_alone_does_not_persist_rows() {
    let (_guard, db) = fresh_db();
    seed_checking(&db);

    let _preview = ImportService::new(&db)
        .preview(ImportRequest::Csv(Box::new(alpha_request(false))))
        .expect("preview");
    // Drop the preview without calling commit.
    assert_eq!(count_transactions(&db), 0);
}

#[test]
fn execute_single_shot_with_dry_run_flag_respects_it() {
    let (_guard, db) = fresh_db();
    seed_checking(&db);

    let result = ImportService::new(&db)
        .execute(ImportRequest::Csv(Box::new(alpha_request(true))))
        .expect("execute dry-run");
    assert!(result.dry_run);
    assert_eq!(result.imported_count, 2);
    assert_eq!(
        count_transactions(&db),
        0,
        "execute with dry_run=true must not persist"
    );
}

#[test]
fn camt053_preview_commits_cleanly() {
    let (_guard, db) = fresh_db();
    seed_checking(&db);
    let service = ImportService::new(&db);

    let preview = service
        .preview(ImportRequest::Camt053(camt_request(
            "camt053-booked-credit.xml",
            false,
        )))
        .expect("camt preview");
    assert_eq!(preview.result.imported_count, 1);
    assert_eq!(count_transactions(&db), 0);

    let result = service.commit(preview).expect("camt commit");
    assert_eq!(result.imported_count, 1);
    assert_eq!(count_transactions(&db), 1);
}

#[test]
fn preview_of_empty_camt053_returns_zero_rows() {
    let (_guard, db) = fresh_db();
    seed_checking(&db);
    let service = ImportService::new(&db);

    let preview = service
        .preview(ImportRequest::Camt053(camt_request(
            "camt053-empty.xml",
            false,
        )))
        .expect("empty camt preview");
    assert_eq!(preview.result.imported_count, 0);
    assert!(preview.result.preview.is_empty());

    let result = service.commit(preview).expect("empty camt commit");
    assert_eq!(result.imported_count, 0);
    assert_eq!(count_transactions(&db), 0);
}

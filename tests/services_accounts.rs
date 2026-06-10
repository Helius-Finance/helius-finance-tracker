use helius::services::accounts::{
    AccountService, AddAccountRequest, DeleteAccountRequest, EditAccountRequest,
    ListAccountsRequest,
};
use helius::{AccountKind, AppError, Db};
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

fn seed_account(service: &AccountService, name: &str, kind: AccountKind) -> i64 {
    service
        .add(AddAccountRequest {
            name: name.to_string(),
            kind,
            opening_balance_cents: 0,
            opened_on: "2026-01-01".to_string(),
        })
        .expect("add_account")
}

#[test]
fn add_and_list_accounts_roundtrip() {
    let (_guard, db) = fresh_db();
    let service = AccountService::new(&db);

    seed_account(&service, "Savings", AccountKind::Savings);
    seed_account(&service, "Checking", AccountKind::Checking);
    seed_account(&service, "Wallet", AccountKind::Cash);

    let accounts = service.list(ListAccountsRequest).expect("list");
    let names: Vec<&str> = accounts.iter().map(|a| a.name.as_str()).collect();
    assert_eq!(names, vec!["Checking", "Savings", "Wallet"]);
    assert_eq!(accounts[0].kind, AccountKind::Checking);
    assert_eq!(accounts[2].kind, AccountKind::Cash);
}

#[test]
fn add_rejects_duplicate_name() {
    let (_guard, db) = fresh_db();
    let service = AccountService::new(&db);

    seed_account(&service, "Checking", AccountKind::Checking);
    let err = service
        .add(AddAccountRequest {
            name: "Checking".to_string(),
            kind: AccountKind::Checking,
            opening_balance_cents: 0,
            opened_on: "2026-01-01".to_string(),
        })
        .expect_err("duplicate should fail");
    assert!(
        matches!(
            err,
            AppError::DuplicateEntity { .. } | AppError::AlreadyExists(_)
        ),
        "got {err:?}"
    );
}

#[test]
fn edit_updates_fields_by_reference() {
    let (_guard, db) = fresh_db();
    let service = AccountService::new(&db);

    let id = seed_account(&service, "Checking", AccountKind::Checking);
    service
        .edit(EditAccountRequest {
            reference: id.to_string(),
            name: Some("Primary Checking".to_string()),
            opening_balance_cents: Some(150_000),
            ..EditAccountRequest::default()
        })
        .expect("edit");

    let listed = service.list(ListAccountsRequest).expect("list");
    let updated = listed.iter().find(|a| a.id == id).expect("account present");
    assert_eq!(updated.name, "Primary Checking");
    assert_eq!(updated.opening_balance_cents, 150_000);
    assert_eq!(updated.kind, AccountKind::Checking);
}

#[test]
fn edit_requires_at_least_one_field_change() {
    let (_guard, db) = fresh_db();
    let service = AccountService::new(&db);

    let id = seed_account(&service, "Checking", AccountKind::Checking);
    let err = service
        .edit(EditAccountRequest {
            reference: id.to_string(),
            ..EditAccountRequest::default()
        })
        .expect_err("no-op edit should fail");
    match err {
        AppError::Validation(message) => {
            assert_eq!(message, "account edit requires at least one field change");
        }
        other => panic!("expected Validation, got {other:?}"),
    }
}

#[test]
fn edit_rejects_missing_reference() {
    let (_guard, db) = fresh_db();
    let service = AccountService::new(&db);

    let err = service
        .edit(EditAccountRequest {
            reference: "9999".to_string(),
            name: Some("Nope".to_string()),
            ..EditAccountRequest::default()
        })
        .expect_err("missing ref should fail");
    assert!(
        matches!(
            err,
            AppError::NotFoundEntity { .. } | AppError::NotFound(_) | AppError::Validation(_)
        ),
        "got {err:?}"
    );
}

#[test]
fn delete_archives_account_and_hides_it_from_list() {
    let (_guard, db) = fresh_db();
    let service = AccountService::new(&db);

    let keep = seed_account(&service, "Checking", AccountKind::Checking);
    let archive = seed_account(&service, "Old Wallet", AccountKind::Cash);

    service
        .delete(DeleteAccountRequest {
            reference: archive.to_string(),
        })
        .expect("delete");

    let remaining: Vec<i64> = service
        .list(ListAccountsRequest)
        .expect("list")
        .iter()
        .map(|a| a.id)
        .collect();
    assert_eq!(remaining, vec![keep]);
}

#[test]
fn delete_rejects_missing_reference() {
    let (_guard, db) = fresh_db();
    let service = AccountService::new(&db);

    let err = service
        .delete(DeleteAccountRequest {
            reference: "9999".to_string(),
        })
        .expect_err("missing ref should fail");
    assert!(
        matches!(
            err,
            AppError::NotFoundEntity { .. } | AppError::NotFound(_) | AppError::Validation(_)
        ),
        "got {err:?}"
    );
}

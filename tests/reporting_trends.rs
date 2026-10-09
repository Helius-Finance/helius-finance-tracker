use chrono::{Datelike, Local, Months, NaiveDate};
use helius::services::reporting::ReportingService;
use helius::{AccountKind, Db};
use rusqlite::{params, Connection};

#[test]
fn cash_flow_and_balance_trends_group_by_month() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("trends.db");
    let db = Db::open_for_init(&path).unwrap();
    db.init("EUR").unwrap();
    let checking = db
        .add_account("Checking", &AccountKind::Checking, 1000, "2000-01-01")
        .unwrap();
    let savings = db
        .add_account("Savings", &AccountKind::Savings, -200, "2000-01-01")
        .unwrap();

    let conn = Connection::open(&path).unwrap();
    conn.execute(
        "INSERT INTO categories (id, name, kind)
         VALUES (9001, 'Trend income', 'income'), (9002, 'Trend expense', 'expense')",
        [],
    )
    .unwrap();

    let today = Local::now().date_naive();
    let current = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
    let first = current.checked_sub_months(Months::new(2)).unwrap();
    let second = first.checked_add_months(Months::new(1)).unwrap();
    let before_first = first.pred_opt().unwrap();
    let first_last_day = second.pred_opt().unwrap();
    let next = current.checked_add_months(Months::new(1)).unwrap();

    for (date, kind, cents, category, deleted_at) in [
        (before_first, "income", 200, 9001, None),
        (first, "income", 100, 9001, None),
        (first_last_day, "expense", 30, 9002, None),
        (current, "expense", 70, 9002, None),
        (current, "income", 999, 9001, Some("deleted")),
        (next, "income", 888, 9001, None),
    ] {
        conn.execute(
            "INSERT INTO transactions
                 (txn_date, kind, amount_cents, account_id, category_id, created_at, updated_at, deleted_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 'test', 'test', ?6)",
            params![date.to_string(), kind, cents, checking, category, deleted_at],
        )
        .unwrap();
    }
    conn.execute(
        "INSERT INTO transactions
             (txn_date, kind, amount_cents, account_id, to_account_id, created_at, updated_at)
         VALUES (?1, 'transfer', 500, ?2, ?3, 'test', 'test')",
        params![current.to_string(), checking, savings],
    )
    .unwrap();

    let month = |date: NaiveDate| date.format("%Y-%m").to_string();
    let service = ReportingService::new(&db);

    let flow = service.monthly_cash_flow_trend(3).unwrap();
    assert_eq!(
        flow.iter()
            .map(|p| (
                p.month.clone(),
                p.income_cents,
                p.expense_cents,
                p.net_cents
            ))
            .collect::<Vec<_>>(),
        vec![
            (month(first), 100, 30, 70),
            (month(second), 0, 0, 0),
            (month(current), 0, 70, -70),
        ]
    );

    let balances = service.total_balance_trend(3).unwrap();
    assert_eq!(
        balances
            .iter()
            .map(|p| (p.month.clone(), p.balance_cents))
            .collect::<Vec<_>>(),
        vec![
            (month(first), 1070),
            (month(second), 1070),
            (month(current), 1000),
        ]
    );

    assert_eq!(service.monthly_cash_flow_trend(0).unwrap().len(), 1);
    assert_eq!(service.total_balance_trend(100).unwrap().len(), 60);
}

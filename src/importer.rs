use std::fs;
use std::path::PathBuf;

use chrono::NaiveDate;
use quick_xml::de::from_str;
use serde::Deserialize;

use crate::error::AppError;
use crate::model::{
    Camt053ImportPlan, CsvAmountStrategy, CsvImportPlan, ImportColumn, ImportPlan,
    ImportPresetSummary, ImportPresetVerification, TransactionKind,
};

const DEFAULT_CSV_DATE_COLUMN: &str = "Date";
const DEFAULT_CSV_AMOUNT_COLUMN: &str = "Amount";
const DEFAULT_CSV_DESCRIPTION_COLUMN: &str = "Description";
const DEFAULT_CSV_DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CsvImportRequest {
    pub path: PathBuf,
    pub account: String,
    pub preset_id: Option<String>,
    pub date_column: Option<String>,
    pub amount_column: Option<String>,
    pub debit_column: Option<String>,
    pub credit_column: Option<String>,
    pub description_column: Option<String>,
    pub category_column: Option<String>,
    pub category: Option<String>,
    pub income_category: Option<String>,
    pub expense_category: Option<String>,
    pub payee_column: Option<String>,
    pub note_column: Option<String>,
    pub type_column: Option<String>,
    pub default_kind: Option<TransactionKind>,
    pub date_format: Option<String>,
    pub delimiter: Option<u8>,
    pub dry_run: bool,
    pub allow_duplicates: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Camt053ImportRequest {
    pub path: PathBuf,
    pub account: String,
    pub income_category: Option<String>,
    pub expense_category: Option<String>,
    pub dry_run: bool,
    pub allow_duplicates: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CsvMappingDefaults {
    pub preset_id: Option<String>,
    pub date_column: String,
    pub amount_column: Option<String>,
    pub debit_column: Option<String>,
    pub credit_column: Option<String>,
    pub description_column: String,
    pub category_column: Option<String>,
    pub payee_column: Option<String>,
    pub note_column: Option<String>,
    pub type_column: Option<String>,
    pub default_kind: Option<TransactionKind>,
    pub date_format: String,
    pub delimiter: char,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedImportRow {
    pub line_number: usize,
    pub txn_date: String,
    pub kind: TransactionKind,
    pub amount_cents: i64,
    pub category_ref: Option<String>,
    pub payee: Option<String>,
    pub note: Option<String>,
}

#[derive(Copy, Clone)]
struct PresetColumn {
    name: &'static str,
    aliases: &'static [&'static str],
}

impl PresetColumn {
    const fn new(name: &'static str, aliases: &'static [&'static str]) -> Self {
        Self { name, aliases }
    }

    fn to_import_column(self) -> ImportColumn {
        ImportColumn::with_aliases(self.name, self.aliases.iter().copied().map(str::to_string))
    }
}

#[derive(Copy, Clone)]
enum PresetAmountStrategy {
    Signed {
        amount_column: PresetColumn,
    },
    Split {
        debit_column: PresetColumn,
        credit_column: PresetColumn,
    },
}

#[derive(Copy, Clone)]
struct CsvImportPreset {
    id: &'static str,
    label: &'static str,
    delimiter: u8,
    date_format: &'static str,
    date_column: PresetColumn,
    amount_strategy: PresetAmountStrategy,
    description_column: PresetColumn,
    category_column: Option<PresetColumn>,
    payee_column: Option<PresetColumn>,
    note_column: Option<PresetColumn>,
    type_column: Option<PresetColumn>,
    default_kind: Option<TransactionKind>,
    verification: ImportPresetVerification,
}

const ALPHA_BANK_GR: CsvImportPreset = CsvImportPreset {
    id: "alpha-bank-gr",
    label: "Alpha Bank Greece CSV",
    delimiter: b';',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Date", &["Transaction Date"]),
    amount_strategy: PresetAmountStrategy::Split {
        debit_column: PresetColumn::new("Debit", &["Debit Amount"]),
        credit_column: PresetColumn::new("Credit", &["Credit Amount"]),
    },
    description_column: PresetColumn::new("Description", &["Transaction Description"]),
    category_column: None,
    payee_column: Some(PresetColumn::new("Beneficiary", &["Counterparty"])),
    note_column: Some(PresetColumn::new("Reference", &["Transaction Reference"])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::FirstParty,
};

const EUROBANK_GR: CsvImportPreset = CsvImportPreset {
    id: "eurobank-gr",
    label: "Eurobank Greece CSV",
    delimiter: b';',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Date", &["Transaction Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Amount", &["Net Amount"]),
    },
    description_column: PresetColumn::new("Details", &["Description"]),
    category_column: None,
    payee_column: Some(PresetColumn::new("Counterparty", &["Beneficiary"])),
    note_column: Some(PresetColumn::new("Reference", &["Transaction Reference"])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::FirstParty,
};

const NBG_GR: CsvImportPreset = CsvImportPreset {
    id: "nbg-gr",
    label: "National Bank of Greece CSV",
    delimiter: b';',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Transaction Date", &["Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Transaction Amount", &["Amount"]),
    },
    description_column: PresetColumn::new("Transaction Description", &["Description"]),
    category_column: None,
    payee_column: Some(PresetColumn::new("Counterparty", &["Payee"])),
    note_column: Some(PresetColumn::new("Reference", &["Notes"])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::FirstParty,
};

const PIRAEUS_GR: CsvImportPreset = CsvImportPreset {
    id: "piraeus-gr",
    label: "Piraeus Bank Greece CSV",
    delimiter: b';',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Date", &["Value Date"]),
    amount_strategy: PresetAmountStrategy::Split {
        debit_column: PresetColumn::new("Debit Amount", &["Debit"]),
        credit_column: PresetColumn::new("Credit Amount", &["Credit"]),
    },
    description_column: PresetColumn::new("Transaction Details", &["Description"]),
    category_column: None,
    payee_column: Some(PresetColumn::new("Counterparty", &["Beneficiary"])),
    note_column: Some(PresetColumn::new("Reference", &["Transaction Reference"])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::FirstParty,
};

const REVOLUT_CSV: CsvImportPreset = CsvImportPreset {
    id: "revolut-csv",
    label: "Revolut CSV",
    delimiter: b',',
    date_format: "%Y-%m-%d",
    date_column: PresetColumn::new("Date completed", &["Completed Date", "Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Amount", &["Base amount"]),
    },
    description_column: PresetColumn::new("Description", &["Reference"]),
    category_column: None,
    payee_column: Some(PresetColumn::new("Merchant", &["Counterparty"])),
    note_column: Some(PresetColumn::new("Reference", &["Notes"])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::FirstParty,
};

// --- Community presets -------------------------------------------------------
// These presets have fixture coverage in this repository, but bank export
// layouts can vary by region, account type, and product version.

const N26_EU: CsvImportPreset = CsvImportPreset {
    id: "n26",
    label: "N26 Bank (EU) CSV",
    delimiter: b',',
    date_format: "%Y-%m-%d",
    date_column: PresetColumn::new("Date", &["Booking Date", "Value Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Amount (EUR)", &["Amount", "Amount (EUR)"]),
    },
    description_column: PresetColumn::new("Payment reference", &["Reference", "Description"]),
    category_column: Some(PresetColumn::new("Category", &[])),
    payee_column: Some(PresetColumn::new(
        "Payee",
        &["Partner Name", "Counterparty"],
    )),
    note_column: Some(PresetColumn::new("Transaction type", &["Type"])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const MONZO_UK: CsvImportPreset = CsvImportPreset {
    id: "monzo",
    label: "Monzo (UK) CSV",
    delimiter: b',',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Date", &["Transaction Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Amount", &["Money In", "Money Out"]),
    },
    description_column: PresetColumn::new("Description", &["Notes and #tags"]),
    category_column: Some(PresetColumn::new("Category", &[])),
    payee_column: Some(PresetColumn::new("Name", &["Merchant"])),
    note_column: Some(PresetColumn::new("Notes and #tags", &["Notes"])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const STARLING_UK: CsvImportPreset = CsvImportPreset {
    id: "starling",
    label: "Starling Bank (UK) CSV",
    delimiter: b',',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Date", &["Transaction Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Amount (GBP)", &["Amount"]),
    },
    description_column: PresetColumn::new("Reference", &["Notes"]),
    category_column: Some(PresetColumn::new("Spending Category", &["Category"])),
    payee_column: Some(PresetColumn::new("Counter Party", &["Counterparty"])),
    note_column: Some(PresetColumn::new("Notes", &["Reference"])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const WISE_EU: CsvImportPreset = CsvImportPreset {
    id: "wise",
    label: "Wise (pan-European) CSV",
    delimiter: b',',
    date_format: "%d-%m-%Y",
    date_column: PresetColumn::new("Date", &["Finished on"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Amount", &["Source amount"]),
    },
    description_column: PresetColumn::new("Description", &["Reference"]),
    category_column: None,
    payee_column: Some(PresetColumn::new("Payee Name", &["Target name"])),
    note_column: Some(PresetColumn::new("Payment Reference", &["Reference"])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const DKB_DE: CsvImportPreset = CsvImportPreset {
    id: "dkb-de",
    label: "DKB (Germany) CSV",
    delimiter: b';',
    date_format: "%d.%m.%y",
    date_column: PresetColumn::new("Buchungstag", &["Buchungsdatum", "Wertstellung"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Betrag (\u{20ac})", &["Betrag", "Betrag (EUR)"]),
    },
    description_column: PresetColumn::new("Verwendungszweck", &["Buchungstext"]),
    category_column: None,
    payee_column: Some(PresetColumn::new(
        "Zahlungsempf\u{00e4}nger*in",
        &["Auftraggeber / Empf\u{00e4}nger", "Empf\u{00e4}nger"],
    )),
    note_column: Some(PresetColumn::new("Verwendungszweck", &["Buchungstext"])),
    // Intentionally no type_column: German export values (Ausgang/Eingang) do
    // not map to the parser's known transaction-type keywords. Rely on the
    // signed amount for income/expense classification instead.
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const COMMERZBANK_DE: CsvImportPreset = CsvImportPreset {
    id: "commerzbank-de",
    label: "Commerzbank (Germany) CSV",
    delimiter: b';',
    date_format: "%d.%m.%Y",
    date_column: PresetColumn::new("Buchungstag", &["Wertstellung"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Betrag", &["Betrag (EUR)", "Umsatz"]),
    },
    description_column: PresetColumn::new("Buchungstext", &["Verwendungszweck"]),
    category_column: None,
    // Commerzbank exports vary in how they name the counterparty column (some
    // templates omit it altogether), so leave payee empty by default and let
    // users add --payee-column manually if their export contains one.
    payee_column: None,
    note_column: Some(PresetColumn::new("Buchungstext", &["Verwendungszweck"])),
    // See DKB note above: German Umsatzart values (Kartenzahlung / Lohn...)
    // do not parse. Rely on the signed Betrag instead.
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

// --- United States -----------------------------------------------------------

const CHASE_US: CsvImportPreset = CsvImportPreset {
    id: "chase-us",
    label: "Chase (US) CSV",
    delimiter: b',',
    date_format: "%m/%d/%Y",
    date_column: PresetColumn::new("Transaction Date", &["Post Date", "Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Amount", &[]),
    },
    description_column: PresetColumn::new("Description", &["Memo"]),
    category_column: Some(PresetColumn::new("Category", &[])),
    payee_column: None,
    note_column: Some(PresetColumn::new("Type", &[])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const BANK_OF_AMERICA_US: CsvImportPreset = CsvImportPreset {
    id: "bank-of-america-us",
    label: "Bank of America (US) CSV",
    delimiter: b',',
    date_format: "%m/%d/%Y",
    date_column: PresetColumn::new("Date", &["Transaction Date", "Posted Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Amount", &[]),
    },
    description_column: PresetColumn::new("Description", &["Payee"]),
    category_column: None,
    payee_column: None,
    note_column: None,
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const WELLS_FARGO_US: CsvImportPreset = CsvImportPreset {
    id: "wells-fargo-us",
    label: "Wells Fargo (US) CSV",
    delimiter: b',',
    date_format: "%m/%d/%Y",
    date_column: PresetColumn::new("Date", &["Transaction date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Amount", &[]),
    },
    description_column: PresetColumn::new("Description", &["Memo"]),
    category_column: None,
    payee_column: None,
    note_column: None,
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const CITI_US: CsvImportPreset = CsvImportPreset {
    id: "citi-us",
    label: "Citi (US) CSV",
    delimiter: b',',
    date_format: "%m/%d/%Y",
    date_column: PresetColumn::new("Date", &["Transaction Date"]),
    amount_strategy: PresetAmountStrategy::Split {
        debit_column: PresetColumn::new("Debit", &["Withdrawal"]),
        credit_column: PresetColumn::new("Credit", &["Deposit"]),
    },
    description_column: PresetColumn::new("Description", &["Payee"]),
    category_column: None,
    payee_column: None,
    note_column: Some(PresetColumn::new("Status", &[])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

// --- United Kingdom (high-street) --------------------------------------------

const BARCLAYS_UK: CsvImportPreset = CsvImportPreset {
    id: "barclays-uk",
    label: "Barclays (UK) CSV",
    delimiter: b',',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Date", &["Transaction Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Amount", &[]),
    },
    description_column: PresetColumn::new("Memo", &["Description"]),
    category_column: Some(PresetColumn::new("Subcategory", &["Category"])),
    payee_column: None,
    note_column: Some(PresetColumn::new("Memo", &[])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const HSBC_UK: CsvImportPreset = CsvImportPreset {
    id: "hsbc-uk",
    label: "HSBC (UK) CSV",
    delimiter: b',',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Date", &["Transaction Date"]),
    amount_strategy: PresetAmountStrategy::Split {
        debit_column: PresetColumn::new("Paid out", &["Debit"]),
        credit_column: PresetColumn::new("Paid in", &["Credit"]),
    },
    description_column: PresetColumn::new("Description", &["Details"]),
    category_column: None,
    payee_column: None,
    note_column: Some(PresetColumn::new("Type", &[])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const LLOYDS_UK: CsvImportPreset = CsvImportPreset {
    id: "lloyds-uk",
    label: "Lloyds Bank (UK) CSV",
    delimiter: b',',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Transaction Date", &["Date"]),
    amount_strategy: PresetAmountStrategy::Split {
        debit_column: PresetColumn::new("Debit Amount", &["Debit"]),
        credit_column: PresetColumn::new("Credit Amount", &["Credit"]),
    },
    description_column: PresetColumn::new("Transaction Description", &["Description", "Narrative"]),
    category_column: None,
    payee_column: None,
    note_column: Some(PresetColumn::new("Transaction Type", &["Type"])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const NATWEST_UK: CsvImportPreset = CsvImportPreset {
    id: "natwest-uk",
    label: "NatWest / RBS (UK) CSV",
    delimiter: b',',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Date", &["Transaction Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Transaction Amount", &["Value", "Amount"]),
    },
    description_column: PresetColumn::new("Description", &["Narrative"]),
    category_column: None,
    payee_column: None,
    note_column: Some(PresetColumn::new("Type", &[])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

// --- Continental Europe ------------------------------------------------------

const ING_NL: CsvImportPreset = CsvImportPreset {
    id: "ing-nl",
    label: "ING (Netherlands) CSV",
    delimiter: b';',
    date_format: "%d-%m-%Y",
    date_column: PresetColumn::new("Datum", &["Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Bedrag (EUR)", &["Bedrag", "Amount"]),
    },
    description_column: PresetColumn::new("Mededelingen", &["Omschrijving"]),
    category_column: None,
    payee_column: Some(PresetColumn::new(
        "Naam/Omschrijving",
        &["Naam", "Tegenrekening"],
    )),
    note_column: Some(PresetColumn::new("MutatieSoort", &["Code"])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const DEUTSCHE_BANK_DE: CsvImportPreset = CsvImportPreset {
    id: "deutsche-bank-de",
    label: "Deutsche Bank (Germany) CSV",
    delimiter: b';',
    date_format: "%d.%m.%Y",
    date_column: PresetColumn::new("Buchungstag", &["Buchungsdatum", "Valuta"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Betrag (EUR)", &["Betrag", "Umsatz"]),
    },
    description_column: PresetColumn::new("Buchungstext", &["Verwendungszweck"]),
    category_column: None,
    payee_column: Some(PresetColumn::new(
        "Auftraggeber / Beg\u{00fc}nstigter",
        &["Beg\u{00fc}nstigter", "Auftraggeber"],
    )),
    note_column: Some(PresetColumn::new("Buchungstext", &[])),
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const BNP_PARIBAS_FR: CsvImportPreset = CsvImportPreset {
    id: "bnp-paribas-fr",
    label: "BNP Paribas (France) CSV",
    delimiter: b';',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Date op\u{00e9}ration", &["Date operation", "Date"]),
    amount_strategy: PresetAmountStrategy::Split {
        debit_column: PresetColumn::new("D\u{00e9}bit", &["Debit"]),
        credit_column: PresetColumn::new("Cr\u{00e9}dit", &["Credit"]),
    },
    description_column: PresetColumn::new("Libell\u{00e9}", &["Libelle", "Description"]),
    category_column: None,
    payee_column: None,
    note_column: None,
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const SANTANDER_ES: CsvImportPreset = CsvImportPreset {
    id: "santander-es",
    label: "Santander (Spain) CSV",
    delimiter: b';',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Fecha", &["Fecha operaci\u{00f3}n", "Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Importe", &["Amount", "Cantidad"]),
    },
    description_column: PresetColumn::new("Concepto", &["Descripci\u{00f3}n", "Description"]),
    category_column: None,
    payee_column: None,
    note_column: None,
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const INTESA_SANPAOLO_IT: CsvImportPreset = CsvImportPreset {
    id: "intesa-sanpaolo-it",
    label: "Intesa Sanpaolo (Italy) CSV",
    delimiter: b';',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Data", &["Data operazione", "Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Importo", &["Amount"]),
    },
    description_column: PresetColumn::new("Descrizione", &["Description", "Causale"]),
    category_column: None,
    payee_column: None,
    note_column: None,
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

// --- Asia-Pacific ------------------------------------------------------------

const COMMBANK_AU: CsvImportPreset = CsvImportPreset {
    id: "commbank-au",
    label: "Commonwealth Bank (Australia) CSV",
    delimiter: b',',
    date_format: "%d/%m/%Y",
    date_column: PresetColumn::new("Date", &["Transaction Date"]),
    amount_strategy: PresetAmountStrategy::Signed {
        amount_column: PresetColumn::new("Amount", &[]),
    },
    description_column: PresetColumn::new("Description", &["Narrative"]),
    category_column: None,
    payee_column: None,
    note_column: None,
    type_column: None,
    default_kind: None,
    verification: ImportPresetVerification::Community,
};

const CSV_PRESETS: [CsvImportPreset; 25] = [
    ALPHA_BANK_GR,
    EUROBANK_GR,
    NBG_GR,
    PIRAEUS_GR,
    REVOLUT_CSV,
    N26_EU,
    MONZO_UK,
    STARLING_UK,
    WISE_EU,
    DKB_DE,
    COMMERZBANK_DE,
    CHASE_US,
    BANK_OF_AMERICA_US,
    WELLS_FARGO_US,
    CITI_US,
    BARCLAYS_UK,
    HSBC_UK,
    LLOYDS_UK,
    NATWEST_UK,
    ING_NL,
    DEUTSCHE_BANK_DE,
    BNP_PARIBAS_FR,
    SANTANDER_ES,
    INTESA_SANPAOLO_IT,
    COMMBANK_AU,
];

pub fn import_preset_summaries() -> Vec<ImportPresetSummary> {
    CSV_PRESETS
        .iter()
        .map(|preset| ImportPresetSummary {
            id: preset.id.to_string(),
            label: preset.label.to_string(),
            verification: preset.verification,
        })
        .collect()
}

pub fn csv_mapping_defaults(preset_id: Option<&str>) -> Result<CsvMappingDefaults, AppError> {
    let preset = preset_id.map(find_csv_preset).transpose()?;
    let mut defaults = CsvMappingDefaults {
        preset_id: preset.map(|value| value.id.to_string()),
        date_column: preset
            .map(|value| value.date_column.name.to_string())
            .unwrap_or_else(|| DEFAULT_CSV_DATE_COLUMN.to_string()),
        amount_column: Some(DEFAULT_CSV_AMOUNT_COLUMN.to_string()),
        debit_column: None,
        credit_column: None,
        description_column: preset
            .map(|value| value.description_column.name.to_string())
            .unwrap_or_else(|| DEFAULT_CSV_DESCRIPTION_COLUMN.to_string()),
        category_column: preset
            .and_then(|value| value.category_column.map(|column| column.name.to_string())),
        payee_column: preset
            .and_then(|value| value.payee_column.map(|column| column.name.to_string())),
        note_column: preset
            .and_then(|value| value.note_column.map(|column| column.name.to_string())),
        type_column: preset
            .and_then(|value| value.type_column.map(|column| column.name.to_string())),
        default_kind: preset.and_then(|value| value.default_kind),
        date_format: preset
            .map(|value| value.date_format.to_string())
            .unwrap_or_else(|| DEFAULT_CSV_DATE_FORMAT.to_string()),
        delimiter: preset.map(|value| value.delimiter as char).unwrap_or(','),
    };

    if let Some(preset) = preset {
        match preset.amount_strategy {
            PresetAmountStrategy::Signed { amount_column } => {
                defaults.amount_column = Some(amount_column.name.to_string());
            }
            PresetAmountStrategy::Split {
                debit_column,
                credit_column,
            } => {
                defaults.amount_column = None;
                defaults.debit_column = Some(debit_column.name.to_string());
                defaults.credit_column = Some(credit_column.name.to_string());
            }
        }
    }

    Ok(defaults)
}

pub fn resolve_csv_import_plan(request: CsvImportRequest) -> Result<CsvImportPlan, AppError> {
    let preset = request
        .preset_id
        .as_deref()
        .map(find_csv_preset)
        .transpose()?;
    let account = normalize_required_field(&request.account, "account")?;
    let delimiter = request
        .delimiter
        .or_else(|| preset.map(|value| value.delimiter))
        .unwrap_or(b',');
    let date_format = normalize_optional_field(request.date_format).unwrap_or_else(|| {
        preset
            .map(|value| value.date_format.to_string())
            .unwrap_or_else(|| DEFAULT_CSV_DATE_FORMAT.to_string())
    });

    if delimiter == 0 || !delimiter.is_ascii() {
        return Err(AppError::Validation(
            "CSV delimiter must be a single ASCII character".to_string(),
        ));
    }

    let amount_strategy = resolve_amount_strategy(
        request.amount_column,
        request.debit_column,
        request.credit_column,
        preset,
    )?;
    let default_kind = request
        .default_kind
        .or_else(|| preset.and_then(|value| value.default_kind));
    if matches!(default_kind, Some(TransactionKind::Transfer)) {
        return Err(AppError::Validation(
            "import defaults cannot use transfer as a transaction type".to_string(),
        ));
    }

    Ok(CsvImportPlan {
        path: request.path,
        account,
        preset_id: request.preset_id.map(|value| value.trim().to_string()),
        date_column: resolve_required_column(
            request.date_column,
            preset.map(|value| value.date_column),
            Some(DEFAULT_CSV_DATE_COLUMN),
            "date column",
        )?,
        amount_strategy,
        description_column: resolve_required_column(
            request.description_column,
            preset.map(|value| value.description_column),
            Some(DEFAULT_CSV_DESCRIPTION_COLUMN),
            "description column",
        )?,
        category_column: resolve_optional_column(
            request.category_column,
            preset.and_then(|value| value.category_column),
        ),
        category: normalize_optional_field(request.category),
        income_category: normalize_optional_field(request.income_category),
        expense_category: normalize_optional_field(request.expense_category),
        payee_column: resolve_optional_column(
            request.payee_column,
            preset.and_then(|value| value.payee_column),
        ),
        note_column: resolve_optional_column(
            request.note_column,
            preset.and_then(|value| value.note_column),
        ),
        type_column: resolve_optional_column(
            request.type_column,
            preset.and_then(|value| value.type_column),
        ),
        default_kind,
        date_format,
        delimiter,
        dry_run: request.dry_run,
        allow_duplicates: request.allow_duplicates,
    })
}

pub fn resolve_camt053_import_plan(
    request: Camt053ImportRequest,
) -> Result<Camt053ImportPlan, AppError> {
    Ok(Camt053ImportPlan {
        path: request.path,
        account: normalize_required_field(&request.account, "account")?,
        income_category: normalize_optional_field(request.income_category),
        expense_category: normalize_optional_field(request.expense_category),
        dry_run: request.dry_run,
        allow_duplicates: request.allow_duplicates,
    })
}

pub fn load_import_rows(
    plan: &ImportPlan,
    db_currency: &str,
) -> Result<Vec<ParsedImportRow>, AppError> {
    let path = plan.path();
    if !path.exists() {
        return Err(AppError::Validation(format!(
            "import source not found: {}",
            path.display()
        )));
    }
    match plan {
        ImportPlan::Csv(plan) => parse_csv_rows(plan),
        ImportPlan::Camt053(plan) => parse_camt053_rows(plan, db_currency),
    }
}

pub fn parse_import_kind(value: &str) -> Result<TransactionKind, AppError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "income" | "credit" | "deposit" | "inflow" | "crdt" => Ok(TransactionKind::Income),
        "expense" | "debit" | "withdrawal" | "outflow" | "dbit" => Ok(TransactionKind::Expense),
        "transfer" => Ok(TransactionKind::Transfer),
        other => Err(AppError::Validation(format!(
            "unsupported import transaction type `{other}`"
        ))),
    }
}

pub fn parse_import_amount_to_cents(value: &str) -> Result<i64, AppError> {
    let raw = value.trim();
    if raw.is_empty() {
        return Err(AppError::Validation(
            "import amount cannot be empty".to_string(),
        ));
    }

    let mut negative = false;
    let mut cleaned = raw.replace(['\u{a0}', ' '], "");
    if cleaned.starts_with('(') && cleaned.ends_with(')') {
        negative = true;
        cleaned = cleaned[1..cleaned.len() - 1].to_string();
    }
    if let Some(rest) = cleaned.strip_prefix('-') {
        negative = true;
        cleaned = rest.to_string();
    } else if let Some(rest) = cleaned.strip_prefix('+') {
        cleaned = rest.to_string();
    }

    let cleaned: String = cleaned
        .chars()
        .filter(|ch| ch.is_ascii_digit() || matches!(ch, '.' | ',' | '\''))
        .collect();
    let cleaned = cleaned.replace('\'', "");
    if cleaned.is_empty() {
        return Err(AppError::Validation(format!(
            "unsupported import amount `{value}`"
        )));
    }

    let decimal_separator = resolve_decimal_separator(&cleaned);
    let normalized = normalize_amount_string(&cleaned, decimal_separator)?;
    let amount_cents = parse_normalized_amount_to_cents(&normalized)?;
    Ok(if negative {
        -amount_cents
    } else {
        amount_cents
    })
}

fn resolve_amount_strategy(
    amount_column: Option<String>,
    debit_column: Option<String>,
    credit_column: Option<String>,
    preset: Option<CsvImportPreset>,
) -> Result<CsvAmountStrategy, AppError> {
    let amount_column = normalize_optional_field(amount_column);
    let debit_column = normalize_optional_field(debit_column);
    let credit_column = normalize_optional_field(credit_column);

    if amount_column.is_some() && (debit_column.is_some() || credit_column.is_some()) {
        return Err(AppError::Validation(
            "use either --amount-column or the --debit-column/--credit-column pair".to_string(),
        ));
    }

    if let Some(column) = amount_column {
        return Ok(CsvAmountStrategy::Signed {
            amount_column: ImportColumn::new(column),
        });
    }

    if debit_column.is_some() || credit_column.is_some() {
        let debit_column = debit_column.ok_or_else(|| {
            AppError::Validation(
                "--debit-column and --credit-column must be provided together".to_string(),
            )
        })?;
        let credit_column = credit_column.ok_or_else(|| {
            AppError::Validation(
                "--debit-column and --credit-column must be provided together".to_string(),
            )
        })?;
        return Ok(CsvAmountStrategy::Split {
            debit_column: ImportColumn::new(debit_column),
            credit_column: ImportColumn::new(credit_column),
        });
    }

    match preset.map(|value| value.amount_strategy) {
        Some(PresetAmountStrategy::Signed { amount_column }) => Ok(CsvAmountStrategy::Signed {
            amount_column: amount_column.to_import_column(),
        }),
        Some(PresetAmountStrategy::Split {
            debit_column,
            credit_column,
        }) => Ok(CsvAmountStrategy::Split {
            debit_column: debit_column.to_import_column(),
            credit_column: credit_column.to_import_column(),
        }),
        None => Ok(CsvAmountStrategy::Signed {
            amount_column: ImportColumn::new(DEFAULT_CSV_AMOUNT_COLUMN),
        }),
    }
}

fn resolve_required_column(
    manual: Option<String>,
    preset: Option<PresetColumn>,
    default_name: Option<&str>,
    label: &str,
) -> Result<ImportColumn, AppError> {
    if let Some(value) = normalize_optional_field(manual) {
        return Ok(ImportColumn::new(value));
    }
    if let Some(column) = preset {
        return Ok(column.to_import_column());
    }
    if let Some(name) = default_name {
        return Ok(ImportColumn::new(name));
    }
    Err(AppError::Validation(format!("{label} is required")))
}

fn resolve_optional_column(
    manual: Option<String>,
    preset: Option<PresetColumn>,
) -> Option<ImportColumn> {
    normalize_optional_field(manual)
        .map(ImportColumn::new)
        .or_else(|| preset.map(PresetColumn::to_import_column))
}

fn normalize_required_field(value: &str, label: &str) -> Result<String, AppError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AppError::Validation(format!("{label} is required")));
    }
    Ok(trimmed.to_string())
}

fn normalize_optional_field(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn find_csv_preset(id: &str) -> Result<CsvImportPreset, AppError> {
    let trimmed = id.trim();
    CSV_PRESETS
        .iter()
        .find(|preset| preset.id.eq_ignore_ascii_case(trimmed))
        .copied()
        .ok_or_else(|| AppError::Validation(format!("unknown CSV preset `{trimmed}`")))
}

fn parse_csv_rows(plan: &CsvImportPlan) -> Result<Vec<ParsedImportRow>, AppError> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(plan.delimiter)
        .from_path(&plan.path)?;
    let headers = reader.headers()?.clone();

    let date_index = find_column_index(&headers, &plan.date_column)?;
    let description_index = find_column_index(&headers, &plan.description_column)?;
    let category_index = find_optional_column_index(&headers, plan.category_column.as_ref())?;
    let payee_index = find_optional_column_index(&headers, plan.payee_column.as_ref())?;
    let note_index = find_optional_column_index(&headers, plan.note_column.as_ref())?;
    let type_index = find_optional_column_index(&headers, plan.type_column.as_ref())?;

    let amount_indexes = match &plan.amount_strategy {
        CsvAmountStrategy::Signed { amount_column } => CsvAmountIndexes::Signed {
            amount_index: find_column_index(&headers, amount_column)?,
        },
        CsvAmountStrategy::Split {
            debit_column,
            credit_column,
        } => CsvAmountIndexes::Split {
            debit_index: find_column_index(&headers, debit_column)?,
            credit_index: find_column_index(&headers, credit_column)?,
        },
    };

    let mut rows = Vec::new();
    for (record_index, record) in reader.records().enumerate() {
        let record = record?;
        let line_number = record_index + 2;
        let txn_date = parse_import_date(
            required_csv_value(&record, date_index, line_number, &plan.date_column.name)?,
            &plan.date_format,
        )?;
        let description = required_csv_value(
            &record,
            description_index,
            line_number,
            &plan.description_column.name,
        )?
        .trim()
        .to_string();

        let row_type = type_index
            .and_then(|index| optional_csv_value(&record, index))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(parse_import_kind)
            .transpose()?;

        let (kind, amount_cents) = match amount_indexes {
            CsvAmountIndexes::Signed { amount_index } => {
                let amount_label = match &plan.amount_strategy {
                    CsvAmountStrategy::Signed { amount_column } => &amount_column.name,
                    CsvAmountStrategy::Split { .. } => unreachable!(),
                };
                let signed_amount = parse_import_amount_to_cents(required_csv_value(
                    &record,
                    amount_index,
                    line_number,
                    amount_label,
                )?)?;
                if signed_amount == 0 {
                    return Err(AppError::Validation(format!(
                        "amount must be non-zero on CSV line {line_number}"
                    )));
                }
                let kind = row_type.or(plan.default_kind).unwrap_or({
                    if signed_amount < 0 {
                        TransactionKind::Expense
                    } else {
                        TransactionKind::Income
                    }
                });
                if kind == TransactionKind::Transfer {
                    return Err(AppError::Validation(format!(
                        "CSV import does not support transfer rows (line {line_number})"
                    )));
                }
                (kind, signed_amount.abs())
            }
            CsvAmountIndexes::Split {
                debit_index,
                credit_index,
            } => {
                let debit = optional_csv_value(&record, debit_index)
                    .map(parse_import_amount_to_cents)
                    .transpose()?
                    .map(i64::abs)
                    .filter(|value| *value > 0);
                let credit = optional_csv_value(&record, credit_index)
                    .map(parse_import_amount_to_cents)
                    .transpose()?
                    .map(i64::abs)
                    .filter(|value| *value > 0);
                let (derived_kind, amount_cents) = match (debit, credit) {
                    (Some(_), Some(_)) => {
                        return Err(AppError::Validation(format!(
                            "CSV line {line_number} cannot contain both debit and credit values"
                        )))
                    }
                    (None, None) => {
                        return Err(AppError::Validation(format!(
                            "CSV line {line_number} must contain either a debit or a credit value"
                        )))
                    }
                    (Some(value), None) => (TransactionKind::Expense, value),
                    (None, Some(value)) => (TransactionKind::Income, value),
                };
                if let Some(kind) = row_type {
                    if kind != derived_kind {
                        return Err(AppError::Validation(format!(
                            "CSV line {line_number} has a type/amount mismatch"
                        )));
                    }
                }
                (derived_kind, amount_cents)
            }
        };

        let payee = payee_index
            .and_then(|index| optional_csv_value(&record, index))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .or_else(|| {
                if description.is_empty() {
                    None
                } else {
                    Some(description.clone())
                }
            });
        let note = note_index
            .and_then(|index| optional_csv_value(&record, index))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned);
        let category_ref = category_index
            .and_then(|index| optional_csv_value(&record, index))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned);

        rows.push(ParsedImportRow {
            line_number,
            txn_date,
            kind,
            amount_cents,
            category_ref,
            payee,
            note,
        });
    }

    Ok(rows)
}

fn parse_camt053_rows(
    plan: &Camt053ImportPlan,
    db_currency: &str,
) -> Result<Vec<ParsedImportRow>, AppError> {
    let xml = fs::read_to_string(&plan.path)?;
    let document: CamtDocument = from_str(&xml)
        .map_err(|error| AppError::Validation(format!("failed to parse camt.053 XML: {error}")))?;

    let mut rows = Vec::new();
    let mut line_number = 1_usize;
    for statement in document.statement_root.statements {
        for entry in statement.entries {
            if entry.status.as_deref().map(str::trim) != Some("BOOK") {
                continue;
            }

            let kind = match entry.credit_debit.trim() {
                "CRDT" => TransactionKind::Income,
                "DBIT" => TransactionKind::Expense,
                other => {
                    return Err(AppError::Validation(format!(
                        "unsupported camt.053 credit/debit indicator `{other}`"
                    )))
                }
            };
            let txn_date = entry
                .booking_date
                .as_ref()
                .and_then(CamtDateNode::to_iso_date)
                .or_else(|| {
                    entry
                        .value_date
                        .as_ref()
                        .and_then(CamtDateNode::to_iso_date)
                })
                .ok_or_else(|| {
                    AppError::Validation("camt.053 entry is missing booking/value date".to_string())
                })?;
            let entry_amount_cents =
                parse_camt_amount(&entry.amount, entry.amount.currency.as_deref(), db_currency)?;
            let details = entry
                .details
                .as_ref()
                .map(|value| value.transaction_details.as_slice())
                .unwrap_or(&[]);

            if details.is_empty() {
                rows.push(ParsedImportRow {
                    line_number,
                    txn_date: txn_date.clone(),
                    kind,
                    amount_cents: entry_amount_cents,
                    category_ref: None,
                    payee: entry.additional_info.clone(),
                    note: join_note_parts([
                        entry.account_service_ref.clone(),
                        entry.additional_info.clone(),
                    ]),
                });
                line_number += 1;
                continue;
            }

            let multiple_details = details.len() > 1;
            for detail in details {
                let detail_amount = extract_detail_amount(
                    detail,
                    entry_amount_cents,
                    db_currency,
                    multiple_details,
                )?;
                rows.push(ParsedImportRow {
                    line_number,
                    txn_date: txn_date.clone(),
                    kind,
                    amount_cents: detail_amount,
                    category_ref: None,
                    payee: derive_camt_payee(kind, detail),
                    note: derive_camt_note(&entry, detail),
                });
                line_number += 1;
            }
        }
    }

    Ok(rows)
}

fn extract_detail_amount(
    detail: &CamtTransactionDetails,
    entry_amount_cents: i64,
    db_currency: &str,
    multiple_details: bool,
) -> Result<i64, AppError> {
    if let Some(amount) = detail
        .amount_details
        .as_ref()
        .and_then(CamtAmountDetails::amount)
    {
        return parse_camt_amount(amount, amount.currency.as_deref(), db_currency);
    }
    if multiple_details {
        return Err(AppError::Validation(
            "camt.053 entries with multiple TxDtls must include per-transaction amounts"
                .to_string(),
        ));
    }
    Ok(entry_amount_cents)
}

fn derive_camt_payee(kind: TransactionKind, detail: &CamtTransactionDetails) -> Option<String> {
    let related = detail.related_parties.as_ref();
    let party_name = match kind {
        TransactionKind::Income => related
            .and_then(|value| value.debtor.as_ref())
            .and_then(CamtParty::name)
            .or_else(|| {
                related
                    .and_then(|value| value.creditor.as_ref())
                    .and_then(CamtParty::name)
            }),
        TransactionKind::Expense => related
            .and_then(|value| value.creditor.as_ref())
            .and_then(CamtParty::name)
            .or_else(|| {
                related
                    .and_then(|value| value.debtor.as_ref())
                    .and_then(CamtParty::name)
            }),
        TransactionKind::Transfer => None,
    };

    party_name.or_else(|| {
        detail
            .remittance_info
            .as_ref()
            .and_then(|value| value.unstructured.first().cloned())
    })
}

fn derive_camt_note(entry: &CamtEntry, detail: &CamtTransactionDetails) -> Option<String> {
    join_note_parts([
        detail
            .references
            .as_ref()
            .and_then(|value| value.account_service_reference.clone()),
        detail
            .references
            .as_ref()
            .and_then(|value| value.end_to_end_id.clone()),
        detail
            .references
            .as_ref()
            .and_then(|value| value.transaction_id.clone()),
        detail.additional_info.clone(),
        detail
            .remittance_info
            .as_ref()
            .map(|value| value.unstructured.join(" | "))
            .filter(|value| !value.is_empty()),
        entry.account_service_ref.clone(),
        entry.additional_info.clone(),
    ])
}

fn join_note_parts(parts: impl IntoIterator<Item = Option<String>>) -> Option<String> {
    let joined = parts
        .into_iter()
        .flatten()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    if joined.is_empty() {
        None
    } else {
        Some(joined.join(" | "))
    }
}

fn parse_camt_amount(
    amount: &CamtAmount,
    currency: Option<&str>,
    db_currency: &str,
) -> Result<i64, AppError> {
    if let Some(currency) = currency {
        if !currency.eq_ignore_ascii_case(db_currency) {
            return Err(AppError::Validation(format!(
                "camt.053 currency mismatch: expected {db_currency}, found {}",
                currency.trim()
            )));
        }
    }
    parse_import_amount_to_cents(&amount.value).map(i64::abs)
}

fn find_column_index(
    headers: &csv::StringRecord,
    column: &ImportColumn,
) -> Result<usize, AppError> {
    let names =
        std::iter::once(column.name.as_str()).chain(column.aliases.iter().map(String::as_str));
    for candidate in names {
        if let Some(index) = headers
            .iter()
            .position(|header| header.trim().eq_ignore_ascii_case(candidate.trim()))
        {
            return Ok(index);
        }
    }
    Err(AppError::Validation(format!(
        "CSV column `{}` was not found",
        column.name
    )))
}

fn find_optional_column_index(
    headers: &csv::StringRecord,
    column: Option<&ImportColumn>,
) -> Result<Option<usize>, AppError> {
    match column {
        Some(column) => find_column_index(headers, column).map(Some),
        None => Ok(None),
    }
}

fn required_csv_value<'a>(
    record: &'a csv::StringRecord,
    index: usize,
    line_number: usize,
    column_name: &str,
) -> Result<&'a str, AppError> {
    let value = record.get(index).ok_or_else(|| {
        AppError::Validation(format!(
            "CSV line {line_number} is missing the `{column_name}` column"
        ))
    })?;
    if value.trim().is_empty() {
        return Err(AppError::Validation(format!(
            "CSV line {line_number} has an empty `{column_name}` value"
        )));
    }
    Ok(value)
}

fn optional_csv_value(record: &csv::StringRecord, index: usize) -> Option<&str> {
    record.get(index).filter(|value| !value.trim().is_empty())
}

fn parse_import_date(value: &str, date_format: &str) -> Result<String, AppError> {
    Ok(NaiveDate::parse_from_str(value.trim(), date_format)?
        .format("%Y-%m-%d")
        .to_string())
}

fn resolve_decimal_separator(value: &str) -> Option<char> {
    let last_dot = value.rfind('.');
    let last_comma = value.rfind(',');
    match (last_dot, last_comma) {
        (Some(dot), Some(comma)) => Some(if dot > comma { '.' } else { ',' }),
        (Some(dot), None) if looks_like_decimal_separator(value, dot) => Some('.'),
        (None, Some(comma)) if looks_like_decimal_separator(value, comma) => Some(','),
        _ => None,
    }
}

fn looks_like_decimal_separator(value: &str, position: usize) -> bool {
    let decimals = value.len().saturating_sub(position + 1);
    (1..=2).contains(&decimals)
}

fn normalize_amount_string(
    value: &str,
    decimal_separator: Option<char>,
) -> Result<String, AppError> {
    let decimal_position = decimal_separator.and_then(|separator| value.rfind(separator));
    let mut normalized = String::new();
    for (index, ch) in value.chars().enumerate() {
        if ch.is_ascii_digit() {
            normalized.push(ch);
        } else if Some(index) == decimal_position {
            normalized.push('.');
        }
    }
    if normalized.is_empty() {
        return Err(AppError::Validation(format!(
            "unsupported import amount `{value}`"
        )));
    }
    Ok(normalized)
}

fn parse_normalized_amount_to_cents(value: &str) -> Result<i64, AppError> {
    let mut parts = value.split('.');
    let whole = parts
        .next()
        .unwrap_or_default()
        .parse::<i64>()
        .map_err(|_| AppError::Validation(format!("unsupported import amount `{value}`")))?;
    let fractional = parts.next();
    if parts.next().is_some() {
        return Err(AppError::Validation(format!(
            "unsupported import amount `{value}`"
        )));
    }
    let cents = match fractional {
        None => 0,
        Some(value) if value.len() == 1 => {
            value
                .parse::<i64>()
                .map_err(|_| AppError::Validation(format!("unsupported import amount `{value}`")))?
                * 10
        }
        Some(value) if value.len() == 2 => value
            .parse::<i64>()
            .map_err(|_| AppError::Validation(format!("unsupported import amount `{value}`")))?,
        Some(_) => {
            return Err(AppError::Validation(format!(
                "unsupported import amount `{value}`"
            )))
        }
    };
    Ok(whole * 100 + cents)
}

enum CsvAmountIndexes {
    Signed {
        amount_index: usize,
    },
    Split {
        debit_index: usize,
        credit_index: usize,
    },
}

#[derive(Debug, Deserialize)]
struct CamtDocument {
    #[serde(rename = "BkToCstmrStmt")]
    statement_root: CamtStatementRoot,
}

#[derive(Debug, Deserialize)]
struct CamtStatementRoot {
    #[serde(rename = "Stmt", default)]
    statements: Vec<CamtStatement>,
}

#[derive(Debug, Deserialize)]
struct CamtStatement {
    #[serde(rename = "Ntry", default)]
    entries: Vec<CamtEntry>,
}

#[derive(Debug, Deserialize)]
struct CamtEntry {
    #[serde(rename = "Sts")]
    status: Option<String>,
    #[serde(rename = "Amt")]
    amount: CamtAmount,
    #[serde(rename = "CdtDbtInd")]
    credit_debit: String,
    #[serde(rename = "BookgDt")]
    booking_date: Option<CamtDateNode>,
    #[serde(rename = "ValDt")]
    value_date: Option<CamtDateNode>,
    #[serde(rename = "AcctSvcrRef")]
    account_service_ref: Option<String>,
    #[serde(rename = "AddtlNtryInf")]
    additional_info: Option<String>,
    #[serde(rename = "NtryDtls")]
    details: Option<CamtEntryDetails>,
}

#[derive(Debug, Deserialize)]
struct CamtAmount {
    #[serde(rename = "@Ccy")]
    currency: Option<String>,
    #[serde(rename = "$text")]
    value: String,
}

#[derive(Debug, Deserialize)]
struct CamtDateNode {
    #[serde(rename = "Dt")]
    date: Option<String>,
    #[serde(rename = "DtTm")]
    date_time: Option<String>,
}

impl CamtDateNode {
    fn to_iso_date(&self) -> Option<String> {
        self.date
            .as_deref()
            .or(self.date_time.as_deref())
            .and_then(|value| value.get(..10))
            .map(str::to_string)
    }
}

#[derive(Debug, Deserialize)]
struct CamtEntryDetails {
    #[serde(rename = "TxDtls", default)]
    transaction_details: Vec<CamtTransactionDetails>,
}

#[derive(Debug, Deserialize)]
struct CamtTransactionDetails {
    #[serde(rename = "Refs")]
    references: Option<CamtReferences>,
    #[serde(rename = "AmtDtls")]
    amount_details: Option<CamtAmountDetails>,
    #[serde(rename = "RltdPties")]
    related_parties: Option<CamtRelatedParties>,
    #[serde(rename = "RmtInf")]
    remittance_info: Option<CamtRemittanceInfo>,
    #[serde(rename = "AddtlTxInf")]
    additional_info: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CamtReferences {
    #[serde(rename = "AcctSvcrRef")]
    account_service_reference: Option<String>,
    #[serde(rename = "EndToEndId")]
    end_to_end_id: Option<String>,
    #[serde(rename = "TxId")]
    transaction_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CamtAmountDetails {
    #[serde(rename = "TxAmt")]
    transaction_amount: Option<CamtNestedAmount>,
    #[serde(rename = "InstdAmt")]
    instructed_amount: Option<CamtNestedAmount>,
    #[serde(rename = "CntrValAmt")]
    counter_value_amount: Option<CamtNestedAmount>,
}

impl CamtAmountDetails {
    fn amount(&self) -> Option<&CamtAmount> {
        self.transaction_amount
            .as_ref()
            .or(self.instructed_amount.as_ref())
            .or(self.counter_value_amount.as_ref())
            .map(|value| &value.amount)
    }
}

#[derive(Debug, Deserialize)]
struct CamtNestedAmount {
    #[serde(rename = "Amt")]
    amount: CamtAmount,
}

#[derive(Debug, Deserialize)]
struct CamtRelatedParties {
    #[serde(rename = "Dbtr")]
    debtor: Option<CamtParty>,
    #[serde(rename = "Cdtr")]
    creditor: Option<CamtParty>,
}

#[derive(Debug, Deserialize)]
struct CamtParty {
    #[serde(rename = "Nm")]
    name: Option<String>,
}

impl CamtParty {
    fn name(&self) -> Option<String> {
        self.name
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    }
}

#[derive(Debug, Deserialize)]
struct CamtRemittanceInfo {
    #[serde(rename = "Ustrd", default)]
    unstructured: Vec<String>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{
        csv_mapping_defaults, import_preset_summaries, load_import_rows,
        parse_import_amount_to_cents, resolve_camt053_import_plan, resolve_csv_import_plan,
        Camt053ImportRequest, CsvImportRequest,
    };
    use crate::model::{CsvAmountStrategy, ImportPlan, ImportPresetVerification, TransactionKind};

    #[test]
    fn preset_list_contains_stable_ids() {
        let ids = import_preset_summaries()
            .into_iter()
            .map(|preset| preset.id)
            .collect::<Vec<_>>();
        assert_eq!(
            ids,
            vec![
                "alpha-bank-gr",
                "eurobank-gr",
                "nbg-gr",
                "piraeus-gr",
                "revolut-csv",
                "n26",
                "monzo",
                "starling",
                "wise",
                "dkb-de",
                "commerzbank-de",
                "chase-us",
                "bank-of-america-us",
                "wells-fargo-us",
                "citi-us",
                "barclays-uk",
                "hsbc-uk",
                "lloyds-uk",
                "natwest-uk",
                "ing-nl",
                "deutsche-bank-de",
                "bnp-paribas-fr",
                "santander-es",
                "intesa-sanpaolo-it",
                "commbank-au",
            ]
        );
    }

    #[test]
    fn preset_verification_levels_separate_first_party_from_community() {
        let summaries = import_preset_summaries();
        let first_party: Vec<_> = summaries
            .iter()
            .filter(|preset| preset.verification == ImportPresetVerification::FirstParty)
            .map(|preset| preset.id.as_str())
            .collect();
        let community: Vec<_> = summaries
            .iter()
            .filter(|preset| preset.verification == ImportPresetVerification::Community)
            .map(|preset| preset.id.as_str())
            .collect();
        assert_eq!(
            first_party,
            vec![
                "alpha-bank-gr",
                "eurobank-gr",
                "nbg-gr",
                "piraeus-gr",
                "revolut-csv",
            ]
        );
        assert_eq!(first_party.len(), 5);
        assert_eq!(community.len(), 20);
        assert!(community.contains(&"chase-us"));
        assert!(community.contains(&"commbank-au"));
    }

    #[test]
    fn defaults_reflect_preset_amount_strategy() {
        let alpha = csv_mapping_defaults(Some("alpha-bank-gr")).unwrap();
        assert!(alpha.amount_column.is_none());
        assert_eq!(alpha.debit_column.as_deref(), Some("Debit"));
        assert_eq!(alpha.credit_column.as_deref(), Some("Credit"));

        let revolut = csv_mapping_defaults(Some("revolut-csv")).unwrap();
        assert_eq!(revolut.amount_column.as_deref(), Some("Amount"));
    }

    #[test]
    fn csv_resolution_prefers_manual_fields_over_preset_defaults() {
        let plan = resolve_csv_import_plan(CsvImportRequest {
            path: PathBuf::from("bank.csv"),
            account: "Checking".to_string(),
            preset_id: Some("eurobank-gr".to_string()),
            date_column: Some("Booked On".to_string()),
            amount_column: Some("Net".to_string()),
            debit_column: None,
            credit_column: None,
            description_column: Some("Narrative".to_string()),
            category_column: None,
            category: None,
            income_category: None,
            expense_category: None,
            payee_column: None,
            note_column: None,
            type_column: None,
            default_kind: None,
            date_format: Some("%m/%d/%Y".to_string()),
            delimiter: Some(b','),
            dry_run: true,
            allow_duplicates: false,
        })
        .unwrap();

        assert_eq!(plan.date_column.name, "Booked On");
        assert_eq!(plan.description_column.name, "Narrative");
        assert_eq!(plan.date_format, "%m/%d/%Y");
        assert_eq!(plan.delimiter, b',');
        match plan.amount_strategy {
            CsvAmountStrategy::Signed { amount_column } => assert_eq!(amount_column.name, "Net"),
            CsvAmountStrategy::Split { .. } => panic!("expected signed amount strategy"),
        }
    }

    #[test]
    fn csv_resolution_accepts_split_amount_columns() {
        let plan = resolve_csv_import_plan(CsvImportRequest {
            path: PathBuf::from("bank.csv"),
            account: "Checking".to_string(),
            preset_id: None,
            date_column: None,
            amount_column: None,
            debit_column: Some("Debit".to_string()),
            credit_column: Some("Credit".to_string()),
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
            dry_run: true,
            allow_duplicates: false,
        })
        .unwrap();
        assert!(matches!(
            plan.amount_strategy,
            CsvAmountStrategy::Split { .. }
        ));
    }

    #[test]
    fn import_amount_parser_handles_eu_and_us_formats() {
        assert_eq!(parse_import_amount_to_cents("-15,25").unwrap(), -1525);
        assert_eq!(parse_import_amount_to_cents("1.234,56").unwrap(), 123456);
        assert_eq!(parse_import_amount_to_cents("1,234.56").unwrap(), 123456);
    }

    #[test]
    fn camt_plan_resolution_keeps_kind_specific_categories() {
        let plan = resolve_camt053_import_plan(Camt053ImportRequest {
            path: PathBuf::from("stmt.xml"),
            account: "Checking".to_string(),
            income_category: Some("Salary".to_string()),
            expense_category: Some("Bills".to_string()),
            dry_run: true,
            allow_duplicates: false,
        })
        .unwrap();
        let plan = ImportPlan::Camt053(plan);
        assert_eq!(
            plan.default_category_for_kind(TransactionKind::Income),
            Some("Salary")
        );
        assert_eq!(
            plan.default_category_for_kind(TransactionKind::Expense),
            Some("Bills")
        );
    }

    #[test]
    fn csv_loading_rejects_missing_debit_and_credit_values() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let csv_path = temp_dir.path().join("split.csv");
        std::fs::write(
            &csv_path,
            "Date,Debit,Credit,Description\n2026-03-01,,,Coffee\n",
        )
        .unwrap();

        let plan = resolve_csv_import_plan(CsvImportRequest {
            path: csv_path,
            account: "Checking".to_string(),
            preset_id: None,
            date_column: None,
            amount_column: None,
            debit_column: Some("Debit".to_string()),
            credit_column: Some("Credit".to_string()),
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
            dry_run: true,
            allow_duplicates: false,
        })
        .unwrap();

        let error = load_import_rows(&ImportPlan::Csv(Box::new(plan)), "EUR").unwrap_err();
        assert!(error
            .to_string()
            .contains("must contain either a debit or a credit value"));
    }
}

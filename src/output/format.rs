use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Output format options
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, clap::ValueEnum, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    #[serde(rename = "json-pretty")]
    #[clap(name = "json-pretty")]
    JsonPretty,
}

/// Trait for types that can be formatted for output
pub trait Outputable: Serialize {
    /// Column headers for table output
    fn headers() -> Vec<&'static str>;

    /// Row values matching headers order
    fn row(&self) -> Vec<String>;
}

/// Format and print a list of items
pub fn print_output<T: Outputable>(items: &[T], format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Table => super::table::print_table(items),
        OutputFormat::Json => super::json::print_json(items, false),
        OutputFormat::JsonPretty => super::json::print_json(items, true),
    }
}

/// Format and print a single item
pub fn print_single<T: Serialize>(item: &T, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Table | OutputFormat::Json => super::json::print_json_single(item, false),
        OutputFormat::JsonPretty => super::json::print_json_single(item, true),
    }
}

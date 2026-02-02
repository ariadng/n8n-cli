use crate::error::Result;
use crate::output::Outputable;
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};

/// Print items as a formatted table
pub fn print_table<T: Outputable>(items: &[T]) -> Result<()> {
    if items.is_empty() {
        println!("No results found.");
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(T::headers());

    for item in items {
        table.add_row(item.row());
    }

    println!("{table}");
    Ok(())
}

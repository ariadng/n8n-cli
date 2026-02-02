use crate::error::{N8nError, Result};
use serde::Serialize;

/// Print items as JSON array
pub fn print_json<T: Serialize>(items: &[T], pretty: bool) -> Result<()> {
    let output = if pretty {
        serde_json::to_string_pretty(items)
    } else {
        serde_json::to_string(items)
    }
    .map_err(N8nError::Serialize)?;

    println!("{}", output);
    Ok(())
}

/// Print a single item as JSON
pub fn print_json_single<T: Serialize>(item: &T, pretty: bool) -> Result<()> {
    let output = if pretty {
        serde_json::to_string_pretty(item)
    } else {
        serde_json::to_string(item)
    }
    .map_err(N8nError::Serialize)?;

    println!("{}", output);
    Ok(())
}

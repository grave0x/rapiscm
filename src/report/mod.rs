//! Report formatting dispatcher (table, JSON, markdown).
//! Also includes the static HTML site generator.

pub mod doc;
pub mod json;
pub mod site;
pub mod summary;
pub mod table;

use crate::types::{OutputFormat, ResponseResult};

/// Format scan results according to the chosen output format.
pub fn format_results(results: &[ResponseResult], format: OutputFormat) -> String {
    match format {
        OutputFormat::Table => table::format_table(results),
        OutputFormat::Json => json::format_json(results),
        OutputFormat::Markdown => {
            let s = summary::format_summary(results);
            let t = table::format_markdown_table(results);
            format!("{s}\n\n## Results\n\n{t}")
        }
        OutputFormat::Doc => doc::format_doc(results),
    }
}

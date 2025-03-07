mod core;

use core::search::{handle, SheetSearchResult};

/// entry point
pub fn search(
    keyword: &str,
    filepath: &str,
) -> Result<Vec<SheetSearchResult>, Box<dyn std::error::Error>> {
    handle(keyword, filepath)
}

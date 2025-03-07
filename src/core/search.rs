use std::fmt;

use calamine::{open_workbook, Data, Reader, Xlsx};

/// search result
#[derive(Debug)]
pub struct SheetSearchResult {
    pub sheet_name: String,
    pub row: usize,
    pub col: usize,
    pub cell_value: String,
}

impl fmt::Display for SheetSearchResult {
    /// format for display
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: ({}, {}) = {}",
            self.sheet_name, self.row, self.col, self.cell_value
        )
    }
}

/// search main handler
pub fn handle(
    keyword: &str,
    filepath: &str,
) -> Result<Vec<SheetSearchResult>, Box<dyn std::error::Error>> {
    let mut ret: Vec<SheetSearchResult> = vec![];

    let mut workbook: Xlsx<_> = open_workbook(filepath)?;
    workbook.sheet_names().iter().for_each(|sheet_name| {
        match workbook.worksheet_range(sheet_name.as_str()) {
            Ok(range) => range.rows().enumerate().for_each(|(row, row_range)| {
                row_range
                    .iter()
                    .enumerate()
                    .for_each(|(col, cell)| match cell {
                        Data::String(cell_value) => {
                            if cell_value.contains(keyword) {
                                let found = SheetSearchResult {
                                    sheet_name: sheet_name.to_owned(),
                                    row,
                                    col,
                                    cell_value: cell_value.to_owned(),
                                };
                                ret.push(found);
                            }
                        }
                        _ => {}
                    })
            }),
            _ => {}
        }
    });

    Ok(ret)
}

use usv::*;
use core::marker::Send;
use std::io::{
    Seek,
    Write,
};
use std::path::{
    Path,
    PathBuf,
};
use std::convert::AsRef;
use rust_xlsxwriter::{
    Workbook, 
    Worksheet,
    XlsxError,
};

/// Convert a USV file to an Excel Workbook then save iit to a file.
pub fn usv_to_xlsx_file(usv: &str, path: &impl AsRef<Path>) -> Result<(), XlsxError> {
    usv_to_xlsx_workbook(usv)?.save(path)
}

/// Convert a USV file to an Excel Workbook then return a buffer of bytes.
pub fn usv_to_xlsx_buffer(usv: &str) -> Result<Vec<u8>, XlsxError> {
    usv_to_xlsx_workbook(usv)?.save_to_buffer()
}

/// Convert a USV file to an Excel Workbook then write it to a given writer.
pub fn usv_to_xlsx_writer<WRITER: Write + Seek + Send>(usv: &str, writer: WRITER) -> Result<(), XlsxError> {
    usv_to_xlsx_workbook(usv)?.save_to_writer(writer)
}

/// Convert a USV file to an Excel Workbook.
pub fn usv_to_xlsx_workbook(usv: &str) -> Result<Workbook, XlsxError> {
    let mut workbook: Workbook = Workbook::new();
    for group in usv.groups() {
        let worksheet = usv_to_xlsx_worksheet(&group)?;
        workbook.push_worksheet(worksheet);
    };
    Ok(workbook)
}

/// Convert a USV group to an Excel Worksheet.
/// 
/// Example:
/// 
/// ```
/// use usv_to_xlsx::*;
/// let usv = "a␟b␟␞c␟d␟␞␝";
/// let mut worksheet =  usv_to_xlsx_worksheet(usv);
/// assert!(worksheet.is_ok())
/// ```
///
pub fn usv_to_xlsx_worksheet(usv: &str) -> Result<Worksheet, XlsxError> {
    let mut worksheet = Worksheet::new();
    let mut row: u32 = 0;
    for record in usv.records() {
        let mut col: u16 = 0;
        for unit in record.units() {
            worksheet.write(row, col, unit)?;
            col += 1;
        };
        row += 1;
    };
    Ok(worksheet)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use once_cell::sync::Lazy;

    pub static TESTS_DIR: Lazy<PathBuf> = Lazy::new(||
        [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>()
    );

    pub static EXAMPLE_XLSX_GROUPS: Lazy<Vec<u8>> = Lazy::new(||
        std::fs::read(&TESTS_DIR.join("common").join("example.xlsx")).expect("EXAMPLE_XLSX_GROUPS")
    );

     #[test]
    fn usv_to_xlsx_file_test() {
        let input = EXAMPLE_STYLE_CONTROLS_GROUPS;
        let dir = std::env::temp_dir();
        let path = dir.join("test.xlsx");
        usv_to_xlsx_file(input, &path).unwrap();
        //TODO test the file
    }

    #[test]
    fn usv_to_xlsx_buffer_test() {
        let input = EXAMPLE_STYLE_CONTROLS_GROUPS;
        let actual = usv_to_xlsx_buffer(input).unwrap();
        //TODO test the buffer by doing an unzip and comparing files
        //assert_eq!(actual, *EXAMPLE_XLSX_GROUPS);
    }

    #[test]
    fn usv_to_xlsx_workbook_test() {
        let input = EXAMPLE_STYLE_CONTROLS_GROUPS;
        let mut workbook = usv_to_xlsx_workbook(input).unwrap();
        assert_eq!((*workbook.worksheets()).len(), 2);
        //TODO test the inner data
    }

    #[test]
    fn usv_to_xlsx_worksheet_test() {
        let input = EXAMPLE_STYLE_CONTROLS_GROUPS;
        let mut _worksheet = usv_to_xlsx_worksheet(input).unwrap();
        //TODO test the inner data
    }

}

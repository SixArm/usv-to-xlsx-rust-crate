use usv::*;
use usv::str_ext::StrExt;
use core::marker::Send;
use std::io::{
    Seek,
    Write,
};
use std::path::Path;
use std::convert::AsRef;
use rust_xlsxwriter::{
    Workbook,
    Worksheet,
    XlsxError,
};

/// Convert USV text to an Excel Workbook then save it to a file.
///
/// Example:
///
/// ```
/// use usv_to_xlsx::*;
/// let usv = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝";
/// let path = std::path::Path::new("example.xlsx");
/// std::fs::remove_file(&path);
/// assert!(!&path.exists());
/// let mut workbook = usv_to_xlsx_file(usv, &path);
/// assert!(&path.exists());
/// ```
///
pub fn usv_to_xlsx_file(usv: &str, path: &impl AsRef<Path>) -> Result<(), XlsxError> {
    usv_to_xlsx_workbook(usv)?.save(path)
}

/// Convert USV text to an Excel Workbook then return a buffer of bytes.
///
/// Example:
///
/// ```
/// use usv_to_xlsx::*;
/// let usv = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝";
/// let buffer = usv_to_xlsx_buffer(usv).unwrap();
/// assert!(buffer.len() > 0);
/// ```
///
pub fn usv_to_xlsx_buffer(usv: &str) -> Result<Vec<u8>, XlsxError> {
    usv_to_xlsx_workbook(usv)?.save_to_buffer()
}

/// Convert USV text to an Excel Workbook then write it to a given writer.
///
/// Example:
///
/// ```
/// use usv_to_xlsx::*;
/// let usv = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝";
/// let path = std::path::Path::new("example.xlsx");
/// std::fs::remove_file(&path);
/// assert!(!path.exists());
/// let mut writer = std::fs::File::create(path).unwrap();
/// let result = usv_to_xlsx_writer(usv, writer);
/// assert!(&path.exists());
/// ```
///
pub fn usv_to_xlsx_writer<WRITER: Write + Seek + Send>(usv: &str, writer: WRITER) -> Result<(), XlsxError> {
    usv_to_xlsx_workbook(usv)?.save_to_writer(writer)
}

/// Convert USV text to an Excel Workbook.
///
/// Example:
///
/// ```
/// use usv_to_xlsx::*;
/// let usv = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝";
/// let mut workbook = usv_to_xlsx_workbook(usv);
/// assert!(workbook.is_ok())
/// ```
///
pub fn usv_to_xlsx_workbook(usv: &str) -> Result<Workbook, XlsxError> {
    let mut workbook: Workbook = Workbook::new();
    for group in usv.groups() {
        let worksheet = usv_group_to_xlsx_worksheet(&group)?;
        workbook.push_worksheet(worksheet);
    };
    Ok(workbook)
}

/// Convert USV groups to an Excel Workbook.
///
/// Example:
///
/// ```
/// use usv_to_xlsx::*;
/// use usv::str_ext::*;
/// let usv = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝";
/// let groups: usv::Groups = usv.groups().collect();
/// let mut workbook = usv_groups_to_xlsx_workbook(&groups);
/// assert!(workbook.is_ok())
/// ```
///
pub fn usv_groups_to_xlsx_workbook(groups: &Groups) -> Result<Workbook, XlsxError> {
    let mut workbook: Workbook = Workbook::new();
    for group in groups {
        let worksheet = usv_group_to_xlsx_worksheet(&group)?;
        workbook.push_worksheet(worksheet);
    };
    Ok(workbook)
}


/// Convert USV text to an Excel Worksheet.
///
/// Example:
///
/// ```
/// use usv_to_xlsx::*;
/// let usv = "a␟b␟␞c␟d␟␞";
/// let mut worksheet =  usv_to_xlsx_worksheet(usv);
/// assert!(worksheet.is_ok())
/// ```
///
pub fn usv_to_xlsx_worksheet(usv: &str) -> Result<Worksheet, XlsxError> {
    let records: usv::Records = usv.records().collect();
    usv_records_to_xlsx_worksheet(&records)
}

/// Convert USV group to an Excel Worksheet.
///
/// Example:
///
/// ```
/// use usv_to_xlsx::*;
/// use usv::str_ext::*;
/// let usv = "a␟b␟␞c␟d␟␞␝";
/// let group: usv::Records = usv.groups().next().unwrap();
/// let mut worksheet = usv_group_to_xlsx_worksheet(&group);
/// assert!(worksheet.is_ok())
/// ```
///
pub fn usv_group_to_xlsx_worksheet(group: &usv::Group) -> Result<Worksheet, XlsxError> {
    usv_records_to_xlsx_worksheet(group as &usv::Records)
}

/// Convert USV records to an Excel Worksheet.
///
/// Example:
///
/// ```
/// use usv_to_xlsx::*;
/// use usv::str_ext::*;
/// let usv = "a␟b␟␞c␟d␟␞";
/// let records: usv::Records = usv.records().collect();
/// let mut worksheet = usv_records_to_xlsx_worksheet(&records);
/// assert!(worksheet.is_ok())
/// ```
///
pub fn usv_records_to_xlsx_worksheet(records: &usv::Records) -> Result<Worksheet, XlsxError> {
    let mut worksheet = Worksheet::new();
    let mut row: u32 = 0;
    for record in records {
        let mut col: u16 = 0;
        for unit in record {
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
        let input = EXAMPLE_GROUPS_STYLE_SYMBOLS;
        let dir = std::env::temp_dir();
        let path = dir.join("test.xlsx");
        usv_to_xlsx_file(input, &path).unwrap();
        //TODO test the file
    }

    #[test]
    fn usv_to_xlsx_buffer_test() {
        let input = EXAMPLE_GROUPS_STYLE_SYMBOLS;
        let actual = usv_to_xlsx_buffer(input).unwrap();
        //TODO test the buffer by doing an unzip and comparing files
        //assert_eq!(actual, *EXAMPLE_XLSX_GROUPS);
    }

    #[test]
    fn usv_to_xlsx_workbook_test() {
        let input = EXAMPLE_GROUPS_STYLE_SYMBOLS;
        let mut workbook = usv_to_xlsx_workbook(input).unwrap();
        assert_eq!((*workbook.worksheets()).len(), 2);
        //TODO test the inner data
    }

    #[test]
    fn usv_to_xlsx_worksheet_test() {
        let input = EXAMPLE_GROUPS_STYLE_SYMBOLS;
        let mut _worksheet = usv_to_xlsx_worksheet(input).unwrap();
        //TODO test the inner data
    }

    #[test]
    fn usv_group_to_xlsx_worksheet_test() {
        let input: usv::Group = EXAMPLE_GROUP_STYLE_SYMBOLS.groups().next().unwrap();
        let mut _worksheet = usv_group_to_xlsx_worksheet(&input).unwrap();
        //TODO test the inner data
    }

    #[test]
    fn usv_records_to_xlsx_worksheet_test() {
        let input: usv::Records = EXAMPLE_RECORDS_STYLE_SYMBOLS.records().collect();
        let mut _worksheet = usv_records_to_xlsx_worksheet(&input).unwrap();
        //TODO test the inner data
    }

}

use usv::*;
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

/// Convert each USV file to an Excel Workbook then save it.
pub fn usv_to_xlsx_files(usv: &str, paths: &Vec<PathBuf/*TODO upgrade to AsRef */>) -> Result<(), XlsxError> {
    for (i, file) in usv.files().enumerate() {
        usv_to_xlsx_file(&file, &paths[i])?;
    }
    Ok(())
}

/// Convert a USV file to an Excel Workbook then save it.
pub fn usv_to_xlsx_file(usv: &str, path: &impl AsRef<Path>) -> Result<Workbook, XlsxError> {
    let mut workbook = usv_to_xlsx_workbook(usv)?;
    workbook.save(path)?;
    Ok(workbook)
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
/// let usv = "a␟b␟␞c␟d␟␞␝";
/// let mut worksheet = usv_to_xlsx_worksheet(usv);
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
    
    #[test]
    fn usv_to_xlsx_files_test() {
        let dir = std::env::temp_dir();
        let path0 = dir.join("test1.xlsx");
        let path1 = dir.join("test2.xlsx");
        let paths = vec![path0, path1];
        usv_to_xlsx_files(EXAMPLE_STYLE_CONTROLS_GROUPS, &paths).unwrap();
        //TODO test results
    }

     #[test]
    fn usv_to_xlsx_file_test() {
        let dir = std::env::temp_dir();
        let path = dir.join("test.xlsx");
        let mut workbook = usv_to_xlsx_file(EXAMPLE_STYLE_CONTROLS_GROUPS, &path).unwrap();
        let worksheets: &Vec<Worksheet> = workbook.worksheets();
        assert_eq!(worksheets.len(), 2);
        //TODO test the inner data
    }
  
    #[test]
    fn usv_to_xlsx_workbook_test() {
        let mut workbook = usv_to_xlsx_workbook(EXAMPLE_STYLE_CONTROLS_GROUPS).unwrap();
        let worksheets: &Vec<Worksheet> = workbook.worksheets();
        assert_eq!(worksheets.len(), 2);
        //TODO test the inner data
    }

    #[test]
    fn usv_to_xlsx_worksheet_test() {
        let mut _worksheet = usv_to_xlsx_worksheet(EXAMPLE_STYLE_CONTROLS_RECORDS).unwrap();
        //TODO test the inner data
    }

}

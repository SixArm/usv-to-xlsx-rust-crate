mod common; use common::*;
use std::process::Command;

#[test]
fn command() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual: Vec<u8> = command_io_str_to_bytes(&mut command, usv::examples::EXAMPLE_STYLE_SYMBOLS_GROUPS);
    // TODO unzip and diff
    //
    // diff -r t1 t2
    // diff --color -r t1/docProps/core.xml t2/docProps/core.xml
    //
    // < <cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"><dc:creator></dc:creator><cp:lastModifiedBy></cp:lastModifiedBy><dcterms:created xsi:type="dcterms:W3CDTF">2024-03-20T22:58:13Z</dcterms:created><dcterms:modified xsi:type="dcterms:W3CDTF">2024-03-20T22:58:13Z</dcterms:modified></cp:coreProperties>
    // \ No newline at end of file
    // ---
    // > <cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"><dc:creator></dc:creator><cp:lastModifiedBy></cp:lastModifiedBy><dcterms:created xsi:type="dcterms:W3CDTF">2024-03-20T22:58:26Z</dcterms:created><dcterms:modified xsi:type="dcterms:W3CDTF">2024-03-20T22:58:26Z</dcterms:modified></cp:coreProperties>
    // \ No newline at end of file
    //
    //assert_eq!(actual, *EXAMPLE_XLSX_GROUPS);
}

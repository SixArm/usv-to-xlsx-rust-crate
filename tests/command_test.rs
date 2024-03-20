mod common; use common::*;
use std::process::Command;

#[test]
fn command() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual: Vec<u8> = command_io_str_to_bytes(&mut command, usv::examples::EXAMPLE_STYLE_SYMBOLS_GROUPS);
    assert_eq!(actual, *EXAMPLE_XLSX_GROUPS);
}

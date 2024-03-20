//! Test helpers

use std::ffi::OsString;
use std::path::PathBuf;
use once_cell::sync::Lazy;
use std::path::Path;

////
//
// Paths that can simplify testing use cases. 
//
////

#[allow(dead_code)]
pub static CARGO_MANIFEST_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR")].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static TESTS_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static TMP_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "tmp"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static BENCHES_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "benches"].iter().collect::<PathBuf>()
);

////
//
// Command locators that can simplify testing use cases. 
//
////

pub const COMMAND: &str = "usv-to-xlsx";

#[allow(dead_code)]
pub static COMMAND_FILE: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "target", "debug", &COMMAND].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static COMMAND_OS: Lazy<OsString> = Lazy::new(||
    OsString::from([env!("CARGO_MANIFEST_DIR"), "target", "debug", &COMMAND].iter().collect::<PathBuf>())
);

////
//
// Command input/output helpers.
//
////

use std::process::{Command, Child, Stdio};
use std::io::Write;

/// Command input/output helper that takes stdin bytes then returns stdout String.
#[allow(dead_code)]
pub fn command_io(command: &mut Command, stdin: &[u8]) -> Vec<u8> {
    //// Spawn
    let mut child = command
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("command");

    //// Input
    let child_stdin = child.stdin.as_mut().expect("child_stdin");
    child_stdin.write_all(stdin).expect("write_all");
    #[allow(dropping_references)]
    let _ = drop(child_stdin);

    //// Output
    let output = child.wait_with_output().expect("wait_with_output");
    output.stdout
}

/// Command input/output helper that takes stdin bytes then returns stdout string.
#[allow(dead_code)]
pub fn command_io_bytes_to_string(command: &mut Command, stdin: &Vec<u8>) -> String {
    String::from_utf8(command_io(command, stdin)).unwrap()
}

/// Command input/output helper that takes stdin str then returns stdout bytes
#[allow(dead_code)]
pub fn command_io_str_to_bytes(command: &mut Command, stdin: &str) -> Vec<u8> {
    command_io(command, stdin.as_bytes())
}

////
//
// Utility helpers.
//
////

#[allow(dead_code)]
pub fn remove_file_if_exists<P: AsRef<Path>>(path: P) -> std::io::Result<()>
{
    if path.as_ref().exists() {
        ::std::fs::remove_file(path)
    } else {
        Ok(())
    }
}

////
//
// Example data suitable for testing.
//
////

#[allow(dead_code)]
pub static EXAMPLE_XLSX_GROUPS: Lazy<Vec<u8>> = Lazy::new(||
    std::fs::read(&TESTS_DIR.join("common").join("example.xlsx")).expect("EXAMPLE_XLSX_GROUPS")
);


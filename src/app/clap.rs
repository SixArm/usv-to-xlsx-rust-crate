//! clap setup.
//!
//! clap is a crate for command line argument parsing.
//! See https://docs.rs/clap/
//!
//! We prefer clap using the `command!` macro, which runs at compile time.
//! We prefer clap using the builder pattern, which offers more capabilities.
//!
//! We favor our convention of doing clap setup in a file named `clap.rs`,
//! rather than in `main.rs`, because we favor the separation of concerns.

use clap::Arg;
use clap::value_parser;
use std::path::PathBuf;

pub fn clap() -> crate::app::args::Args {
    let matches = clap::command!()
    .name("usv-to-xlsx")
    .version("1.2.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .about("Convert Unicode Separated Values (USV) to JavaScript Object Notation (XLSX)")
    .arg(Arg::new("test")
        .help("Print test output for debugging, verifying, tracing, and the like.\nExample: --test")
        .long("test")
        .action(clap::ArgAction::SetTrue))
    .arg(Arg::new("verbose")
        .help("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.\nExample: --verbose …")
        .short('v')
        .long("verbose")
        .action(clap::ArgAction::Count))
    .arg(Arg::new("output")
        .help("Set the output path")
        .short('o')
        .long("output")
        .default_value("output.xlsx")
        .value_parser(value_parser!(std::ffi::OsString))
        .action(clap::ArgAction::Set))
    .get_matches();
    
    crate::app::args::Args {
        test: matches.get_flag("test"),
        log_level: crate::app::log::u8_to_log_level(matches.get_count("verbose")),
        output_paths: matches.get_occurrences::<PathBuf>("output").unwrap().map(Iterator::collect).collect(),
    }
}

//! # usv-to-xlsx
//!
//! Convert [Unicode Separated Values (USV)](https://github.com/sixarm/usv)
//! to Microsoft Excel (XLSX).
//!
//! Syntax:
//!
//! ```sh
//! stdin | usv-to-xlsx [options] | stdout
//! ```
//!
//! Example:
//!
//! ```sh
//! cat example.usv | usv-to-xlsx
//! ```
//!
//! More examples below.
//!
//! ## Options
//!
//! * -h, --help : Print help
//!
//! * -V, --version : Print version
//!
//! * -v, --verbose... : Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace. Example: --verbose …
//!
//! * --test : Print test output for debugging, verifying, tracing, and the like. Example: --test
//!
//! ## Install
//!
//! Install:
//!
//! ```sh
//! cargo install usv-to-xlsx
//! ```
//!
//! Link: [https://crates.io/crates/usv-to-xlsx](https://crates.io/crates/usv-to-xlsx)
//!
//! ## Example
//!
//! Suppose example.usv contains:
//!
//! ```usv
//! a␟b␟␞
//! c␟d␟␞
//! ```
//!
//! Run:
//!
//! ```sh
//! cat example.usv | usv-to-xlsx > example.xlsx
//! ```
//!
//! You can now open the output file by using Microsoft Excel.
//!
//! ## FAQ
//!
//! ### What converters are available?
//!
//! * [asv-to-usv](https://crates.io/crates/asv-to-usv) & [usv-to-asv](https://crates.io/crates/usv-to-asv)
//!
//! * [csv-to-usv](https://crates.io/crates/asv-to-csv) & [usv-to-csv](https://crates.io/crates/usv-to-csv)
//!
//! * [json-to-usv](https://crates.io/crates/json-to-usv) & [usv-to-json](https://crates.io/crates/usv-to-json)
//!
//! * [xlsx-to-usv](https://crates.io/crates/xlsx-to-usv) & [usv-to-xlsx](https://crates.io/crates/usv-to-xlsx)
//!
//! ### When to use this command?
//!
//! Use this command when you want to convert from USV to XLSX.
//!
//! A typical use case is when you have USV data, such as a collection of units and
//! records, and you want to convert it to XLSX data, such as for a spreadsheet
//! import.
//!
//! Our real-world use case is converting a bunch of USV document-oriented data
//! from a variety of programs, including a CMS, to USV so we're better-able to
//! import the data into Excel.
//!
//! ### Why use USV instead of XLSX?
//!
//! See the documentation for [USV](https://github.com/sixarm/usv).
//!
//! ### Is USV aiming to become a standard?
//!
//! Yes, USV is submitted to IETF.org as an Internet-Draft work in progress:
//! [link](https://datatracker.ietf.org/doc/draft-unicode-separated-values/).
//!
//! ### Can I build my own USV tools?
//!
//! Yes, and you may freely use the
//! [USV Rust crate](https://github.com/sixarm/usv-rust-crate).
//!
//! ## Help wanted
//!
//! Constructive feedback welcome. Pull requests and feature requests welcome.
//!
//! ## Tracking
//!
//! * Package: usv-to-xlsx-rust-crate
//! * Version: 0.4.1
//! * Created: 2024-03-09T13:33:20Z
//! * Updated: 2024-04-11T19:55:29Z
//! * License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
//! * Contact: Joel Parker Henderson (joel@sixarm.com)

//// log
#[macro_use]
extern crate log;
extern crate env_logger;

pub mod app {
    pub mod args;
    pub mod clap;
    pub mod log;
}

use std::io::{Read, Write, self, stdin, stdout};


fn main() -> std::io::Result<()> {
    let args: crate::app::args::Args = crate::app::clap::clap();
    if args.test { println!("{:?}", args); }
    let mut stdin = stdin().lock();
    let mut s = String::new();
    stdin.read_to_string(&mut s)?;
    match usv_to_xlsx::usv_to_xlsx_buffer(&s) {
        Ok(buffer) => {
            stdout().write_all(&buffer)?
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

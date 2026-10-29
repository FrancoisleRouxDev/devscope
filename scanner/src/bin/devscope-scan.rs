//! # devscope-scan — command-line front-end to the scanner
//!
//! PURPOSE
//!   Run the scanner WITHOUT Python or Electron. Useful for:
//!     * debugging the Rust core in isolation,
//!     * benchmarking (`hyperfine "devscope-scan ~/big-repo"`),
//!     * comparing output against tools like `tokei` / `cloc`.
//!
//! PLANNED USAGE
//!   cargo run --release --bin devscope-scan -- <PATH> [--json] [--hidden] [--no-gitignore] [--top N]
//!
//! This binary only parses args, calls `devscope_scanner::scan`, and prints.
//! All real behaviour lives in the library so the CLI and Python stay in sync.

// ============================================================================
// LOGIC SKETCH (commented out — not implemented yet)
// ============================================================================
//
// use clap::Parser;
// use std::{path::PathBuf, process::ExitCode};
//
// #[derive(Parser)]
// #[command(version, about = "Count files and lines in a project")]
// struct Args {
//     path: PathBuf,
//     #[arg(long)] json: bool,
//     #[arg(long)] hidden: bool,
//     #[arg(long)] no_gitignore: bool,
//     #[arg(long, default_value_t = 20)] top: usize,
// }
//
// fn main() -> ExitCode {
//     let args = Args::parse();
//     match devscope_scanner::scan(&args.path, &opts_from(&args)) {
//         Ok(report) if args.json => { println!("{}", serde_json::to_string_pretty(&report)?); ExitCode::SUCCESS }
//         Ok(report)              => { print_table(&report); ExitCode::SUCCESS }
//         Err(e)                  => { eprintln!("error: {e}"); ExitCode::FAILURE }
//     }
// }

// Placeholder so the crate builds. Replace with the sketch above.
fn main() {}

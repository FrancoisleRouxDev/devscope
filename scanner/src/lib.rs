// lib.rs - crate root and PyO3 module definition.
// Declares the submodules and exposes the public Python API
// (e.g. scan_project(path, options) -> ScanResult) as the
// devscope_scanner Python module. No scanning logic lives here;
// it only wires Python <-> Rust.

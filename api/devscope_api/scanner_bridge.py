# scanner_bridge.py - thin wrapper around the compiled devscope_scanner
# Rust module. Keeps PyO3 details out of the routes and runs scans off the
# event loop (thread pool) so the API stays responsive.

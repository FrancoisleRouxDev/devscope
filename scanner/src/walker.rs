// walker.rs - recursive, parallel directory traversal.
// Walks a project tree, honouring .gitignore and a built-in ignore list
// (node_modules, target, .git, venv, dist, ...), and yields candidate
// source files to the counter.

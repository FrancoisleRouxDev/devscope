# scanner/tests/fixtures

Small, hand-written sample projects with **known** line counts, used by
`tests/scan_integration.rs`.

Planned layout:

```text
fixtures/
└── mini_project/
    ├── expected.json        # the exact ScanReport numbers the test asserts on
    ├── .gitignore           # ignores ignored_dir/ — proves gitignore support
    ├── main.rs              # code + // line comments + /* block */ comments
    ├── app.py               # code + # comments + docstrings
    ├── index.ts
    ├── ignored_dir/         # must NOT be counted
    └── node_modules/        # must NOT be counted even without .gitignore
```

Rules:

- Keep fixtures tiny (a few lines each) so expected counts are verifiable by eye.
- Fixtures that are OS-sensitive (symlinks, CRLF line endings, non-UTF-8
  names, binary blobs) are **generated at test time** in a temp dir instead of
  being committed — git and Windows checkouts would otherwise mangle them.
  See `/.gitattributes`.

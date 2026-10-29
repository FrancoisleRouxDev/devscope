# Junior vs Senior: how the same code gets written in Rust and Python

devscope's placeholder files each carry a **JUNIOR vs SENIOR** comment block
showing two ways to write that file's logic. This page collects the ideas
behind them and links to where each one appears.

"Junior" here doesn't mean "bad developer". It means code that works on the
happy path, on the author's machine, today. "Senior" code still works when
the input is strange, the OS is different, the load is higher, or someone
else changes it six months from now. The senior versions are usually not
longer. They come from **asking different questions** before writing code.

---

## The questions seniors ask

| Question                                          | Junior default                     | Senior default                                  |
|---------------------------------------------------|------------------------------------|-------------------------------------------------|
| What happens when this fails?                     | `unwrap()` / bare `except:`        | Typed errors, handled at the right layer        |
| What does the weirdest valid input look like?     | ASCII paths, UTF-8 files, small repos | Non-UTF-8 bytes, 500k files, symlink loops, CRLF |
| Does this work on Windows, macOS *and* Linux?     | Works on mine                      | `Path` APIs, no shell strings, per-OS branches isolated |
| Who else can call this?                           | "Only my frontend"                 | Any local process / web page → bind 127.0.0.1 + token |
| Where does state live and who cleans it up?       | Module-level globals               | Owned by a value with a lifecycle (`app.state`, lifespan) |
| Can I test this without the whole stack?          | Not really                         | Layers + interfaces (`Protocol`, pure functions) |
| Will the next person understand the contract?     | `dict` / `HashMap<String, …>`      | Named types: structs, enums, Pydantic models    |
| Does it block something it shouldn't?             | Blocking call in `async def`; GIL held | Threads for blocking work; GIL released in Rust |
| Is it bounded?                                    | Caches/queues grow forever         | Size limits, timeouts, eviction                 |

---

## Rust

| Topic | Junior | Senior | Where |
|---|---|---|---|
| Error handling | `unwrap()`, `String` errors, lost context | `thiserror` enum, `?`, path attached to the error, `#[non_exhaustive]` | [`scanner/src/error.rs`](../scanner/src/error.rs) |
| Closed sets of values | `String` language names, if/else chains | `Copy` enum + exhaustive `match`, data next to the type | [`scanner/src/language.rs`](../scanner/src/language.rs) |
| Reading files | `read_to_string` (UTF-8 only, whole file in RAM) | `BufReader` + one reused `Vec<u8>`, size check first, binary sniffing | [`scanner/src/counter.rs`](../scanner/src/counter.rs) |
| Walking directories | Hand-rolled recursion, follows symlinks, enters `node_modules` | `ignore::WalkBuilder`: parallel, `.gitignore`-aware, no symlink following | [`scanner/src/walker.rs`](../scanner/src/walker.rs) |
| Parallel aggregation | Global `Mutex<HashMap>` that every thread fights over | `rayon` `fold`/`reduce` with per-thread accumulators, bounded Top-N heap | [`scanner/src/stats.rs`](../scanner/src/stats.rs) |
| API shape | Five positional args, nested maps returned | `&Path` + options struct with `Default`, named return type | [`scanner/src/scan.rs`](../scanner/src/scan.rs) |
| Python bindings | Holds the GIL, panics cross the FFI, JSON round-trip | `py.detach`, mapped exceptions, `#[pyclass(frozen)]`, `.pyi` stubs | [`scanner/src/python.rs`](../scanner/src/python.rs) |

### Rust in short

1. **Make invalid states unrepresentable.** Enums over strings, newtypes over bare `u64`s when units matter.
2. **Borrow by default** (`&Path`, `&[u8]`); allocate deliberately.
3. **Panics are for bugs, not for input.** User data never panics.
4. **Use the proven crate** (`ignore`, `rayon`, `thiserror`) instead of rewriting it.
5. **Measure before optimising.** Write the clear version first, then benchmark (`criterion`, `hyperfine`), then optimise the hot path the numbers point to.

---

## Python

| Topic | Junior | Senior | Where |
|---|---|---|---|
| App structure | One `main.py`, global `app`, global dict | `create_app(settings)` factory, routes → services → bridge layers | [`api/devscope_api/app.py`](../api/devscope_api/app.py) |
| Configuration | `os.environ.get` everywhere, secrets printed | Frozen `pydantic-settings` model, `SecretStr`, required secrets have no default | [`api/devscope_api/config.py`](../api/devscope_api/config.py) |
| Request/response data | `body: dict`, `{"ok": True, "data": ...}` | Pydantic models with bounds, `response_model`, `extra="forbid"`, real status codes | [`api/devscope_api/schemas.py`](../api/devscope_api/schemas.py) |
| Calling native/blocking code | Blocking call in `async def`, bare `except:`, `print()` | Thread offload, specific exceptions with `raise ... from`, `logging`, `Protocol` for fakes | [`api/devscope_api/scanner_bridge.py`](../api/devscope_api/scanner_bridge.py) |
| Shared state & caching | Unbounded dict keyed by path, no locking | Bounded LRU, a lock around read-modify-write, immutable records swapped atomically | [`api/devscope_api/services/scan_jobs.py`](../api/devscope_api/services/scan_jobs.py) |
| Security | `0.0.0.0:8000`, no auth | `127.0.0.1`, random port, per-launch bearer token, `secrets.compare_digest` | [`api/devscope_api/security.py`](../api/devscope_api/security.py), [`__main__.py`](../api/devscope_api/__main__.py) |
| Testing | Manual clicking, or tests that need Rust compiled | `TestClient` + `FakeScanner` + `dependency_overrides`; real-extension tests use `importorskip` | [`api/tests/`](../api/tests/) |

### Python in short

1. **Type everything** and run `mypy --strict`. Types are documentation that the computer checks.
2. **Validate at the edges** (Pydantic, `pathlib`), then trust your own types inside.
3. **Never block the event loop.** `async def` is for awaiting, not for CPU or disk work.
4. **Catch narrowly, log fully** with `log.exception(...)`, and keep failures visible to the caller.
5. **No import-time side effects.** Importing a module shouldn't open sockets, read env vars, or load native code.

---

## Cross-cutting (Electron/TS)

The same habits apply in the desktop shell. See the comments in
[`dashboard/electron/main.js`](../dashboard/electron/main.js) (`nodeIntegration: true`
vs `contextIsolation` + `sandbox` + a narrow preload) and
[`dashboard/electron/sidecar.js`](../dashboard/electron/sidecar.js) (shell strings vs
`spawn` with an argument array; `taskkill /T` on Windows).

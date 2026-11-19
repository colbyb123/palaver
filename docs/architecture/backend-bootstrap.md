# Backend bootstrap plan

**Status:** Accepted following review of [issue #1](https://github.com/colbyb123/palaver/issues/1).

The first backend PR should start a native Windows HTTP process, expose a small health contract for [the frontend bootstrap](https://github.com/colbyb123/palaver/issues/2), and give contributors a repeatable way to build, test, and stop it. It follows the [architecture overview](overview.md) and [native runtime ADR](../adr/0004-native-runtime.md).

## Cargo and application layout

Use one Cargo package named `palaver` under `backend/`. Commit `backend/Cargo.lock`. At implementation time, select a stable Rust version compatible with the locked dependencies and pin it in a root `rust-toolchain.toml` with the minimal rustup profile, rustfmt, and Clippy. Use the same pin locally and in CI, and update it explicitly in a PR. Document rustup, the Windows MSVC host toolchain, and the Visual Studio C++ build tools and Windows SDK needed for native builds. Keep the pin portable across hosts.

- `backend/src/main.rs` starts Tokio, handles the `serve` command, initializes logging, binds the listener, and runs Axum. It is the application composition root. Add explicit module wiring here when the first feature arrives.
- `backend/src/config.rs` reads and validates startup settings. Keep parsing testable with supplied values so tests never need to mutate the process environment.
- `backend/src/http.rs` builds the router and defines the health handler. Router tests can send requests in process without binding a port.

## Startup and shutdown

From the repository root, run `cargo run --locked --manifest-path backend/Cargo.toml -- serve`. The compiled equivalent is `palaver.exe serve`. Support `--help` and return a nonzero exit code for unknown commands or invalid startup options. A small argument parser is sufficient for the initial command. Revisit the CLI library choice when adding jobs or additional flags.

Bind IPv4 loopback `127.0.0.1` on port `8080` by default. The optional `PALAVER_PORT` environment variable overrides the port. Validate it as a decimal port from 1 to 65535 and report invalid values or bind failures with an actionable error and a nonzero exit code. Keep the host fixed and use defaults plus environment settings for this PR. Revisit TOML when the app needs more configuration.

Use human-readable console logs via `tracing` and `tracing-subscriber`. Default to `info` and allow `RUST_LOG` to override the filter. Report the bound address after startup and log orderly shutdown and operational errors. Forward Tokio's Ctrl+C signal to Axum's graceful shutdown so it stops accepting new connections and lets in-flight requests finish. Treat a signal-handler error as a runtime failure with a diagnostic and a nonzero exit code. Scheduled startup and service-specific shutdown handling belong to later work.

## Health endpoint contract

| Item | Contract |
| --- | --- |
| Request | `GET /api/v1/health` |
| Successful status | `200 OK` |
| Content type | `application/json` |
| Body | `{"status":"ok"}` |
| Meaning | The HTTP process can respond. This is a liveness check. |

The frontend can use a relative request path with its local development proxy and treat network failures, non-200 responses, or unexpected data as unavailable. Keep database readiness out of this contract when persistence is added.

## First PR and verification

1. Add the Cargo package, locked dependencies, toolchain pin, and ignore rule for Cargo build output. Keep dependencies limited to HTTP, async runtime, JSON, and console logging needs.
2. Implement the startup path, router, and health response. Test default and overridden settings, invalid port values, and the health response status, content type, and parsed JSON body.
3. Extend the existing CI workflow with a Windows Rust job. From `backend/`, run `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked`, and `cargo build --locked`. Keep the existing repository checks.
4. Document native Windows prerequisites and run commands in the README. On native Windows, start from PowerShell and use `curl.exe -i http://127.0.0.1:8080/api/v1/health` from a second terminal to confirm the contract. Verify that Ctrl+C returns control to the original terminal and that restarting can bind the same port. Try an overridden port and an invalid value to verify both startup paths. Record the Windows environment and results in the PR.

## Follow-up work

Plan SQLite driver selection, database path, migrations, and persistence tests as a separate change. Frontend tooling and proxy setup belong to issue #2. Puzzle logic, Task Scheduler registration, and release packaging come later.

## References

- [Rust Windows MSVC target requirements](https://doc.rust-lang.org/rustc/platform-support/windows-msvc.html)
- [Axum graceful shutdown API](https://docs.rs/axum/latest/axum/serve/struct.Serve.html)

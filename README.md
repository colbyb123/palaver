# Palaver

Palaver is a local-first, modular personal dashboard for daily puzzles, news, and more. The first planned feature is a puzzle dashboard for finding daily games, recording results, and reviewing history and statistics.

The repository is in the foundation stage. The Rust backend uses Axum and Tokio and exposes a local health endpoint. The React and TypeScript frontend and SQLite persistence are planned.

- [Architecture overview](docs/architecture/overview.md)
- [Architecture decision records](docs/adr/README.md)
- [Roadmap and GitHub workflow](docs/roadmap.md)
- [Contributor and coding-agent guidance](AGENTS.md)

## Run the backend on Windows

Install Rust through [rustup](https://rustup.rs/) with the native MSVC toolchain. Install Visual Studio Build Tools with the Desktop development with C++ workload and a Windows SDK. The repository selects the pinned Rust toolchain and its formatting and linting components.

From the repository root in PowerShell:

```powershell
cargo run --locked --manifest-path backend/Cargo.toml -- serve
```

In a second terminal, check that the server responds with HTTP 200 and `{"status":"ok"}`:

```powershell
curl.exe -i http://127.0.0.1:8080/api/v1/health
```

Press Ctrl+C in the server terminal to stop it. To change the port for a new run, set `$env:PALAVER_PORT = "8090"` before starting the server. Use `Remove-Item Env:PALAVER_PORT` to return to the default. The server always binds to IPv4 loopback.

See the [backend bootstrap plan](docs/architecture/backend-bootstrap.md) for the API contract and verification checklist.

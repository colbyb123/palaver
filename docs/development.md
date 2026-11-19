# Local development

Palaver currently runs as a Rust HTTP server and a separate frontend development server. Native Windows is the primary target. The same backend can be built and run on Linux. The server listens on your own machine at `127.0.0.1:8080` by default.

## One-time setup

- **Windows:** Install Git, then install Visual Studio Build Tools with **Desktop development with C++** and a Windows SDK. Install Rust via rustup and use the native MSVC toolchain. Open a fresh PowerShell window after installation. [Microsoft's Windows Rust setup guide](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup)
- **Ubuntu or Debian:** Install Git, a compiler and linker, and curl with `sudo apt install git build-essential curl`. Install Rust through [rustup](https://rustup.rs/) and open a fresh terminal.
- **Frontend (Windows or Linux):** Install Node.js 24.22.3 and npm. The pinned version is also recorded in `frontend/.nvmrc`; use a Node version manager if desired.
- **VS Code:** Open the repository folder and install the **rust-analyzer** extension. The repo's toolchain file includes `rust-src` for editor analysis. If the editor still reports an old toolchain after a pin update, run **Developer: Reload Window**. [rust-analyzer installation guide](https://rust-analyzer.github.io/book/installation.html)

Clone the repo if needed and enter it. The existing clone can be used directly.

```sh
git clone https://github.com/colbyb123/palaver.git
cd palaver
```

From the repo root, install and confirm the selected toolchain:

```sh
rustup toolchain install --no-self-update
rustc --version
cargo --version
```

[`rust-toolchain.toml`](../rust-toolchain.toml) selects the Rust version and installs formatting, linting, and standard library sources. Cargo fetches the Rust dependencies during the first build using the committed lockfile. The initial build needs internet access. The backend itself needs no Node.js, database service, or scheduled task.

## Run and smoke-test

From the repository root in PowerShell or a Linux shell:

```sh
cargo run --locked --manifest-path backend/Cargo.toml -- serve
```

In a second terminal, send a request with `curl.exe` on Windows or `curl` on Linux. You can also open the URL directly in your browser.

```sh
curl -i http://127.0.0.1:8080/api/v1/health
```

Expect HTTP 200 and `{"status":"ok"}`. Press Ctrl+C in the server terminal to stop it.

If the default port is occupied, start the server on another port and use that same port in the request. On Windows PowerShell, set `$env:PALAVER_PORT = "8090"` before running Cargo and use `Remove-Item Env:PALAVER_PORT` to clear it. On Linux, use `PALAVER_PORT=8090 cargo run --locked --manifest-path backend/Cargo.toml -- serve` to override it for one run.

## Run the backend checks

From the repository root:

```sh
cd backend
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked
```

These are the checks run by the [Windows CI job](../.github/workflows/ci.yml). See the [backend bootstrap plan](architecture/backend-bootstrap.md) for the API contract.

## Run the frontend

With the backend running as above, open another terminal at the repository root:

```sh
npm ci --prefix frontend
npm run dev --prefix frontend
```

Open the printed Vite URL. The screen shows whether the backend responds; if it is unavailable, start the backend and select **Retry**. The frontend development server listens on loopback and proxies `/api` to `http://127.0.0.1:8080`. This proxy is only for development.

If the backend uses another port, set `PALAVER_API_ORIGIN` before starting Vite. In PowerShell, use `$env:PALAVER_API_ORIGIN = "http://127.0.0.1:8090"` and later `Remove-Item Env:PALAVER_API_ORIGIN`. On Linux, use `PALAVER_API_ORIGIN=http://127.0.0.1:8090 npm run dev --prefix frontend`. Restart Vite after changing it.

## Run the frontend checks

From the repository root:

```sh
cd frontend
npm ci
npm run lint
npm run typecheck
npm test
npm run build
```

The Windows CI job runs these checks. For a native Windows browser smoke test, confirm the connected status, stop the backend and use Retry to see unavailable, then restart it and retry successfully.

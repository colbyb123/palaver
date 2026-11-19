# Local development

Palaver currently runs as a Rust HTTP server and a separate frontend development server. Native Windows is the primary target. The same backend can be built and run on Linux. The server listens on your own machine at `127.0.0.1:8080` by default.

## Start Palaver

Complete the [one-time setup](#one-time-setup) first, including `npm ci --prefix frontend`. Then open **two terminals**, both in the repository root (`palaver/`, the directory containing `backend/` and `frontend/`). These commands work in PowerShell and Bash:

1. In the first terminal, start the backend and leave it running:

   ```sh
   cargo run --locked --manifest-path backend/Cargo.toml -- serve
   ```

2. In the second terminal, start the frontend and leave it running:

   ```sh
   npm run dev --prefix frontend
   ```

3. Open the **Local** URL printed by Vite, usually [http://127.0.0.1:5173](http://127.0.0.1:5173). The page should display **Backend status: Connected**. If it says unavailable, check that the backend is running and select **Retry**. Press Ctrl+C in each terminal to stop its server.

The browser connects to Vite, which proxies `/api` requests to the backend at `127.0.0.1:8080`. The Rust server does not currently serve the frontend. `npm run build --prefix frontend` produces static files in `frontend/dist/`, but starting the backend alone will not serve those files.

All npm commands in this guide use `--prefix frontend` and run from the repository root. If your terminal is already in `frontend/`, omit `--prefix frontend` instead: for example, `npm run dev`. The repository root has no `package.json`.

## One-time setup

You need Git, the Rust toolchain and a C++ compiler for the backend, and Node.js **24.22.3** with npm for the frontend. The first installation and builds require internet access. No database server is needed. Use native PowerShell on Windows or a Bash terminal on Ubuntu/Debian; open a new terminal after installing tools so PATH changes take effect.

### Windows

1. Install [Git for Windows](https://git-scm.com/install/windows), accepting the option to use Git from the command line.
2. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/). In the installer, select **Desktop development with C++** and a Windows SDK, then install the selected components.
3. Download and run [rustup-init.exe](https://rustup.rs/) and choose the default native MSVC installation. See [Microsoft's Rust setup guide](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup) if you need help with the C++ prerequisites.
4. Open the [Node.js 24.22.3 download archive](https://nodejs.org/en/download/archive/v24.22.3). Under Installer Packages, download and run the Windows `.msi` for your machine (usually x64; use ARM64 for Windows on ARM). Keep the installer options for npm and adding Node to PATH enabled. npm comes with this Node installer; it does not need a separate installation.
5. Close and reopen PowerShell, then check that the tools are available:

   ```powershell
   git --version
   rustup --version
   node --version
   npm --version
   ```

   `node --version` should print `v24.22.3`; `npm --version` should print a version number. If a command is not found, reopen the terminal and check the installer/PATH. If PowerShell blocks `npm.ps1` because of its execution policy, use `npm.cmd` in place of `npm` in the commands below, or consult your administrator about the policy.

### Ubuntu or Debian

Install Git and the native build tools, then install Rust with rustup:

```bash
sudo apt update
sudo apt install git build-essential curl ca-certificates
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the rustup prompts, then open a new terminal. Install [nvm](https://github.com/nvm-sh/nvm#installing-and-updating), a per-user Node version manager, using its published installer:

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.7/install.sh | bash
```

Open a new terminal so nvm is loaded, then install Node and its bundled npm:

```bash
nvm install 24.22.3
nvm use 24.22.3
git --version
rustup --version
node --version
npm --version
```

`node --version` should print `v24.22.3`; `npm --version` should print a version number. If `nvm` is not found after reopening the terminal, follow the troubleshooting steps in the [nvm installation guide](https://github.com/nvm-sh/nvm#installing-and-updating). You can later run `nvm use` from `frontend/` to select the version in `.nvmrc`.

### Get the project and install its dependencies

In PowerShell or Bash, clone the repository and enter it. If you already have a clone, open a terminal in its root instead.

```sh
git clone https://github.com/colbyb123/palaver.git
cd palaver
```

From the repository root, install the Rust version and components selected by the project, then check them:

```sh
rustup toolchain install --no-self-update
rustc --version
cargo --version
```

[`rust-toolchain.toml`](../rust-toolchain.toml) tells rustup which Rust release, rustfmt, Clippy, and editor sources to use when working in this repository. Cargo downloads the backend dependencies on the first build; `--locked` tells it to use the committed [`backend/Cargo.lock`](../backend/Cargo.lock) without changing the resolved versions.

Install the frontend dependencies from the repository root:

```sh
node --version
npm --version
npm ci --prefix frontend
```

[`frontend/.nvmrc`](../frontend/.nvmrc) records the Node version used by CI, but it does **not** install or switch Node automatically. The Windows installer steps above or `nvm use` on Linux select it. [`frontend/package.json`](../frontend/package.json) declares the frontend dependencies and commands; [`frontend/package-lock.json`](../frontend/package-lock.json) records their exact dependency tree. `npm ci` uses that lockfile to make a clean install in `frontend/node_modules`, replacing any existing `node_modules`. Run it again after pulling a changed lockfile. Do not edit either lockfile by hand; intentional dependency changes should update and commit the corresponding lockfile.

The repository root has no `package.json`: run npm commands with `--prefix frontend` from the root, or change to `frontend` first and omit the prefix. If `npm ci` reports an out-of-sync lockfile, first check that your checkout is up to date and that `node --version` and `npm --version` match the pinned Node release and its bundled npm (11.19.1). Report a persistent mismatch rather than changing the lockfile just to install the project.

### Editor (optional)

In VS Code, open the repository folder and install the [rust-analyzer extension](https://rust-analyzer.github.io/book/installation.html). If the editor reports an old Rust toolchain after a pin update, run **Developer: Reload Window**.

## Check the backend directly

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

## Use a different backend port

If the backend uses another port, set `PALAVER_API_ORIGIN` before starting Vite. In PowerShell, use `$env:PALAVER_API_ORIGIN = "http://127.0.0.1:8090"` and later `Remove-Item Env:PALAVER_API_ORIGIN`. On Linux, use `PALAVER_API_ORIGIN=http://127.0.0.1:8090 npm run dev --prefix frontend`. Restart Vite after changing it.

## Run the frontend checks

From the repository root:

```sh
npm ci --prefix frontend
npm run lint --prefix frontend
npm run typecheck --prefix frontend
npm test --prefix frontend
npm run build --prefix frontend
```

The Windows CI job runs these checks. For a native Windows browser smoke test, confirm the connected status, stop the backend and use Retry to see unavailable, then restart it and retry successfully.

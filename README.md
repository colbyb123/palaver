# Palaver

Palaver is a local-first, modular personal dashboard for daily puzzles, news, and more. The first planned feature is a puzzle dashboard for finding daily games, recording results, and reviewing history and statistics.

The repository is in the foundation stage. The Rust backend exposes a local health endpoint, and the React and TypeScript frontend displays its connection status. SQLite persistence is planned.

- [Architecture overview](docs/architecture/overview.md)
- [Architecture decision records](docs/adr/README.md)
- [Roadmap and GitHub workflow](docs/roadmap.md)
- [Contributor and coding-agent guidance](AGENTS.md)

## Local development

After the [one-time setup](docs/development.md#one-time-setup), start these commands in two terminals from the repository root:

```sh
cargo run --locked --manifest-path backend/Cargo.toml -- serve
```

```sh
npm run dev --prefix frontend
```

Open the Local URL printed by Vite in your browser. See the [local development guide](docs/development.md) for setup, details, and checks.

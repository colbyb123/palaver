# Palaver

Palaver is a local-first, modular personal dashboard for daily puzzles, news, and more. The first planned feature is a puzzle dashboard for finding daily games, recording results, and reviewing history and statistics.

The repository is in the foundation stage. The Rust backend uses Axum and Tokio and exposes a local health endpoint. The React and TypeScript frontend and SQLite persistence are planned.

- [Architecture overview](docs/architecture/overview.md)
- [Architecture decision records](docs/adr/README.md)
- [Roadmap and GitHub workflow](docs/roadmap.md)
- [Contributor and coding-agent guidance](AGENTS.md)

## Local development

See the [local development guide](docs/development.md) for Windows and Linux prerequisites, VS Code setup, how to run the backend, and how to run the CI checks.

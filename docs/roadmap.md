# Roadmap

| Milestone | Exit criteria |
| --- | --- |
| M0 — Foundation | Repository docs, issue and PR templates, baseline CI, and focused issues for both bootstraps. |
| M1 — Application core | Backend bootstrap runs natively on Windows with configuration, logging, and a proposed `/api/v1/health` endpoint. A separate frontend bootstrap reaches it locally. SQLite migrations and test setup follow. |
| M2 — Puzzle domain | Define module boundaries, result types, duplicate identity, and an ingestion example. Then implement parsing, validation, and durable history for the selected games. |
| M3 — Puzzle dashboard | Deliver daily puzzle links, paste-based submissions, history, and initial statistics. |
| M4 — First release | Verify native Windows installation and recovery, document backup and restore, and publish a downloadable release when packaging is ready. |
| M5 — Expansion | Explore additional modules, configurable widgets, new collection adapters, and Linux packaging as demand warrants. |

## GitHub workflow

Track work in GitHub Issues with a clear outcome, scope, and acceptance criteria. Use short-lived branches and focused PRs into `main`. Link the issue, describe verification, and review significant design changes before merging. Squash merging and required CI checks are proposed repository settings. A project board, label taxonomy, repository license, and visibility are administrative decisions.

The baseline CI job checks whitespace in changes on pull requests and pushes to `main`. Add Rust formatting, Clippy, tests, and build checks with the Cargo project. Add frontend lint, type checking, tests, and production build with the frontend and its lockfile. Include a Windows build and test run when the backend exists. Add database integration tests with persistence.

Release automation can build downloadable Windows artifacts once packaging is established, with Linux artifacts later. Local installation and Task Scheduler setup are separate from CI.

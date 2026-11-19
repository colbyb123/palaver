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

Track work in GitHub Issues with a clear outcome, scope, and acceptance criteria. `main` is the stable branch; `dev` integrates ongoing work. Start each small change from an up-to-date `dev`, use a descriptive short-lived branch such as `feature/puzzle-links`, `fix/health-status`, or `docs/setup`, and open a focused PR back into `dev`. Link the issue, describe verification, and review significant design changes before merging.

When a set of changes is ready for promotion, open a separate PR from `dev` to `main`. Use a merge commit for that promotion so Git retains the shared history between the long-lived branches. If an urgent change must be made on `main`, bring it back into `dev` before starting work that depends on it. Delete short-lived remote branches after their PRs are merged. Git cannot store a branch named `dev` alongside branch names starting with `dev/`, so use the prefixes above instead.

CI runs on PRs and on pushes to both `dev` and `main`. Required status checks, branch protection, squash merging for individual feature PRs, a project board, label taxonomy, repository license, and visibility remain repository settings or administrative decisions.

CI checks whitespace and runs Rust and frontend formatting or linting, type checks, tests, and builds on Windows as applicable. See the [workflow](../.github/workflows/ci.yml) for the current checks. Add database integration tests with persistence.

Release automation can build downloadable Windows artifacts once packaging is established, with Linux artifacts later. Local installation and Task Scheduler setup are separate from CI.

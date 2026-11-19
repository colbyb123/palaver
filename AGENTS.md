# Working on Palaver

Palaver is a local-first, single-user dashboard and a Rust learning project. The repository is the durable project record; GitHub issues track scoped work. Do not assume a historical chat or an issue checkbox reflects current implementation. Inspect the code and current issue before acting.

## Find context efficiently

- Start with [README](README.md) and the relevant issue. For setup and verification, use [development](docs/development.md).
- For design work, read the [architecture overview](docs/architecture/overview.md) and relevant [ADRs](docs/adr/README.md); consult the [roadmap](docs/roadmap.md) for sequencing. Read task-specific documents only as needed.
- Accepted ADRs record decisions; proposed plans and open questions are not decisions. Reconcile discrepancies with the user when consequential. Record significant decisions or changes in an ADR and update affected docs.

## Working conventions

- Keep changes focused on the issue and acceptance criteria. Plan structural changes before implementation and explain significant Rust tradeoffs and unfamiliar patterns clearly; the owner is learning Rust.
- Keep features in the modular monolith. Wire built-in modules explicitly at startup and keep feature domain logic independent of Axum, SQLite, and operating-system integrations.
- Use idiomatic Rust and TypeScript. Prefer composition and straightforward functions. Introduce traits at meaningful integration boundaries and justify new dependencies.
- Keep native Windows development and deployment viable. Isolate platform-specific startup and scheduling from feature code so Linux remains feasible.
- Test new business rules and changed boundaries. Run relevant formatting, lint, type, test, and build checks once their tooling exists. Document commands and results in the PR; distinguish native Windows verification from Linux checks.
- Update repository docs when behavior or build and run instructions change. Branch from `dev` using names such as `feature/...`, `fix/...`, or `docs/...`; open focused PRs into `dev`. Promote reviewed changes from `dev` to `main` with a separate PR. See the [GitHub workflow](docs/roadmap.md#github-workflow).

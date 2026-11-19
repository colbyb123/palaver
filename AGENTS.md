# Working on Palaver

Read the [architecture overview](docs/architecture/overview.md), [decision records](docs/adr/README.md), and [roadmap](docs/roadmap.md) before making design or implementation changes. Use the repository as the project record. Treat options listed as open or proposed as pending decisions and document significant changes in an ADR.

- Keep changes focused on the issue and its acceptance criteria. Plan structural changes before implementation and explain significant Rust tradeoffs in review.
- Keep features in the modular monolith. Wire modules explicitly at startup and keep feature domain logic independent of Axum, SQLite, and operating-system integrations.
- Use idiomatic Rust and TypeScript. Prefer composition and straightforward functions. Introduce traits at meaningful integration boundaries and justify new dependencies.
- Keep native Windows development and deployment viable. Isolate platform-specific startup and scheduling from feature code so Linux remains feasible.
- Test new business rules and changed boundaries. Run the relevant formatting, lint, type, test, and build checks once their tooling exists. Document commands and results in the PR.
- Update the repository docs when behavior or build and run instructions change. Use focused branches and GitHub PRs for review.

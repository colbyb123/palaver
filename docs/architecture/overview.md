# Architecture overview

## Product and operating model

Palaver is a single-user browser dashboard backed by a local server. Daily puzzles are the first feature. News and other information sources can be added as independent modules. The initial server listens on loopback, with the machine allowed to sleep normally. Portability means another machine can build and run the project with documented setup. Native Windows comes first, with Linux portability kept practical.

## Application structure

The React and TypeScript client calls a versioned HTTP API. Axum handles HTTP routing and request handling, and Tokio provides asynchronous execution and I/O. The Rust backend runs as a modular monolith. The application root owns startup, configuration, logging, shared database infrastructure, and module composition. Namespaces such as `/api/v1/puzzles/…` keep feature behavior out of the application root.

Feature modules own their use cases, domain rules, and storage queries. HTTP handlers and CLI entry points call use cases. Use cases enforce domain rules and use storage interfaces where a boundary is useful. SQLite adapters implement persistence, while domain types stay independent of transport and database types. Prefer composition and small functions, and add traits where they help isolate an external dependency or support interchangeable adapters.

The composition root explicitly registers built-in modules and wires their dependencies and routes. A backend module can supply data to several frontend widgets. Availability comes from registration, while enablement and homepage presentation are separate concerns. The first version uses built-in modules compiled with the application.

## Data and jobs

SQLite is the source of truth for historical puzzle results and durable application state. Keep machine-specific settings, such as filesystem paths and bind address, outside the database. Use migrations and document a consistent backup and restore procedure so a compatible installation can restore the same app state.

Initial puzzle ingestion is planned around pasted share results. The backend owns validation, duplicate handling, and persistence through a shared use case. Future import tools or browser collection adapters should be able to call that use case. Decide supported games, canonical result types, and duplicate identity during feature design.

Use one native executable with server and job subcommands. Windows Task Scheduler starts the server at login when desired and invokes periodic jobs as separate commands. Keep job logic callable without starting HTTP. Specify repeat-run and missed-run behavior before scheduling jobs that write data. Linux equivalents can be added outside the application core.

## Proposals and open details

- TOML is a candidate for machine settings and module enablement. The conversation favored SQLite for layout that users can edit in the browser. TOML homepage layout, defaults, and import/export remain open, along with the format and precedence of settings.
- Vite is the frontend bootstrap candidate. Choose package tooling and the production static asset strategy during scaffolding.
- SQLx and rusqlite are candidates for SQLite access. Pick the driver, data directory convention, and migration tooling during backend bootstrap or persistence design.
- Command examples such as `palaver serve` and `palaver jobs run daily` express the intended lifecycle. Final command syntax, scheduled task registration, and release packaging belong to later milestones.

See the [decision records](../adr/README.md) for rationale and the [roadmap](../roadmap.md) for delivery order.

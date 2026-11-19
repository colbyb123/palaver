# 0003 — Local persistence with SQLite

**Status:** Accepted

## Context

Palaver needs complete historical puzzle results and durable local state. A single-user local installation should have little database administration and a practical path to moving state between machines.

## Decision

Use SQLite with versioned schema migrations. Keep durable application state in the database and machine-specific configuration outside it. Feature modules own their data access while sharing connection and migration infrastructure.

## Consequences

Setup needs no separate database service. Document consistent backups and restoration, including how to handle active database writes. Server and scheduled writes must account for SQLite concurrency. Driver selection and the persistence format for homepage presentation are open.

# 0002 — Rust backend and React frontend

**Status:** Accepted

## Context

The project is an opportunity to learn Rust by building a maintainable native application. A browser client should provide a straightforward development path and room for interactive dashboards.

## Decision

Use Rust and Cargo for the backend, Axum for HTTP and Tokio for asynchronous I/O. Use React and TypeScript for the frontend and communicate over a versioned HTTP API.

## Consequences

Rust provides explicit ownership and typed modeling, with a learning curve around async code and shared state. Keep the first implementation small and explain unfamiliar patterns in review. Develop and validate the frontend independently through the API. Frontend tooling and production asset packaging remain open.

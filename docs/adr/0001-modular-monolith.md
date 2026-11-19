# 0001 — Modular monolith and feature boundaries

**Status:** Accepted

## Context

Palaver starts with puzzles and should accommodate independent features later. It runs on one machine for one user. Result collection can change without changing how results are validated and stored.

## Decision

Run one backend with feature-oriented modules. Use lightweight ports and adapters inside each feature and keep dependency wiring and explicit module registration at startup. Share platform infrastructure while feature modules own their domain and use cases.

## Consequences

One deployment keeps operations simple. Explicit registration makes dependencies and routes visible in code and means adding a built-in module requires a rebuild. Domain logic can be exercised independently of adapters. Avoid introducing a shared module interface until concrete repetition calls for it.

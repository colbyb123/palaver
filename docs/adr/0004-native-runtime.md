# 0004 — Native Windows runtime and scheduling

**Status:** Accepted

## Context

The primary installation is a personal Windows PC with browser access on the same host. Setup should stay lightweight and preserve a route to native Linux builds.

## Decision

Develop and deploy natively on Windows. Use a loopback HTTP server and CLI entry points for one-shot jobs. Use Windows Task Scheduler for periodic jobs and optional login startup.

## Consequences

Startup and schedule integration live outside feature code. The runtime needs no container, WSL environment, or dedicated scheduler service. A later Linux installation can use the same CLI from a service and timer. Define scheduling and recovery details when the first job is implemented.

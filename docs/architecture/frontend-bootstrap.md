# Frontend bootstrap plan

**Status:** Implemented following [issue #2](https://github.com/colbyb123/palaver/issues/2). See the code and development guide for current behavior and verification.

## Tooling and layout

- Put the frontend in `frontend/` and scaffold it with Vite's React and TypeScript template.
- Use npm, commit `package-lock.json`, and use `npm ci` for repeatable installs. Pin Node.js 24.22.3 in `.nvmrc`, document it, and use it in CI.
- Keep the bootstrap small: use the template's ESLint and TypeScript configuration, and add Vitest and React Testing Library for component behavior tests. Avoid routing, a component library, and a global state library until needed.

## Local development and API

Install Node.js and npm on native Windows as well as the backend prerequisites in the [development guide](../development.md). From the repository root, run `npm ci --prefix frontend` once and `npm run dev --prefix frontend` in a terminal. In another terminal run `cargo run --locked --manifest-path backend/Cargo.toml -- serve`. Open the Vite URL printed in the terminal.

Configure Vite to listen on loopback and proxy `/api` to `http://127.0.0.1:8080`. The browser requests the relative path `/api/v1/health`; the proxy avoids a cross-origin configuration change. For a backend on another port, allow an optional `PALAVER_API_ORIGIN` environment variable in the Vite configuration, defaulting to that address. Document PowerShell and Linux examples. This proxy is for development only; production asset serving is deferred.

## Screen and behavior

Show a Palaver heading and a plainly labeled backend status. On initial render, show a checking state, then request `GET /api/v1/health`. Show connected only for HTTP 200 with a JSON object whose `status` is `ok`, as specified in the [backend plan](backend-bootstrap.md). Treat a network error, another status, invalid JSON, or an unexpected response as unavailable. Offer a retry button so the user can start or restart the backend without reloading the page. Avoid presenting the liveness response as database or application readiness.

## Implementation PR

1. Scaffold and pin the frontend tooling and lockfile; add an ignore rule for generated dependencies and build output.
2. Add the loopback development proxy, typed health request, and minimal accessible status screen with retry.
3. Add behavior tests for the checking, connected, unavailable, and retry states, including malformed or unexpected responses. Mock the HTTP boundary in component tests rather than requiring a running backend.
4. Add a Windows CI job that uses the documented Node version and `npm ci`, then runs frontend lint, TypeScript checking, tests, and production build. Preserve the existing jobs.
5. Update the development guide and README with prerequisites, commands, proxy and port override. On native Windows, run both processes, verify connected status in a browser, stop the backend and verify unavailable status and retry, then restart and retry successfully. Record the environment and results in the PR. Run the same automated checks on Linux when available and distinguish those results from native Windows verification.

Dashboard widgets, puzzle UI, persistence, and production asset packaging remain follow-up work.

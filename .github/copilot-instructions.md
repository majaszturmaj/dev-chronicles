This project is a hybrid desktop application (Tauri) with a React + Vite frontend and a Rust backend that embeds an Axum ingestion server and uses SQLite (sqlx). The instructions below give focused, actionable context for AI code agents to be productive quickly.

1) Big-picture architecture (what to edit)
- Frontend: `src/` — React + TypeScript, built with Vite (`package.json` scripts). UI entry: `src/main.tsx` and `src/App.tsx`.
- Backend / app shell: `src-tauri/` — Rust Tauri crate. App bootstraps in `src-tauri/src/lib.rs` and `src-tauri/src/main.rs`.
- Ingestion HTTP server: embedded Axum server built in `src-tauri/src/server/mod.rs` with handlers in `src-tauri/src/server/handlers.rs`. Routes: `/ingest/terminal`, `/ingest/vscode`, `/ingest/browser`, `/health`.
- Database: SQLite via `sqlx` with schema in `src-tauri/src/db/schema.sql` (used by `src-tauri/src/db/mod.rs` and `src-tauri/src/db/models.rs`).
- AI integration: `src-tauri/src/ai/` — `client.rs` (reqwest wrapper) and `mod.rs` (generation logic and system prompt). AI provider is configurable via DB (`ai_settings`) and the UI.

2) Developer workflows & commands (what you can run)
- Frontend dev server (required by Tauri dev): npm run dev  (Vite on http://localhost:5173)
- Full app (Tauri):
  - Development: `npm run tauri:dev` (invokes `tauri dev`, expects `npm run dev` to be running or uses `beforeDevCommand` in `src-tauri/tauri.conf.json`).
  - Build: `npm run tauri:build` — builds frontend then Tauri package.
- Quick checks:
  - Build frontend only: `npm run build` (runs `tsc && vite build`).
  - Run health check against embedded server (after app starts): GET http://127.0.0.1:3030/health — returns "OK".

3) Project-specific patterns & important notes
- Tauri + Axum: The Rust side both exposes Tauri commands (for UI <-> backend RPC) and runs an internal HTTP server (Axum) on port 3030 for external ingestion. Editing either boundary requires thinking about serialization/types in `src-tauri/src/db/models.rs` and the Tauri commands in `src-tauri/src/commands.rs`.
- SQLx + chrono: DB rows store timestamps as RFC3339 strings. Conversions happen in `src-tauri/src/db/models.rs` via TryFrom for rows -> domain structs. Keep that pattern when adding queries.
- AI settings are persisted with id=1 (single-row config) — see `upsert_ai_settings` in `src-tauri/src/db/mod.rs`. When adding settings fields, update the SQL, the Rust models, and the frontend `Settings` component.
- Model selection logic: `src-tauri/src/ai/mod.rs` contains provider URL heuristics (openai/anthropic/local). Preserve or update this when changing AI behavior.
- Sanitization: `src-tauri/src/sanitizer` provides JSON sanitization used before sending logs to the AI. Never bypass it when calling `generate_summary`.

4) Common quick edits an agent might make
- Add a new ingestion route: add route in `src-tauri/src/server/mod.rs`, and reuse handler `handlers::ingest` or create a specialized handler in `src-tauri/src/server/handlers.rs`.
- Change Tauri command payloads: update `src-tauri/src/commands.rs` payload structs, update `tauri::generate_handler![]` list in `src-tauri/src/lib.rs`, and update the frontend caller in `src/App.tsx` or `src/utils/tauri.ts`.
- Update DB schema: modify `src-tauri/src/db/schema.sql`, then update `models.rs` conversions and any queries in `commands.rs` or `server/handlers.rs`.

5) Where to look for examples
- How frontend invokes backend commands: `src/App.tsx` -> `invokeCommand` (see `src/utils/tauri.ts`) and `src-tauri/src/commands.rs` functions `fetch_ai_settings`, `trigger_manual_summary`, `get_logs_by_date`.
- AI request formatting & system prompt: `src-tauri/src/ai/mod.rs` (SYSTEM_PROMPT and ChatRequest/Message shapes).
- HTTP ingest example and DB write: `src-tauri/src/server/handlers.rs` -> `insert_log` shows SQL insert pattern and error handling.

6) Constraints and gotchas for agents
- Do not commit secrets: API keys may be stored in DB at runtime; never hardcode secrets in code. Use the Settings UI or ask the user for safe injection.
- Local development needs both frontend dev server and Tauri dev (see `src-tauri/tauri.conf.json` beforeDevCommand). If tests or CI are added, prefer running frontend build first.
- When modifying the SQL schema, keep `init_db` (which runs `schema.sql`) consistent for new and existing users. Consider migrations (not present now).

7) Minimal checklist for PRs in this repo
- Run TypeScript checks: `npm run build` (ensures `tsc` passes).
- Build the Tauri app locally: `npm run tauri:dev` to smoke-test UI + backend.
- If Rust code changed: `cd src-tauri && cargo build` and run `cargo clippy` if available.

Feedback request: I created or updated this file summarizing discovered patterns and commands. Tell me what sections are unclear or need more detail (examples, exact SQL, or extra commands) and I will iterate.

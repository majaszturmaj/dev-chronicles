# DevChronicle

DevChronicle is a hybrid desktop application (Tauri) that ingests developer activity (browser, terminal, VSCode) and generates AI summaries. It includes a React + Vite frontend and a Rust/Tauri backend with an embedded Axum ingestion server and SQLite storage.

## 🚀 Quick Start

**The easiest way to run everything:**

```bash
./run.sh
```

This single command starts:
- ✅ Frontend dev server (Vite) on `http://localhost:5173`
- ✅ Desktop app window with Tauri
- ✅ Backend ingestion server on `http://127.0.0.1:3030`

Press `Ctrl+C` to stop. That's it!

## Setup

These steps get the project running locally. The project expects Node (npm) and Rust toolchains installed.

### Linux

1. Install prerequisites

   - Node.js (18+ recommended) and npm
   - Rust (rustup) and cargo
   - System libs for Tauri (GTK, libssl, build tools). On Debian/Ubuntu these commonly include `build-essential`, `libgtk-3-dev`, `libssl-dev`, and others required by `@tauri-apps/cli`.

2. Install JS deps

```bash
npm install
```

3. Start the frontend dev server (Vite)

```bash
npm run dev
```

4. In a second terminal start the Tauri app (this may run the frontend automatically depending on `tauri.conf.json`)

```bash
npm run tauri:dev
```

5. Quick verify: health endpoint

After the app (Tauri) starts, the embedded ingestion server listens on port `3030`. Verify with:

```bash
curl -sS http://127.0.0.1:3030/health
# should return: OK
```

### Windows

1. Install prerequisites

   - Node.js (18+ recommended) and npm
   - Rust (rustup) and cargo
   - Visual Studio Build Tools (MSVC toolchain / Desktop development with C++) for building Tauri on Windows

2. Install JS deps

```powershell
npm install
```

3. Start the frontend dev server

```powershell
npm run dev
```

4. Start the Tauri dev app (in a new terminal)

```powershell
npm run tauri:dev
```

5. Verify health endpoint

```powershell
curl -sS http://127.0.0.1:3030/health
# should return: OK
```

## Testing the ingestion server

See `docs/ingest-samples.md` for a collection of sample curl commands that post typical browser, terminal, and VSCode events to the embedded Axum server. These are copy/paste-ready and useful for local testing.

### Quick Setup and Test Script

For automated setup and testing, use the Python quick setup script:

```bash
python3 quick_setup.py
```

This script will:
1. Set up all extensions (terminal hook, VS Code compilation)
2. Start the DevChronicle application
3. Send 20 realistic "building Yocto Linux" workflow events to test AI summarization

**Requirements:**
- Python 3.6+
- `requests` library (optional, falls back to curl): `pip install requests`
- All system dependencies (curl, jq, bc, node, npm, cargo)

## Contributing

- Run `npm run build` to run TypeScript checks and build the frontend assets
- If you change Rust code, `cd src-tauri && cargo build` to check the Rust build
- For quick debugging, check console output where the Tauri app runs — the embedded server prints helpful logs during ingestion and summary generation

If you want, I can extend this README with troubleshooting steps for Tauri build errors or add a small script to send the sample ingestion commands.

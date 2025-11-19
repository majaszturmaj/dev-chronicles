# Ingestion Server: Sample Requests

Use these curl commands to test the embedded ingestion server at http://localhost:3030. Each command posts a JSON payload to the corresponding `/ingest/*` route.

Note: these examples assume the server is running locally and listening on port 3030.

---

## Browser examples

Browser 1 — page visit

```bash
curl -sS -X POST http://localhost:3030/ingest/browser \
  -H "Content-Type: application/json" \
  -d '{
    "source": "browser",
    "payload": {
      "url": "https://docs.rs/tokio",
      "title": "Tokio Documentation",
      "time_on_page_sec": 120,
      "referrer": "https://google.com",
      "user_agent": "Mozilla/5.0 (X11; Linux x86_64)"
    }
  }'
```

Browser 2 — form submit

```bash
curl -sS -X POST http://localhost:3030/ingest/browser \
  -H "Content-Type: application/json" \
  -d '{
    "source": "browser",
    "payload": {
      "url": "https://example.com/signup",
      "title": "Sign Up",
      "time_on_page_sec": 45,
      "form_fields": ["email", "name"],
      "success": true
    }
  }'
```

Browser 3 — error / console event

```bash
curl -sS -X POST http://localhost:3030/ingest/browser \
  -H "Content-Type: application/json" \
  -d '{
    "source": "browser",
    "payload": {
      "url": "https://app.example.com/dashboard",
      "title": "Dashboard",
      "time_on_page_sec": 5,
      "event": "console_error",
      "message": "ReferenceError: x is not defined",
      "line": 234
    }
  }'
```

---

## Terminal examples

Terminal 1 — simple command run (success)

```bash
curl -sS -X POST http://localhost:3030/ingest/terminal \
  -H "Content-Type: application/json" \
  -d '{
    "source": "terminal",
    "payload": {
      "command": "git status",
      "exit_code": 0,
      "duration_sec": 0.12,
      "cwd": "/home/maja/projects/dev-chronicle"
    }
  }'
```

Terminal 2 — long-running build

```bash
curl -sS -X POST http://localhost:3030/ingest/terminal \
  -H "Content-Type: application/json" \
  -d '{
    "source": "terminal",
    "payload": {
      "command": "cargo build --release",
      "exit_code": 0,
      "duration_sec": 142.4,
      "cwd": "/home/maja/projects/myapp",
      "stdout_lines": 120,
      "stderr_lines": 2
    }
  }'
```

Terminal 3 — failing script

```bash
curl -sS -X POST http://localhost:3030/ingest/terminal \
  -H "Content-Type: application/json" \
  -d '{
    "source": "terminal",
    "payload": {
      "command": "./deploy.sh",
      "exit_code": 2,
      "duration_sec": 3.7,
      "cwd": "/home/maja/scripts",
      "error": "failed to push to remote"
    }
  }'
```

---

## VSCode examples

VSCode 1 — file open

```bash
curl -sS -X POST http://localhost:3030/ingest/vscode \
  -H "Content-Type: application/json" \
  -d '{
    "source": "vscode",
    "payload": {
      "event": "file_open",
      "file": "/home/maja/projects/dev-chronicle/src/main.rs",
      "language": "rust",
      "workspace": "/home/maja/projects/dev-chronicle",
      "cursor_line": 12,
      "cursor_col": 4
    }
  }'
```

VSCode 2 — file save

```bash
curl -sS -X POST http://localhost:3030/ingest/vscode \
  -H "Content-Type: application/json" \
  -d '{
    "source": "vscode",
    "payload": {
      "event": "file_save",
      "file": "/home/maja/projects/dev-chronicle/README.md",
      "language": "markdown",
      "workspace": "/home/maja/projects/dev-chronicle",
      "changes": 3,
      "time_spent_sec": 18.6
    }
  }'
```

VSCode 3 — extension event (e.g. lint)

```bash
curl -sS -X POST http://localhost:3030/ingest/vscode \
  -H "Content-Type: application/json" \
  -d '{
    "source": "vscode",
    "payload": {
      "event": "linter_run",
      "file": "/home/maja/projects/dev-chronicle/src/lib.rs",
      "warnings": 2,
      "errors": 0,
      "tool": "rust-analyzer"
    }
  }'
```

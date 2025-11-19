# DevChronicle Browser Logger

Manifest V3 browser extension that captures active tab activity and pushes it to the DevChronicle desktop hub.

## What it does

- Watches for tab activation or completed loads.
- Waits 5 seconds to ensure the page is in focus.
- Sends a POST request to `http://localhost:3030/ingest` with the URL, title, and timestamp.

## Load as an unpacked extension

1. Open Chrome (or Edge).
2. Navigate to `chrome://extensions/` and enable **Developer Mode**.
3. Click **Load unpacked** and select this folder (`extensions/browser-logger`).
4. Ensure the DevChronicle desktop app is running so the ingest endpoint accepts events.

## Customisation

- The ingest endpoint defaults to `http://localhost:3030`. Adjust `DEFAULT_ENDPOINT` in `background.js` if your desktop app uses a different port.
- Icons (`icon48.png`, `icon128.png`) can be replaced with your own branding.

## Future ideas

- Add popup UI for toggling the logger.
- Persist dwell time thresholds.
- Filter out noise such as internal `chrome://` URLs.


# DevChronicle Extensions Setup Guide

This guide explains how to set up the three DevChronicle extensions for Chrome browser, terminal (bash/zsh), and VS Code on Ubuntu.

## Prerequisites

1. **DevChronicle Desktop App**: Make sure the DevChronicle desktop application is running. The ingestion server listens on `http://localhost:3030`.

2. **Test the server**: Verify the server is running:
   ```bash
   curl http://127.0.0.1:3030/health
   # Should return: OK
   ```

---

## 1. Chrome Browser Extension

### Installation Steps

1. **Open Chrome Extensions Page**:
   - Navigate to `chrome://extensions/`
   - Or: Menu (☰) → Extensions → Manage Extensions

2. **Enable Developer Mode**:
   - Toggle "Developer mode" switch in the top-right corner

3. **Load the Extension**:
   - Click "Load unpacked"
   - Navigate to: `/path/to/dev-chronicles/extensions/browser-logger/`
   - Select the folder and click "Select"

4. **Verify Installation**:
   - The extension should appear in your extensions list
   - Navigate to any website and wait 5 seconds
   - Check the browser console (F12) for any errors

### What It Does

- Captures page visits after 5 seconds of viewing
- Sends URL, title, time on page, referrer, and user agent to DevChronicle
- Endpoint: `POST http://localhost:3030/ingest/browser`

### Troubleshooting

- If events aren't being sent, check Chrome's extension console (click "service worker" link)
- Ensure the DevChronicle app is running
- Check that `localhost:3030` is accessible

---

## 2. Terminal Extension (Bash/Zsh)

### Installation Steps

#### For Bash

1. **Install Dependencies** (if not already installed):
   ```bash
   sudo apt-get install curl jq bc
   ```

2. **Add to `~/.bashrc`**:
   ```bash
   echo "" >> ~/.bashrc
   echo "# DevChronicle Terminal Logger" >> ~/.bashrc
   echo "export DEVCHRONICLE_ENDPOINT=\"http://localhost:3030\"" >> ~/.bashrc
   echo "export DEVCHRONICLE_ENABLED=1" >> ~/.bashrc
   echo "source /path/to/dev-chronicles/extensions/terminal-logger/dev-chronicle-hook.sh" >> ~/.bashrc
   ```
   
   **Important**: Replace `/path/to/dev-chronicles` with the actual path to your dev-chronicles directory.

3. **Reload Bash Configuration**:
   ```bash
   source ~/.bashrc
   ```
   
   Or open a new terminal window.

#### For Zsh

1. **Install Dependencies** (if not already installed):
   ```bash
   sudo apt-get install curl jq bc
   ```

2. **Add to `~/.zshrc`**:
   ```bash
   echo "" >> ~/.zshrc
   echo "# DevChronicle Terminal Logger" >> ~/.zshrc
   echo "export DEVCHRONICLE_ENDPOINT=\"http://localhost:3030\"" >> ~/.zshrc
   echo "export DEVCHRONICLE_ENABLED=1" >> ~/.zshrc
   echo "source /path/to/dev-chronicles/extensions/terminal-logger/dev-chronicle-hook.sh" >> ~/.zshrc
   ```
   
   **Important**: Replace `/path/to/dev-chronicles` with the actual path to your dev-chronicles directory.

3. **Reload Zsh Configuration**:
   ```bash
   source ~/.zshrc
   ```
   
   Or open a new terminal window.

### What It Does

- Captures every command you run in the terminal
- Sends command, exit code, duration, and working directory to DevChronicle
- Endpoint: `POST http://localhost:3030/ingest/terminal`

### Configuration

You can disable the logger temporarily:
```bash
export DEVCHRONICLE_ENABLED=0
```

Or change the endpoint:
```bash
export DEVCHRONICLE_ENABLED="http://your-server:3030"
```

### Troubleshooting

- Commands like `history`, `cd`, `pwd`, `exit`, `clear` are skipped
- Check that `curl`, `jq`, and `bc` are installed
- Verify the hook script path is correct
- Test manually: `curl http://127.0.0.1:3030/health`

---

## 3. VS Code Extension

### Installation Steps

1. **Build the Extension**:
   ```bash
   cd /path/to/dev-chronicles/extensions/vscode-logger
   npm install
   npm run compile
   ```
   
   This will compile TypeScript to JavaScript in the `out/` directory.

2. **Install in VS Code**:
   - Open VS Code
   - Press `F1` or `Ctrl+Shift+P` to open command palette
   - Type: "Extensions: Install from VSIX..."
   - **OR** use the developer method:
     - Press `F5` to open Extension Development Host
     - Or: Menu → Run → Start Debugging
   - **OR** copy the extension folder to VS Code extensions directory:
     ```bash
     cp -r /path/to/dev-chronicles/extensions/vscode-logger ~/.vscode/extensions/devchronicle-logger
     ```
     Then restart VS Code

3. **Configure Endpoint** (optional):
   - Open VS Code Settings (`Ctrl+,`)
   - Search for "DevChronicle"
   - Set `devChronicle.endpoint` to `http://localhost:3030` (default)

### What It Does

- Captures file open events (when you open a file)
- Captures file save events (when you save a file)
- Sends file path, language, workspace, and time spent to DevChronicle
- Endpoint: `POST http://localhost:3030/ingest/vscode`

### Troubleshooting

- Check VS Code Developer Console: Help → Toggle Developer Tools
- Ensure the extension is activated (check Output panel → "DevChronicle")
- Verify the DevChronicle app is running
- Check that TypeScript compiled successfully (`out/extension.js` exists)

---

## Testing All Extensions

Run the test script to verify all endpoints are working:

```bash
cd /path/to/dev-chronicles/extensions
./test-extensions.sh
```

This will test:
1. Health endpoint
2. Browser endpoint
3. Terminal endpoint
4. VS Code endpoint

---

## Quick Verification

After setup, verify each extension:

1. **Browser**: Navigate to a website, wait 5 seconds, check DevChronicle dashboard
2. **Terminal**: Run any command (e.g., `ls`), check DevChronicle dashboard
3. **VS Code**: Open and save a file, check DevChronicle dashboard

---

## Troubleshooting Common Issues

### Server Not Running
- Start the DevChronicle desktop app first
- Verify: `curl http://127.0.0.1:3030/health`

### Extension Not Sending Events
- Check browser/VS Code console for errors
- Verify endpoint URL is correct
- Ensure CORS is enabled (should be by default)

### Terminal Hook Not Working
- Verify the hook script is executable: `chmod +x dev-chronicle-hook.sh`
- Check that dependencies are installed: `which curl jq bc`
- Test the hook manually by sourcing it in a terminal

### VS Code Extension Not Loading
- Ensure TypeScript is compiled: `npm run compile`
- Check that `out/extension.js` exists
- Verify VS Code version is 1.84.0 or higher

---

## Uninstallation

### Browser Extension
- Go to `chrome://extensions/`
- Find "DevChronicle Browser Logger"
- Click "Remove"

### Terminal Extension
- Remove the lines added to `~/.bashrc` or `~/.zshrc`
- Reload your shell configuration

### VS Code Extension
- Remove from `~/.vscode/extensions/devchronicle-logger`
- Restart VS Code


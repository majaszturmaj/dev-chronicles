# DevChronicle VS Code Logger

This lightweight VS Code extension streams save events to the DevChronicle desktop hub so they can be ingested into the activity timeline.

## Features

- Automatically POSTs to `http://localhost:3030/ingest` (configurable) whenever you save a file.
- Sends the file path, language ID, workspace folder and a timestamp.

## Setup

1. Install dependencies:

   ```bash
   npm install
   ```

2. Build (or watch):

   ```bash
   npm run compile
   # or
   npm run watch
   ```

3. Press `F5` in VS Code to launch the extension in a new Extension Development Host window.

## Configuration

`DevChronicle Logger: Endpoint` (setting ID `devChronicle.endpoint`) controls the ingest server URL. Leave it at the default (`http://localhost:3030`) to target the running DevChronicle desktop app.

## Packaging

Use [`vsce`](https://code.visualstudio.com/api/working-with-extensions/publishing-extension) if you want to package the extension for distribution:

```bash
npm install -g @vscode/vsce
vsce package
```


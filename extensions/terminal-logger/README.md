# DevChronicle Terminal Logger

Bash/Zsh hook script that captures terminal commands and sends them to the DevChronicle desktop app.

## Features

- Captures command, exit code, duration, and working directory
- Works with both bash and zsh
- Non-blocking (sends requests in background)
- Configurable via environment variables

## Setup

### For Bash

Add to your `~/.bashrc`:

```bash
source /path/to/dev-chronicles/extensions/terminal-logger/dev-chronicle-hook.sh
```

### For Zsh

Add to your `~/.zshrc`:

```bash
source /path/to/dev-chronicles/extensions/terminal-logger/dev-chronicle-hook.sh
```

## Configuration

Set environment variables before sourcing the script:

- `DEVCHRONICLE_ENDPOINT`: API endpoint (default: `http://localhost:3030`)
- `DEVCHRONICLE_ENABLED`: Enable/disable logging (default: `1`, set to `0` to disable)

Example:

```bash
export DEVCHRONICLE_ENDPOINT="http://localhost:3030"
export DEVCHRONICLE_ENABLED=1
source /path/to/dev-chronicles/extensions/terminal-logger/dev-chronicle-hook.sh
```

## Requirements

- `curl` - for sending HTTP requests
- `jq` - for JSON encoding
- `bc` - for floating point calculations

Install on Ubuntu:

```bash
sudo apt-get install curl jq bc
```


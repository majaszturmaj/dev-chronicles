#!/bin/bash
# Test script for DevChronicle extensions
# Make sure the DevChronicle app is running before executing this script

ENDPOINT="${DEVCHRONICLE_ENDPOINT:-http://localhost:3030}"

echo "Testing DevChronicle Extensions"
echo "================================"
echo ""

# Test health endpoint
echo "1. Testing health endpoint..."
if curl -sS "$ENDPOINT/health" | grep -q "OK"; then
    echo "   ✅ Health check passed"
else
    echo "   ❌ Health check failed - is the DevChronicle app running?"
    exit 1
fi
echo ""

# Test browser endpoint
echo "2. Testing browser endpoint..."
BROWSER_RESPONSE=$(curl -sS -w "\n%{http_code}" -X POST "$ENDPOINT/ingest/browser" \
    -H "Content-Type: application/json" \
    -d '{
      "source": "browser",
      "payload": {
        "url": "https://example.com/test",
        "title": "Test Page",
        "time_on_page_sec": 10,
        "user_agent": "Mozilla/5.0 (Test)"
      }
    }')
HTTP_CODE=$(echo "$BROWSER_RESPONSE" | tail -n1)
if [ "$HTTP_CODE" = "201" ]; then
    echo "   ✅ Browser endpoint test passed"
else
    echo "   ❌ Browser endpoint test failed (HTTP $HTTP_CODE)"
fi
echo ""

# Test terminal endpoint
echo "3. Testing terminal endpoint..."
TERMINAL_RESPONSE=$(curl -sS -w "\n%{http_code}" -X POST "$ENDPOINT/ingest/terminal" \
    -H "Content-Type: application/json" \
    -d '{
      "source": "terminal",
      "payload": {
        "command": "echo test",
        "exit_code": 0,
        "duration_sec": 0.1,
        "cwd": "/tmp"
      }
    }')
HTTP_CODE=$(echo "$TERMINAL_RESPONSE" | tail -n1)
if [ "$HTTP_CODE" = "201" ]; then
    echo "   ✅ Terminal endpoint test passed"
else
    echo "   ❌ Terminal endpoint test failed (HTTP $HTTP_CODE)"
fi
echo ""

# Test vscode endpoint
echo "4. Testing VS Code endpoint..."
VSCODE_RESPONSE=$(curl -sS -w "\n%{http_code}" -X POST "$ENDPOINT/ingest/vscode" \
    -H "Content-Type: application/json" \
    -d '{
      "source": "vscode",
      "payload": {
        "event": "file_save",
        "file": "/tmp/test.rs",
        "language": "rust",
        "workspace": "/tmp"
      }
    }')
HTTP_CODE=$(echo "$VSCODE_RESPONSE" | tail -n1)
if [ "$HTTP_CODE" = "201" ]; then
    echo "   ✅ VS Code endpoint test passed"
else
    echo "   ❌ VS Code endpoint test failed (HTTP $HTTP_CODE)"
fi
echo ""

echo "================================"
echo "Extension endpoint tests complete!"
echo ""
echo "Note: To test the actual extensions:"
echo "  1. Browser: Load the extension in Chrome and navigate to a page"
echo "  2. Terminal: Source the hook script and run a command"
echo "  3. VS Code: Install the extension and save a file"


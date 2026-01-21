#!/bin/bash
# Test script for web version of DevChronicle

echo "=========================================="
echo "  DevChronicle Web Version Test"
echo "=========================================="
echo ""

# Check if Vite is running
if curl -s http://localhost:5173 > /dev/null 2>&1; then
    echo "✓ Vite dev server is running"
    echo "  URL: http://localhost:5173"
    echo ""
    echo "Open your browser and navigate to: http://localhost:5173"
    echo ""
    echo "Note: The web version shows the frontend UI, but full functionality"
    echo "      (database access, AI features) requires the Tauri backend."
    echo ""
    echo "To test with full backend:"
    echo "  1. Stop Vite (Ctrl+C or kill the process)"
    echo "  2. Run: npm run tauri:dev"
    echo ""
else
    echo "✗ Vite dev server is not running"
    echo ""
    echo "Starting Vite dev server..."
    cd "$(dirname "$0")"
    npm run dev &
    VITE_PID=$!
    echo "Vite started (PID: $VITE_PID)"
    echo "Waiting for server to be ready..."
    
    for i in {1..10}; do
        sleep 1
        if curl -s http://localhost:5173 > /dev/null 2>&1; then
            echo "✓ Vite is ready!"
            echo "  URL: http://localhost:5173"
            break
        fi
        echo -n "."
    done
    echo ""
fi

# Check API key status
echo "API Key Status:"
python3 "$(dirname "$0")/update_api_key.py" 2>&1 | grep -E "(Provider URL|API Key|Model)" | head -3
echo ""


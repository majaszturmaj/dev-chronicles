#!/bin/bash
# Quick test script to verify the app starts correctly

set -e

echo "Testing DevChronicle application startup..."
echo ""

# Check if already running
if curl -sS http://localhost:3030/health > /dev/null 2>&1; then
    echo "✓ Backend server is already running"
else
    echo "Starting application..."
    
    # Start in background
    npm run tauri:dev > /tmp/tauri-dev.log 2>&1 &
    TAURI_PID=$!
    
    echo "Tauri process started (PID: $TAURI_PID)"
    echo "Waiting for services to start..."
    
    # Wait for Vite
    echo -n "Waiting for Vite dev server"
    for i in {1..60}; do
        if curl -sS http://localhost:5173 > /dev/null 2>&1; then
            echo " ✓"
            echo "✓ Vite dev server is ready"
            break
        fi
        echo -n "."
        sleep 1
    done
    
    # Wait for backend
    echo -n "Waiting for backend server"
    for i in {1..45}; do
        if curl -sS http://localhost:3030/health > /dev/null 2>&1; then
            echo " ✓"
            echo "✓ Backend server is ready"
            break
        fi
        echo -n "."
        sleep 1
    done
    
    echo ""
    echo "Application should be running now!"
    echo "Check the DevChronicle window - it should show the UI"
    echo ""
    echo "To stop: kill $TAURI_PID"
    echo "Logs: tail -f /tmp/tauri-dev.log"
fi


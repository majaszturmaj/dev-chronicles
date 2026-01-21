#!/bin/bash
# DevChronicle - All-in-One Run Script
# Starts the entire app: Vite frontend + Tauri backend + Axum ingestion server

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "🚀 DevChronicle - Starting Application"
echo "======================================"
echo ""

# Cleanup function for graceful shutdown
cleanup() {
    echo ""
    echo "🛑 Shutting down DevChronicle..."
    
    # Kill any process using port 5173 (Vite)
    if lsof -ti:5173 >/dev/null 2>&1; then
        echo "  Cleaning up port 5173..."
        lsof -ti:5173 | xargs kill -9 2>/dev/null || true
    fi
    
    # Kill the main Tauri process
    if [ ! -z "$TAURI_PID" ]; then
        kill $TAURI_PID 2>/dev/null || true
    fi
    
    echo "✅ Cleanup complete"
    exit 0
}

# Register cleanup on exit
trap cleanup EXIT INT TERM

# Check if port 5173 is already in use
if lsof -ti:5173 >/dev/null 2>&1; then
    echo "⚠️  Port 5173 is already in use. Freeing it..."
    lsof -ti:5173 | xargs kill -9 2>/dev/null || true
    sleep 1
fi

echo "✓ Dependencies check..."
if ! command -v npm &> /dev/null; then
    echo "❌ npm not found. Please install Node.js"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ cargo not found. Please install Rust"
    exit 1
fi

echo "✓ npm: $(npm --version)"
echo "✓ cargo: $(cargo --version)"
echo ""

# Start Tauri dev (which handles Vite automatically via beforeDevCommand)
echo "📦 Starting DevChronicle..."
echo "   Frontend: http://localhost:5173"
echo "   Backend:  http://127.0.0.1:3030"
echo ""
echo "Press Ctrl+C to stop"
echo ""

npm run tauri:dev &
TAURI_PID=$!

# Wait for the process to complete
wait $TAURI_PID 2>/dev/null || true

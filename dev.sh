#!/bin/bash
# DevChronicle Helper Commands

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

case "${1:-}" in
    run)
        echo "Starting DevChronicle..."
        ./run.sh
        ;;
    build)
        echo "Building DevChronicle..."
        npm run build
        echo "✅ Build complete"
        ;;
    build:app)
        echo "Building Tauri app package..."
        npm run tauri:build
        echo "✅ App build complete"
        ;;
    clean)
        echo "Cleaning build artifacts..."
        rm -rf dist
        rm -rf target
        cargo clean 2>/dev/null || true
        echo "✅ Cleaned"
        ;;
    test-health)
        echo "Testing backend health..."
        curl -s http://127.0.0.1:3030/health && echo "" || echo "Backend not running"
        ;;
    test-extensions)
        echo "Testing all extension endpoints..."
        cd extensions && bash test-extensions.sh
        ;;
    logs)
        echo "Showing logs for today..."
        curl -s http://127.0.0.1:3030/ingest/browser -X POST \
            -H "Content-Type: application/json" \
            -d '{"source":"test","payload":{"test":true}}' 2>&1 || true
        ;;
    *)
        echo "DevChronicle Helper"
        echo ""
        echo "Usage: ./dev.sh [command]"
        echo ""
        echo "Commands:"
        echo "  run              Start the app (default)"
        echo "  build            Build frontend only"
        echo "  build:app        Build Tauri app package"
        echo "  clean            Remove build artifacts"
        echo "  test-health      Test backend health endpoint"
        echo "  test-extensions  Test all extension endpoints"
        echo ""
        echo "Examples:"
        echo "  ./dev.sh run"
        echo "  ./dev.sh build"
        echo "  ./dev.sh test-extensions"
        ;;
esac

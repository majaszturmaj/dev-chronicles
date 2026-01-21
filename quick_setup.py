#!/usr/bin/env python3
"""
DevChronicle Quick Setup and Test Script

This script:
1. Sets up all extensions (terminal hook, VS Code compilation)
2. Starts the DevChronicle application
3. Sends 20 realistic "building Yocto Linux" workflow events to test AI summarization
"""

import os
import sys
import json
import time
import subprocess
import shutil
from pathlib import Path
from datetime import datetime, timedelta
from typing import Dict, List, Optional

try:
    import requests
    HAS_REQUESTS = True
except ImportError:
    HAS_REQUESTS = False
    print("Warning: 'requests' library not found. Install with: pip install requests")

# Colors for terminal output
class Colors:
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    RED = '\033[91m'
    BLUE = '\033[94m'
    CYAN = '\033[96m'
    RESET = '\033[0m'
    BOLD = '\033[1m'

def print_step(msg: str):
    print(f"{Colors.CYAN}{Colors.BOLD}▶ {msg}{Colors.RESET}")

def print_success(msg: str):
    print(f"{Colors.GREEN}✓ {msg}{Colors.RESET}")

def print_warning(msg: str):
    print(f"{Colors.YELLOW}⚠ {msg}{Colors.RESET}")

def print_error(msg: str):
    print(f"{Colors.RED}✗ {msg}{Colors.RESET}")

def print_info(msg: str):
    print(f"{Colors.BLUE}ℹ {msg}{Colors.RESET}")

# Get project root directory
PROJECT_ROOT = Path(__file__).parent.absolute()
ENDPOINT = "http://localhost:3030"

def check_dependencies() -> bool:
    """Check if required dependencies are installed."""
    print_step("Checking dependencies...")
    
    deps = {
        'curl': shutil.which('curl'),
        'jq': shutil.which('jq'),
        'bc': shutil.which('bc'),
        'node': shutil.which('node'),
        'npm': shutil.which('npm'),
        'cargo': shutil.which('cargo'),
    }
    
    missing = [name for name, path in deps.items() if not path]
    
    if missing:
        print_error(f"Missing dependencies: {', '.join(missing)}")
        print_info("Install missing dependencies:")
        print_info("  sudo apt-get install curl jq bc")
        print_info("  Install Node.js and Rust from their official websites")
        return False
    
    if not HAS_REQUESTS:
        print_warning("Python 'requests' library not found (optional)")
        print_info("Install with: pip install requests (or pip3 install requests)")
        print_info("Script will use curl as fallback")
    else:
        print_success("Python 'requests' library found")
    
    print_success("All required dependencies found")
    return True

def setup_terminal_extension() -> bool:
    """Set up terminal extension hook."""
    print_step("Setting up terminal extension...")
    
    hook_script = PROJECT_ROOT / "extensions" / "terminal-logger" / "dev-chronicle-hook.sh"
    if not hook_script.exists():
        print_error(f"Hook script not found: {hook_script}")
        return False
    
    # Detect shell
    shell = os.environ.get('SHELL', '/bin/bash')
    shell_rc = Path.home() / ('.zshrc' if 'zsh' in shell else '.bashrc')
    
    # Check if already installed
    try:
        with open(shell_rc, 'r') as f:
            if 'dev-chronicle-hook.sh' in f.read():
                print_success("Terminal hook already installed")
                return True
    except FileNotFoundError:
        pass
    
    # Add hook to shell rc
    hook_line = f'\nsource {hook_script}\n'
    
    try:
        with open(shell_rc, 'a') as f:
            f.write(f"\n# DevChronicle Terminal Logger\n")
            f.write(f"export DEVCHRONICLE_ENDPOINT=\"{ENDPOINT}\"\n")
            f.write(f"export DEVCHRONICLE_ENABLED=1\n")
            f.write(f"source {hook_script}\n")
        
        print_success(f"Terminal hook added to {shell_rc}")
        print_warning(f"Run 'source {shell_rc}' or restart your terminal to activate")
        return True
    except Exception as e:
        print_error(f"Failed to add terminal hook: {e}")
        return False

def setup_vscode_extension() -> bool:
    """Compile VS Code extension."""
    print_step("Setting up VS Code extension...")
    
    vscode_dir = PROJECT_ROOT / "extensions" / "vscode-logger"
    if not vscode_dir.exists():
        print_error(f"VS Code extension directory not found: {vscode_dir}")
        return False
    
    # Check if node_modules exists
    node_modules = vscode_dir / "node_modules"
    if not node_modules.exists():
        print_info("Installing VS Code extension dependencies...")
        try:
            subprocess.run(
                ["npm", "install"],
                cwd=vscode_dir,
                check=True,
                capture_output=True
            )
            print_success("VS Code extension dependencies installed")
        except subprocess.CalledProcessError as e:
            print_error(f"Failed to install dependencies: {e}")
            return False
    
    # Compile TypeScript
    print_info("Compiling VS Code extension...")
    try:
        result = subprocess.run(
            ["npm", "run", "compile"],
            cwd=vscode_dir,
            check=True,
            capture_output=True,
            text=True
        )
        print_success("VS Code extension compiled successfully")
        return True
    except subprocess.CalledProcessError as e:
        print_error(f"Failed to compile VS Code extension: {e}")
        print_info(f"Output: {e.stdout}\n{e.stderr}")
        return False

def check_chrome_extension() -> bool:
    """Check if Chrome extension can be loaded."""
    print_step("Checking Chrome extension...")
    
    browser_ext_dir = PROJECT_ROOT / "extensions" / "browser-logger"
    manifest = browser_ext_dir / "manifest.json"
    
    if not manifest.exists():
        print_error(f"Chrome extension manifest not found: {manifest}")
        return False
    
    print_success("Chrome extension files found")
    print_warning("Manual setup required:")
    print_info("  1. Open Chrome and go to chrome://extensions/")
    print_info("  2. Enable 'Developer mode'")
    print_info(f"  3. Click 'Load unpacked' and select: {browser_ext_dir}")
    return True

def wait_for_vite(timeout: int = 30) -> bool:
    """Wait for Vite dev server to be ready."""
    print_step(f"Waiting for Vite dev server at http://localhost:5173...")
    
    start_time = time.time()
    while time.time() - start_time < timeout:
        if HAS_REQUESTS:
            try:
                response = requests.get("http://localhost:5173", timeout=2)
                if response.status_code == 200:
                    print_success("Vite dev server is ready!")
                    return True
            except requests.exceptions.RequestException:
                pass
        else:
            # Fallback to curl
            try:
                result = subprocess.run(
                    ["curl", "-sS", "-o", "/dev/null", "-w", "%{http_code}", "http://localhost:5173"],
                    capture_output=True,
                    text=True,
                    timeout=2
                )
                if result.returncode == 0 and result.stdout.strip() == "200":
                    print_success("Vite dev server is ready!")
                    return True
            except (subprocess.TimeoutExpired, FileNotFoundError):
                pass
        
        time.sleep(1)
        print(".", end="", flush=True)
    
    print()
    print_error(f"Vite dev server not ready after {timeout} seconds")
    return False

def wait_for_server(timeout: int = 30) -> bool:
    """Wait for the DevChronicle server to be ready."""
    print_step(f"Waiting for server at {ENDPOINT}...")
    
    start_time = time.time()
    while time.time() - start_time < timeout:
        if HAS_REQUESTS:
            try:
                response = requests.get(f"{ENDPOINT}/health", timeout=2)
                if response.status_code == 200 and response.text.strip() == "OK":
                    print_success("Server is ready!")
                    return True
            except requests.exceptions.RequestException:
                pass
        else:
            # Fallback to curl
            try:
                result = subprocess.run(
                    ["curl", "-sS", f"{ENDPOINT}/health"],
                    capture_output=True,
                    text=True,
                    timeout=2
                )
                if result.returncode == 0 and result.stdout.strip() == "OK":
                    print_success("Server is ready!")
                    return True
            except (subprocess.TimeoutExpired, FileNotFoundError):
                pass
        
        time.sleep(1)
        print(".", end="", flush=True)
    
    print()
    print_error(f"Server not ready after {timeout} seconds")
    return False

def check_vite_running() -> bool:
    """Check if Vite dev server is already running."""
    if HAS_REQUESTS:
        try:
            response = requests.get("http://localhost:5173", timeout=2)
            if response.status_code == 200:
                return True
        except requests.exceptions.RequestException:
            pass
    else:
        try:
            result = subprocess.run(
                ["curl", "-sS", "-o", "/dev/null", "-w", "%{http_code}", "http://localhost:5173"],
                capture_output=True,
                timeout=2
            )
            if result.returncode == 0 and result.stdout.strip() == "200":
                return True
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass
    return False

def kill_process_on_port(port: int) -> bool:
    """Kill any process using the specified port."""
    try:
        # Find process using the port
        result = subprocess.run(
            ["lsof", "-ti", f":{port}"],
            capture_output=True,
            text=True,
            timeout=5
        )
        
        if result.returncode == 0 and result.stdout.strip():
            pids = result.stdout.strip().split('\n')
            for pid in pids:
                if pid:
                    try:
                        subprocess.run(["kill", "-9", pid], check=True, timeout=5)
                        print_info(f"Killed process {pid} using port {port}")
                    except subprocess.CalledProcessError:
                        pass
            return True
    except (subprocess.CalledProcessError, FileNotFoundError, subprocess.TimeoutExpired):
        # lsof might not be available or no process found
        pass
    
    # Fallback: try using fuser if lsof is not available
    try:
        result = subprocess.run(
            ["fuser", "-k", f"{port}/tcp"],
            capture_output=True,
            timeout=5
        )
        if result.returncode == 0:
            print_info(f"Freed port {port} using fuser")
            return True
    except (subprocess.CalledProcessError, FileNotFoundError, subprocess.TimeoutExpired):
        pass
    
    return False

def start_app() -> Optional[subprocess.Popen]:
    """Start the DevChronicle application."""
    print_step("Starting DevChronicle application...")
    
    # Check if already running
    if HAS_REQUESTS:
        try:
            response = requests.get(f"{ENDPOINT}/health", timeout=2)
            if response.status_code == 200:
                print_success("Application is already running")
                return None
        except requests.exceptions.RequestException:
            pass
    else:
        # Fallback to curl
        try:
            result = subprocess.run(
                ["curl", "-sS", f"{ENDPOINT}/health"],
                capture_output=True,
                timeout=2
            )
            if result.returncode == 0:
                print_success("Application is already running")
                return None
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass
    
    # Start the app
    try:
        # First ensure npm dependencies are installed
        if not (PROJECT_ROOT / "node_modules").exists():
            print_info("Installing npm dependencies...")
            subprocess.run(
                ["npm", "install"],
                cwd=PROJECT_ROOT,
                check=True,
                capture_output=True
            )
        
        # Check if port 3030 is in use (backend server)
        try:
            result = subprocess.run(
                ["lsof", "-ti", ":3030"],
                capture_output=True,
                timeout=2
            )
            if result.returncode == 0 and result.stdout.strip():
                print_warning("Port 3030 is already in use")
                print_info("Another instance of DevChronicle may be running")
                print_info("Attempting to free the port...")
                if kill_process_on_port(3030):
                    print_success("Port 3030 freed")
                    time.sleep(2)  # Wait for port to be fully released
                else:
                    print_warning("Could not free port 3030")
                    print_info("The application may fail to start")
        except FileNotFoundError:
            # Try fuser as fallback
            try:
                result = subprocess.run(
                    ["fuser", f"3030/tcp"],
                    capture_output=True,
                    timeout=2
                )
                if result.returncode == 0:
                    print_warning("Port 3030 is in use")
                    if kill_process_on_port(3030):
                        print_success("Port 3030 freed")
                        time.sleep(2)
            except FileNotFoundError:
                pass
        except subprocess.TimeoutExpired:
            pass
        
        # Start Tauri app in background
        # Tauri's beforeDevCommand will start Vite automatically
        print_info("Starting Tauri application (this may take a moment)...")
        print_info("Tauri will automatically start the Vite dev server")
        
        # Don't pipe stdout/stderr so we can see what's happening
        # But we'll still run it in background
        process = subprocess.Popen(
            ["npm", "run", "tauri:dev"],
            cwd=PROJECT_ROOT,
            # Don't pipe output so Tauri can display properly
            # stdout=subprocess.PIPE,
            # stderr=subprocess.PIPE,
            text=True
        )
        
        print_success("Tauri process started")
        print_info("Waiting for application to initialize...")
        return process
            
    except Exception as e:
        print_error(f"Failed to start application: {e}")
        return None

def send_event(endpoint: str, payload: Dict) -> bool:
    """Send an event to the ingestion server."""
    if HAS_REQUESTS:
        try:
            response = requests.post(
                f"{ENDPOINT}{endpoint}",
                json=payload,
                headers={"Content-Type": "application/json"},
                timeout=5
            )
            return response.status_code == 201
        except Exception as e:
            print_error(f"Failed to send event: {e}")
            return False
    else:
        # Fallback to curl
        try:
            json_data = json.dumps(payload)
            result = subprocess.run(
                ["curl", "-sS", "-X", "POST", f"{ENDPOINT}{endpoint}",
                 "-H", "Content-Type: application/json",
                 "-d", json_data],
                capture_output=True,
                timeout=5
            )
            return result.returncode == 0
        except Exception as e:
            print_error(f"Failed to send event: {e}")
            return False

def generate_yocto_workflow() -> List[Dict]:
    """Generate 20 realistic Yocto Linux build workflow events."""
    
    base_time = datetime.now() - timedelta(minutes=45)
    workspace = "/home/dev/yocto-build"
    
    events = []
    
    # Terminal events
    terminal_events = [
        {"command": "cd ~/yocto-build", "exit_code": 0, "duration": 0.1, "cwd": str(Path.home())},
        {"command": "git clone https://git.yoctoproject.org/poky", "exit_code": 0, "duration": 45.2, "cwd": workspace},
        {"command": "cd poky", "exit_code": 0, "duration": 0.1, "cwd": workspace},
        {"command": "git checkout kirkstone", "exit_code": 0, "duration": 2.3, "cwd": f"{workspace}/poky"},
        {"command": "source oe-init-build-env build", "exit_code": 0, "duration": 1.5, "cwd": f"{workspace}/poky"},
        {"command": "bitbake-layers show-layers", "exit_code": 0, "duration": 3.2, "cwd": f"{workspace}/poky/build"},
        {"command": "bitbake core-image-minimal", "exit_code": 1, "duration": 120.5, "cwd": f"{workspace}/poky/build", "error": "ERROR: No space left on device"},
        {"command": "df -h", "exit_code": 0, "duration": 0.3, "cwd": f"{workspace}/poky/build"},
        {"command": "sudo apt-get clean", "exit_code": 0, "duration": 5.1, "cwd": f"{workspace}/poky/build"},
        {"command": "bitbake core-image-minimal", "exit_code": 0, "duration": 892.4, "cwd": f"{workspace}/poky/build"},
        {"command": "ls -lh tmp/deploy/images/qemux86-64/", "exit_code": 0, "duration": 0.2, "cwd": f"{workspace}/poky/build"},
    ]
    
    # Browser events
    browser_events = [
        {"url": "https://docs.yoctoproject.org/", "title": "Yocto Project Documentation", "time_on_page": 180},
        {"url": "https://docs.yoctoproject.org/dev-manual/common-tasks.html", "title": "Common Tasks - Yocto Project", "time_on_page": 240},
        {"url": "https://stackoverflow.com/questions/yocto-build-error", "title": "Yocto build error - Stack Overflow", "time_on_page": 120},
        {"url": "https://www.yoctoproject.org/docs/current/ref-manual/ref-manual.html", "title": "Yocto Project Reference Manual", "time_on_page": 95},
    ]
    
    # VS Code events
    vscode_events = [
        {"event": "file_open", "file": f"{workspace}/poky/build/conf/local.conf", "language": "conf", "workspace": workspace},
        {"event": "file_save", "file": f"{workspace}/poky/build/conf/local.conf", "language": "conf", "workspace": workspace, "time_spent": 45},
        {"event": "file_open", "file": f"{workspace}/poky/build/conf/bblayers.conf", "language": "conf", "workspace": workspace},
        {"event": "file_save", "file": f"{workspace}/poky/build/conf/bblayers.conf", "language": "conf", "workspace": workspace, "time_spent": 30},
        {"event": "file_open", "file": f"{workspace}/poky/meta-custom/recipes-core/images/my-image.bb", "language": "bitbake", "workspace": workspace},
        {"event": "file_save", "file": f"{workspace}/poky/meta-custom/recipes-core/images/my-image.bb", "language": "bitbake", "workspace": workspace, "time_spent": 120},
    ]
    
    # Combine and interleave events with timestamps
    time_offset = 0
    
    # Add terminal events
    for i, event in enumerate(terminal_events):
        events.append({
            "timestamp": (base_time + timedelta(minutes=time_offset)).isoformat() + "Z",
            "source": "terminal",
            "payload": {
                "command": event["command"],
                "exit_code": event["exit_code"],
                "duration_sec": event["duration"],
                "cwd": event["cwd"],
                **({"error": event["error"]} if "error" in event else {})
            }
        })
        time_offset += max(1, int(event["duration"] / 60)) + 1
    
    # Add browser events (interleaved)
    browser_time_offset = 5
    for event in browser_events:
        events.append({
            "timestamp": (base_time + timedelta(minutes=browser_time_offset)).isoformat() + "Z",
            "source": "browser",
            "payload": {
                "url": event["url"],
                "title": event["title"],
                "time_on_page_sec": event["time_on_page"],
                "user_agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36"
            }
        })
        browser_time_offset += max(2, event["time_on_page"] // 60) + 3
    
    # Add VS Code events (interleaved)
    vscode_time_offset = 10
    for event in vscode_events:
        events.append({
            "timestamp": (base_time + timedelta(minutes=vscode_time_offset)).isoformat() + "Z",
            "source": "vscode",
            "payload": {
                "event": event["event"],
                "file": event["file"],
                "language": event["language"],
                "workspace": event["workspace"],
                **({"time_spent_sec": event["time_spent"]} if "time_spent" in event else {})
            }
        })
        vscode_time_offset += 5
    
    # Sort by timestamp and limit to 20 events
    events.sort(key=lambda x: x["timestamp"])
    return events[:20]

def send_test_events():
    """Send test events simulating Yocto Linux build workflow."""
    print_step("Generating Yocto Linux build workflow events...")
    
    events = generate_yocto_workflow()
    print_success(f"Generated {len(events)} workflow events")
    
    print_step("Sending events to DevChronicle...")
    
    success_count = 0
    for i, event in enumerate(events, 1):
        source = event["source"]
        endpoint = f"/ingest/{source}"
        
        # Remove timestamp from payload (server will use current time if not provided)
        payload = {
            "source": source,
            "payload": event["payload"]
        }
        
        if send_event(endpoint, payload):
            success_count += 1
            print(f"  [{i}/{len(events)}] {Colors.GREEN}✓{Colors.RESET} {source}: {event['payload'].get('command', event['payload'].get('url', event['payload'].get('file', 'event')))[:50]}")
        else:
            print(f"  [{i}/{len(events)}] {Colors.RED}✗{Colors.RESET} Failed to send {source} event")
        
        # Small delay between events
        time.sleep(0.3)
    
    print()
    if success_count == len(events):
        print_success(f"All {success_count} events sent successfully!")
    else:
        print_warning(f"Sent {success_count}/{len(events)} events")
    
    print_info("Check the DevChronicle dashboard to see the AI-generated summary")
    print_info("The AI should summarize this as: 'Building Yocto Linux image, encountered disk space issue, resolved it, and successfully completed the build'")

def main():
    """Main execution flow."""
    print(f"{Colors.BOLD}{Colors.CYAN}")
    print("=" * 60)
    print("  DevChronicle Quick Setup and Test Script")
    print("=" * 60)
    print(f"{Colors.RESET}\n")
    
    # Step 1: Check dependencies
    if not check_dependencies():
        print_error("Please install missing dependencies and try again")
        sys.exit(1)
    
    print()
    
    # Step 2: Setup extensions
    print_step("Setting up extensions...")
    setup_terminal_extension()
    setup_vscode_extension()
    check_chrome_extension()
    
    print()
    
    # Step 3: Check and handle port conflicts
    vite_running = check_vite_running()
    if vite_running:
        print_success("Vite dev server is already running")
    else:
        # Check if port is in use (but Vite not responding)
        print_info("Checking if port 5173 is available...")
        port_in_use = False
        try:
            result = subprocess.run(
                ["lsof", "-ti", ":5173"],
                capture_output=True,
                timeout=2
            )
            if result.returncode == 0 and result.stdout.strip():
                port_in_use = True
                print_warning("Port 5173 is in use but Vite not responding properly")
                print_info("Attempting to free the port...")
                if kill_process_on_port(5173):
                    print_success("Port 5173 freed")
                    time.sleep(2)  # Wait for port to be fully released
                else:
                    print_error("Could not free port 5173 automatically")
                    print_info("Please manually stop the process:")
                    print_info("  lsof -ti :5173 | xargs kill -9")
                    print_info("Or restart your system")
                    sys.exit(1)
        except FileNotFoundError:
            # lsof not available, try fuser
            try:
                result = subprocess.run(
                    ["fuser", f"5173/tcp"],
                    capture_output=True,
                    timeout=2
                )
                if result.returncode == 0:
                    port_in_use = True
                    print_warning("Port 5173 is in use")
                    if kill_process_on_port(5173):
                        print_success("Port 5173 freed")
                        time.sleep(2)
            except FileNotFoundError:
                # Neither lsof nor fuser available
                print_warning("Cannot check port status (lsof/fuser not available)")
                print_info("If you get a port conflict error, manually kill the process")
        except subprocess.TimeoutExpired:
            pass
        
        if not port_in_use:
            print_info("Port 5173 is available")
        
        print_info("Vite will be started automatically by Tauri")
    
    print()
    
    # Step 4: Start application (Tauri will start Vite via beforeDevCommand)
    app_process = start_app()
    
    # Wait for both Vite and backend server
    print()
    print_step("Waiting for frontend and backend to be ready...")
    print_info("This may take 30-60 seconds on first run...")
    
    vite_ready = wait_for_vite(timeout=60)
    server_ready = wait_for_server(timeout=45)
    
    print()
    if server_ready and vite_ready:
        print_success("Both frontend and backend are ready!")
    elif server_ready:
        print_warning("Backend is ready, but Vite may still be starting")
        print_info("If the window is blank, wait 10-20 more seconds")
    elif vite_ready:
        print_warning("Frontend is ready, but backend may still be starting")
    else:
        print_error("Services are still starting - this is normal on first run")
        print_info("The application window should appear shortly")
    
    if not app_process and not server_ready:
        print_warning("Backend server not responding yet")
        print_info("The application may still be starting - check the window")
        print_info("You can proceed with testing - events will be queued")
    
    print()
    
    # Step 5: Send test events
    send_test_events()
    
    print()
    print(f"{Colors.BOLD}{Colors.GREEN}Setup complete!{Colors.RESET}")
    print()
    print_info("Next steps:")
    print_info("  1. Check the DevChronicle dashboard for AI-generated summaries")
    print_info("  2. Load the Chrome extension manually (see instructions above)")
    print_info("  3. Restart your terminal or run: source ~/.bashrc (or ~/.zshrc)")
    
    if app_process:
        print()
        print_warning("DevChronicle application is running in the background")
        print_info("Press Ctrl+C to stop the application when done testing")
        print_info("Note: The application window should show the DevChronicle UI")
        print_info("If you see a blank/white window:")
        print_info("  1. Wait 10-20 seconds for Vite to fully start")
        print_info("  2. Check the terminal output for any errors")
        print_info("  3. Try refreshing the window (if possible) or restart the app")

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n\n" + Colors.YELLOW + "Interrupted by user" + Colors.RESET)
        sys.exit(0)
    except Exception as e:
        print_error(f"Unexpected error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


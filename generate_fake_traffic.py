#!/usr/bin/env python3
"""
DevChronicle Fake Traffic Generator

This script generates and sends simulated developer activity to the DevChronicle
ingestion server for testing and demo purposes.

Usage:
  python generate_fake_traffic.py                    # Send Yocto workflow
  python generate_fake_traffic.py --workflow yocto   # Explicit Yocto workflow
  python generate_fake_traffic.py --count 50         # Send 50 events
  python generate_fake_traffic.py --interval 0.5     # 0.5s between events
  python generate_fake_traffic.py --url http://localhost:3030  # Custom server
"""

import argparse
import json
import random
import sys
import time
from datetime import datetime, timedelta
from typing import Dict, Any, List
import requests


class Colors:
    """ANSI color codes"""
    RESET = "\033[0m"
    BOLD = "\033[1m"
    RED = "\033[31m"
    GREEN = "\033[32m"
    YELLOW = "\033[33m"
    CYAN = "\033[36m"


def print_step(msg: str):
    """Print a step message."""
    print(f"{Colors.BOLD}{Colors.CYAN}▶ {msg}{Colors.RESET}")


def print_success(msg: str):
    """Print a success message."""
    print(f"{Colors.GREEN}✓ {msg}{Colors.RESET}")


def print_warning(msg: str):
    """Print a warning message."""
    print(f"{Colors.YELLOW}⚠ {msg}{Colors.RESET}")


def print_error(msg: str):
    """Print an error message."""
    print(f"{Colors.RED}✗ {msg}{Colors.RESET}")


def print_info(msg: str):
    """Print an info message."""
    print(f"ℹ {msg}")


def send_event(server_url: str, endpoint: str, payload: Dict[str, Any]) -> bool:
    """Send an event to the ingestion server."""
    try:
        url = f"{server_url.rstrip('/')}{endpoint}"
        response = requests.post(
            url,
            json=payload,
            headers={"Content-Type": "application/json"},
            timeout=5
        )
        return response.status_code in (200, 201)
    except Exception as e:
        print_error(f"Failed to send event: {e}")
        return False


def generate_yocto_workflow() -> List[Dict[str, Any]]:
    """Generate a simulated Yocto Linux build workflow."""
    now = datetime.now()
    events = []

    # Terminal: Start build
    events.append({
        "source": "terminal",
        "timestamp": now.isoformat(),
        "payload": {
            "command": "bitbake core-image-minimal",
            "exit_code": 0,
            "duration_sec": 15.5,
            "cwd": "/yocto/build"
        }
    })

    # Browser: Check Yocto docs
    events.append({
        "source": "browser",
        "timestamp": (now + timedelta(seconds=20)).isoformat(),
        "payload": {
            "url": "https://docs.yoctoproject.org/manual/system-requirements.html",
            "title": "Yocto Project System Requirements",
            "time_on_page_sec": 45
        }
    })

    # VSCode: Edit bitbake recipe
    events.append({
        "source": "vscode",
        "timestamp": (now + timedelta(seconds=65)).isoformat(),
        "payload": {
            "file_path": "/yocto/meta-custom/recipes-app/myapp/myapp_1.0.bb",
            "language": "bitbake"
        }
    })

    # Terminal: Build fails due to disk space
    events.append({
        "source": "terminal",
        "timestamp": (now + timedelta(seconds=120)).isoformat(),
        "payload": {
            "command": "bitbake core-image-minimal 2>&1 | grep -i 'disk\\|space'",
            "exit_code": 1,
            "duration_sec": 8.2,
            "cwd": "/yocto/build"
        }
    })

    # Browser: Check disk cleanup
    events.append({
        "source": "browser",
        "timestamp": (now + timedelta(seconds=150)).isoformat(),
        "payload": {
            "url": "https://docs.yoctoproject.org/dev-manual/disk-space.html",
            "title": "Managing Disk Space in Yocto",
            "time_on_page_sec": 60
        }
    })

    # Terminal: Clean build artifacts
    events.append({
        "source": "terminal",
        "timestamp": (now + timedelta(seconds=210)).isoformat(),
        "payload": {
            "command": "rm -rf tmp/ sstate-diff/ && df -h",
            "exit_code": 0,
            "duration_sec": 5.1,
            "cwd": "/yocto/build"
        }
    })

    # VSCode: Update config
    events.append({
        "source": "vscode",
        "timestamp": (now + timedelta(seconds=280)).isoformat(),
        "payload": {
            "file_path": "/yocto/build/conf/local.conf",
            "language": "conf"
        }
    })

    # Terminal: Retry build successfully
    events.append({
        "source": "terminal",
        "timestamp": (now + timedelta(seconds=320)).isoformat(),
        "payload": {
            "command": "bitbake core-image-minimal",
            "exit_code": 0,
            "duration_sec": 120.5,
            "cwd": "/yocto/build"
        }
    })

    # Browser: Check build result docs
    events.append({
        "source": "browser",
        "timestamp": (now + timedelta(seconds=450)).isoformat(),
        "payload": {
            "url": "https://docs.yoctoproject.org/ref-manual/images.html",
            "title": "Yocto Project Images",
            "time_on_page_sec": 30
        }
    })

    return events


def generate_generic_workflow(count: int = 10) -> List[Dict[str, Any]]:
    """Generate generic developer activity."""
    events = []
    now = datetime.now()

    urls = [
        "https://github.com",
        "https://stackoverflow.com",
        "https://docs.python.org",
        "https://developer.mozilla.org",
        "https://www.npmjs.com"
    ]

    commands = [
        "git status",
        "npm run build",
        "npm test",
        "python -m pytest",
        "cargo build",
        "make clean",
        "docker ps -a"
    ]

    files = [
        "src/main.ts",
        "src/utils/helpers.py",
        "tests/unit_test.rs",
        "config.json",
        "Dockerfile",
        "package.json"
    ]

    for i in range(count):
        timestamp = now + timedelta(seconds=i * 10)
        event_type = random.choice(["terminal", "browser", "vscode"])

        if event_type == "terminal":
            events.append({
                "source": "terminal",
                "timestamp": timestamp.isoformat(),
                "payload": {
                    "command": random.choice(commands),
                    "exit_code": random.choice([0, 0, 0, 1]),  # Mostly success
                    "duration_sec": random.uniform(0.5, 30),
                    "cwd": "/home/dev/project"
                }
            })
        elif event_type == "browser":
            events.append({
                "source": "browser",
                "timestamp": timestamp.isoformat(),
                "payload": {
                    "url": random.choice(urls),
                    "title": "Developer Resource",
                    "time_on_page_sec": random.randint(10, 180)
                }
            })
        else:  # vscode
            events.append({
                "source": "vscode",
                "timestamp": timestamp.isoformat(),
                "payload": {
                    "file_path": f"/home/dev/project/{random.choice(files)}",
                    "language": random.choice(["typescript", "python", "rust", "json"])
                }
            })

    return events


def main():
    """Main execution."""
    parser = argparse.ArgumentParser(
        description="Generate and send fake traffic to DevChronicle server",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python generate_fake_traffic.py                    # Send Yocto workflow
  python generate_fake_traffic.py --workflow yocto   # Explicit Yocto workflow
  python generate_fake_traffic.py --workflow generic --count 50
  python generate_fake_traffic.py --interval 0.2
        """
    )

    parser.add_argument(
        "--url",
        default="http://localhost:3030",
        help="Server URL (default: http://localhost:3030)"
    )
    parser.add_argument(
        "--workflow",
        choices=["yocto", "generic"],
        default="yocto",
        help="Workflow type to simulate (default: yocto)"
    )
    parser.add_argument(
        "--count",
        type=int,
        default=None,
        help="Number of events (for generic workflow)"
    )
    parser.add_argument(
        "--interval",
        type=float,
        default=0.3,
        help="Delay between events in seconds (default: 0.3)"
    )

    args = parser.parse_args()

    print(f"\n{Colors.BOLD}{Colors.CYAN}")
    print("=" * 60)
    print("  DevChronicle Fake Traffic Generator")
    print("=" * 60)
    print(f"{Colors.RESET}\n")

    # Verify server is reachable
    print_step(f"Verifying server at {args.url}...")
    try:
        response = requests.get(f"{args.url}/health", timeout=5)
        if response.status_code == 200:
            print_success("Server is reachable")
        else:
            print_warning(f"Server returned status {response.status_code}")
    except Exception as e:
        print_error(f"Cannot reach server: {e}")
        print_info("Make sure DevChronicle is running: npm run tauri:dev")
        sys.exit(1)

    # Generate events
    print()
    if args.workflow == "yocto":
        print_step("Generating Yocto Linux build workflow events...")
        events = generate_yocto_workflow()
    else:
        count = args.count or 10
        print_step(f"Generating {count} generic developer activity events...")
        events = generate_generic_workflow(count)

    print_success(f"Generated {len(events)} events")

    # Send events
    print_step(f"Sending events to {args.url} (interval: {args.interval}s)...")
    print()

    success_count = 0
    for i, event in enumerate(events, 1):
        source = event["source"]
        endpoint = f"/ingest/{source}"

        payload = {
            "source": source,
            "payload": event["payload"]
        }

        if send_event(args.url, endpoint, payload):
            success_count += 1
            detail = event["payload"].get(
                "command",
                event["payload"].get(
                    "url",
                    event["payload"].get("file_path", "event")
                )
            )[:50]
            print(f"  [{i:2d}/{len(events)}] {Colors.GREEN}✓{Colors.RESET} {source:10s} {detail}")
        else:
            print(f"  [{i:2d}/{len(events)}] {Colors.RED}✗{Colors.RESET} Failed to send {source} event")

        if i < len(events):
            time.sleep(args.interval)

    print()
    if success_count == len(events):
        print_success(f"All {success_count} events sent successfully!")
    else:
        print_warning(f"Sent {success_count}/{len(events)} events")

    print()
    print_info("Check the DevChronicle dashboard to see the AI-generated summary")
    print()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print(f"\n\n{Colors.YELLOW}Interrupted by user{Colors.RESET}")
        sys.exit(0)
    except Exception as e:
        print_error(f"Unexpected error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)

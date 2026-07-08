#!/usr/bin/env python3
"""
Deployment script for SDK-IA project
"""

import subprocess
import sys
from pathlib import Path


def start_services():
    """Start all services"""
    print("Starting SDK-IA services...")
    
    rust_proc = subprocess.Popen(
        ["cargo", "run", "--release"],
        cwd=Path(__file__).parent.parent,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    
    python_proc = subprocess.Popen(
        [sys.executable, "-m", "secretario.main"],
        cwd=Path(__file__).parent.parent / "python",
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    
    ui_proc = subprocess.Popen(
        ["npm", "run", "dev"],
        cwd=Path(__file__).parent.parent / "ui",
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    
    print("All services started. Press Ctrl+C to stop.")
    
    try:
        rust_proc.wait()
        python_proc.wait()
        ui_proc.wait()
    except KeyboardInterrupt:
        print("Stopping services...")
        rust_proc.terminate()
        python_proc.terminate()
        ui_proc.terminate()
        return True
    
    return True


def main():
    """Main deploy function"""
    if not start_services():
        sys.exit(1)


if __name__ == "__main__":
    main()

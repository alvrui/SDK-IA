#!/usr/bin/env python3
"""
Build script for SDK-IA project
"""

import subprocess
import sys
from pathlib import Path


def build_rust():
    """Build Rust backend"""
    print("Building Rust backend...")
    try:
        result = subprocess.run(
            ["cargo", "build", "--release"],
            cwd=Path(__file__).parent.parent,
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            print("Rust build failed:", result.stderr)
            return False
        print("Rust backend built successfully")
        return True
    except Exception as e:
        print("Error building Rust:", e)
        return False


def build_python():
    """Install Python dependencies"""
    print("Installing Python dependencies...")
    try:
        result = subprocess.run(
            [sys.executable, "-m", "pip", "install", "-e", "."],
            cwd=Path(__file__).parent.parent,
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            print("Python install failed:", result.stderr)
            return False
        print("Python dependencies installed successfully")
        return True
    except Exception as e:
        print("Error installing Python dependencies:", e)
        return False


def build_ui():
    """Build UI frontend"""
    print("Building UI frontend...")
    try:
        result = subprocess.run(
            ["npm", "run", "build"],
            cwd=Path(__file__).parent.parent / "ui",
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            print("UI build failed:", result.stderr)
            return False
        print("UI frontend built successfully")
        return True
    except Exception as e:
        print("Error building UI:", e)
        return False


def main():
    """Main build function"""
    print("Starting SDK-IA build process...")
    
    success = True
    success = build_rust() and success
    success = build_python() and success
    success = build_ui() and success
    
    if success:
        print("Build completed successfully!")
        sys.exit(0)
    else:
        print("Build failed!")
        sys.exit(1)


if __name__ == "__main__":
    main()

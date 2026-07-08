#!/usr/bin/env python3
"""
Setup script for SDK-IA development environment on Debian
Configures Python 3.13.5, Rust 1.96, and project dependencies
"""

import subprocess
import sys
import os
from pathlib import Path


def check_command_exists(command: str) -> bool:
    """Check if a command exists in the system"""
    try:
        result = subprocess.run(
            ["which", command],
            capture_output=True,
            text=True,
        )
        return result.returncode == 0
    except Exception:
        return False


def run_command(command: str, description: str) -> bool:
    """Run a command and report results"""
    print(f"  {description}...")
    try:
        result = subprocess.run(
            command,
            shell=True,
            capture_output=True,
            text=True,
        )
        if result.returncode == 0:
            print(f"    Success")
            return True
        else:
            print(f"    Failed: {result.stderr}")
            return False
    except Exception as e:
        print(f"    Error: {e}")
        return False


def check_python() -> bool:
    """Check Python 3.13.5 installation"""
    print("Checking Python 3.13.5...")
    try:
        result = subprocess.run(
            ["python3", "--version"],
            capture_output=True,
            text=True,
        )
        version = result.stdout.strip()
        print(f"  Found: {version}")
        
        if "3.13.5" in version:
            print("  Python 3.13.5 is installed")
            return True
        else:
            print("  Python 3.13.5 not found, attempting to install...")
            return install_python()
    except Exception:
        return install_python()


def install_python() -> bool:
    """Install Python 3.13.5 on Debian"""
    print("Installing Python 3.13.5...")
    
    # Update package lists
    if not run_command("sudo apt-get update", "Updating package lists"):
        return False
    
    # Install build dependencies
    if not run_command(
        "sudo apt-get install -y build-essential zlib1g-dev libncurses5-dev libgdbm-dev libnss3-dev libssl-dev libreadline-dev libffi-dev libsqlite3-dev wget libbz2-dev",
        "Installing build dependencies"
    ):
        return False
    
    # Download and install Python 3.13.5
    print("  Downloading Python 3.13.5 source...")
    if not run_command(
        "wget https://www.python.org/ftp/python/3.13.5/Python-3.13.5.tar.xz",
        "Downloading Python source"
    ):
        return False
    
    print("  Extracting source...")
    if not run_command("tar -xf Python-3.13.5.tar.xz", "Extracting source"):
        return False
    
    print("  Configuring...")
    if not run_command(
        "cd Python-3.13.5 && ./configure --enable-optimizations",
        "Configuring Python"
    ):
        return False
    
    print("  Building (this may take a while)...")
    if not run_command("cd Python-3.13.5 && make -j $(nproc)", "Building Python"):
        return False
    
    print("  Installing...")
    if not run_command("cd Python-3.13.5 && sudo make altinstall", "Installing Python"):
        return False
    
    print("  Cleaning up...")
    run_command("rm -rf Python-3.13.5 Python-3.13.5.tar.xz", "Cleaning up")
    
    # Verify installation
    return check_python()


def check_rust() -> bool:
    """Check Rust 1.96 installation"""
    print("Checking Rust 1.96...")
    try:
        result = subprocess.run(
            ["rustc", "--version"],
            capture_output=True,
            text=True,
        )
        version = result.stdout.strip()
        print(f"  Found: {version}")
        
        if "1.96" in version:
            print("  Rust 1.96 is installed")
            return True
        else:
            print("  Rust 1.96 not found, attempting to install...")
            return install_rust()
    except Exception:
        return install_rust()


def install_rust() -> bool:
    """Install Rust 1.96 using rustup"""
    print("Installing Rust 1.96...")
    
    # Check if rustup is installed
    if not check_command_exists("rustup"):
        print("  Installing rustup...")
        if not run_command(
            "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
            "Installing rustup"
        ):
            return False
    
    # Install specific version
    print("  Installing Rust 1.96...")
    if not run_command(
        "rustup install 1.96",
        "Installing Rust 1.96"
    ):
        return False
    
    # Set default
    if not run_command("rustup default 1.96", "Setting Rust 1.96 as default"):
        return False
    
    # Verify
    return check_rust()


def check_cargo() -> bool:
    """Check Cargo 1.96 installation"""
    print("Checking Cargo 1.96...")
    try:
        result = subprocess.run(
            ["cargo", "--version"],
            capture_output=True,
            text=True,
        )
        version = result.stdout.strip()
        print(f"  Found: {version}")
        
        if "1.96" in version:
            print("  Cargo 1.96 is installed")
            return True
        else:
            print("  Cargo 1.96 will be installed with Rust")
            return True
    except Exception:
        return True


def check_nodejs() -> bool:
    """Check Node.js installation for UI development"""
    print("Checking Node.js...")
    try:
        result = subprocess.run(
            ["node", "--version"],
            capture_output=True,
            text=True,
        )
        version = result.stdout.strip()
        print(f"  Found: {version}")
        
        # Check if version is 18 or later
        major_version = int(version.replace("v", "").split(".")[0])
        if major_version >= 18:
            print("  Node.js 18+ is installed")
            return True
        else:
            print("  Node.js 18+ not found, attempting to install...")
            return install_nodejs()
    except Exception:
        return install_nodejs()


def install_nodejs() -> bool:
    """Install Node.js 18+ on Debian"""
    print("Installing Node.js 18+...")
    
    # Use NodeSource repository
    if not run_command(
        "curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -",
        "Adding NodeSource repository"
    ):
        return False
    
    if not run_command(
        "sudo apt-get install -y nodejs",
        "Installing Node.js"
    ):
        return False
    
    # Verify
    return check_nodejs()


def check_python_dependencies() -> bool:
    """Check Python dependencies"""
    print("Checking Python dependencies...")
    
    # Check if virtual environment exists
    venv_path = Path("python/venv")
    if not venv_path.exists():
        print("  Creating Python virtual environment...")
        if not run_command(
            "python3.13 -m venv python/venv",
            "Creating virtual environment"
        ):
            return False
    
    # Install dependencies
    print("  Installing Python dependencies...")
    if not run_command(
        "python/venv/bin/pip install -r python/requirements.txt",
        "Installing Python dependencies"
    ):
        return False
    
    print("  Python dependencies installed")
    return True


def check_rust_dependencies() -> bool:
    """Check Rust dependencies"""
    print("Checking Rust dependencies...")
    
    # Just try to build to check dependencies
    print("  Checking Rust project...")
    if not run_command(
        "cd src && cargo check",
        "Checking Rust project"
    ):
        return False
    
    print("  Rust dependencies are ready")
    return True


def check_ui_dependencies() -> bool:
    """Check UI dependencies"""
    print("Checking UI dependencies...")
    
    if not run_command(
        "cd ui && npm install",
        "Installing UI dependencies"
    ):
        return False
    
    print("  UI dependencies installed")
    return True


def create_directories() -> bool:
    """Create necessary directories"""
    print("Creating necessary directories...")
    
    directories = [
        "data",
        "data/projects",
        "data/logs",
        "python/venv",
    ]
    
    for directory in directories:
        path = Path(directory)
        if not path.exists():
            print(f"  Creating {directory}...")
            path.mkdir(parents=True, exist_ok=True)
    
    return True


def main():
    """Main setup function"""
    print("=" * 60)
    print("SDK-IA Development Environment Setup")
    print("=" * 60)
    print()
    
    # Create directories first
    if not create_directories():
        print("Error creating directories")
        sys.exit(1)
    
    # Check Python
    if not check_python():
        print("Error setting up Python")
        sys.exit(1)
    print()
    
    # Check Rust
    if not check_rust():
        print("Error setting up Rust")
        sys.exit(1)
    print()
    
    # Check Cargo
    if not check_cargo():
        print("Error setting up Cargo")
        sys.exit(1)
    print()
    
    # Check Node.js
    if not check_nodejs():
        print("Error setting up Node.js")
        sys.exit(1)
    print()
    
    # Check Python dependencies
    if not check_python_dependencies():
        print("Error setting up Python dependencies")
        sys.exit(1)
    print()
    
    # Check Rust dependencies
    if not check_rust_dependencies():
        print("Error setting up Rust dependencies")
        sys.exit(1)
    print()
    
    # Check UI dependencies
    if not check_ui_dependencies():
        print("Error setting up UI dependencies")
        sys.exit(1)
    print()
    
    print("=" * 60)
    print("Environment setup completed successfully!")
    print("=" * 60)
    print()
    print("You can now run:")
    print("  make build    - Build all components")
    print("  make run      - Start all services")
    print("  make test     - Run all tests")
    print()


if __name__ == "__main__":
    main()

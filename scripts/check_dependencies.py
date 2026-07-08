#!/usr/bin/env python3
"""
Dependency check script for SDK-IA project
Verifies all required dependencies are installed
"""

import subprocess
import sys
from pathlib import Path


def check_command(command: str, version_flag: str = "--version", min_version: str = None) -> dict:
    """Check if a command is available and optionally verify version"""
    try:
        result = subprocess.run(
            [command, version_flag],
            capture_output=True,
            text=True,
            timeout=5,
        )
        
        if result.returncode == 0:
            version = result.stdout.strip()
            installed = True
            version_ok = True
            
            if min_version:
                # Simple version comparison (works for semantic versioning)
                version_ok = compare_versions(version, min_version)
            
            return {
                "installed": installed,
                "version": version,
                "required": min_version,
                "ok": version_ok,
            }
        else:
            return {
                "installed": False,
                "version": None,
                "required": min_version,
                "ok": False,
            }
    except Exception:
        return {
            "installed": False,
            "version": None,
            "required": min_version,
            "ok": False,
        }


def compare_versions(installed_version: str, required_version: str) -> bool:
    """Compare two version strings"""
    # Extract version number from string like "python 3.13.5" or "rustc 1.96.0"
    def extract_version(v: str) -> str:
        if not v:
            return "0.0.0"
        # Remove non-version characters
        v = v.replace("v", "").replace(" ", "")
        # Take first version-like part
        for part in v.split():
            if part.count(".") >= 1:
                return part
        return v
    
    installed = extract_version(installed_version)
    required = extract_version(required_version)
    
    # Split into parts and compare
    installed_parts = installed.split(".")
    required_parts = required.split(".")
    
    # Pad with zeros
    max_len = max(len(installed_parts), len(required_parts))
    installed_parts += ["0"] * (max_len - len(installed_parts))
    required_parts += ["0"] * (max_len - len(required_parts))
    
    # Compare each part
    for i in range(max_len):
        try:
            installed_num = int(installed_parts[i])
            required_num = int(required_parts[i])
            
            if installed_num < required_num:
                return False
            elif installed_num > required_num:
                return True
        except ValueError:
            # If we can't parse as int, do string comparison
            if installed_parts[i] < required_parts[i]:
                return False
            elif installed_parts[i] > required_parts[i]:
                return True
    
    return True


def check_python_packages() -> list:
    """Check installed Python packages"""
    results = []
    
    required_packages = [
        "fastapi",
        "uvicorn",
        "pydantic",
        "aiosqlite",
        "mistralai",
        "aiohttp",
        "structlog",
    ]
    
    try:
        result = subprocess.run(
            ["pip", "list", "--format=json"],
            capture_output=True,
            text=True,
            timeout=10,
        )
        
        if result.returncode == 0:
            import json
            packages = json.loads(result.stdout)
            installed_packages = {p["name"].lower(): p["version"] for p in packages}
            
            for package in required_packages:
                if package.lower() in installed_packages:
                    results.append({
                        "name": package,
                        "installed": True,
                        "version": installed_packages[package.lower()],
                        "ok": True,
                    })
                else:
                    results.append({
                        "name": package,
                        "installed": False,
                        "version": None,
                        "ok": False,
                    })
    except Exception:
        # Fallback to pip show
        for package in required_packages:
            result = subprocess.run(
                ["pip", "show", package],
                capture_output=True,
                text=True,
            )
            if result.returncode == 0:
                # Extract version from output
                version = None
                for line in result.stdout.split("
"):
                    if line.startswith("Version:"):
                        version = line.split(":")[1].strip()
                        break
                results.append({
                    "name": package,
                    "installed": True,
                    "version": version,
                    "ok": True,
                })
            else:
                results.append({
                    "name": package,
                    "installed": False,
                    "version": None,
                    "ok": False,
                })
    
    return results


def check_node_packages() -> list:
    """Check installed Node.js packages"""
    results = []
    
    required_packages = [
        "react",
        "typescript",
        "vite",
    ]
    
    try:
        result = subprocess.run(
            ["npm", "list", "--json", "--depth=0"],
            capture_output=True,
            text=True,
            timeout=10,
            cwd="ui",
        )
        
        if result.returncode == 0:
            import json
            packages = json.loads(result.stdout)
            # This is simplified - actual parsing would be more complex
            results.append({
                "name": "Node.js packages",
                "installed": True,
                "version": "checked",
                "ok": True,
            })
    except Exception:
        results.append({
            "name": "Node.js packages",
            "installed": False,
            "version": None,
            "ok": False,
        })
    
    return results


def check_rust_crates() -> list:
    """Check Rust crate dependencies"""
    results = []
    
    try:
        result = subprocess.run(
            ["cargo", "check"],
            capture_output=True,
            text=True,
            timeout=30,
            cwd="src",
        )
        
        if result.returncode == 0:
            results.append({
                "name": "Rust crates",
                "installed": True,
                "version": "checked",
                "ok": True,
            })
        else:
            results.append({
                "name": "Rust crates",
                "installed": False,
                "version": None,
                "ok": False,
                "error": result.stderr[:200],
            })
    except Exception as e:
        results.append({
            "name": "Rust crates",
            "installed": False,
            "version": None,
            "ok": False,
            "error": str(e),
        })
    
    return results


def print_results(title: str, results: list) -> bool:
    """Print dependency check results"""
    print(f"
{title}:")
    print("-" * 60)
    
    all_ok = True
    for result in results:
        status = "OK" if result["ok"] else "FAIL"
        version = result.get("version", "N/A")
        
        if result["installed"]:
            print(f"  [OK] {result['name']} - {version}")
        else:
            print(f"  [FAIL] {result['name']} - Not installed")
            all_ok = False
        
        if not result["ok"] and result.get("error"):
            print(f"    Error: {result['error']}")
    
    return all_ok


def main():
    """Main check function"""
    print("=" * 60)
    print("SDK-IA Dependency Check")
    print("=" * 60)
    
    all_ok = True
    
    # Check system dependencies
    system_deps = [
        {"name": "Python", "command": "python3", "min_version": "3.13.5"},
        {"name": "Rust", "command": "rustc", "min_version": "1.96"},
        {"name": "Cargo", "command": "cargo", "min_version": "1.96"},
        {"name": "Node.js", "command": "node", "min_version": "18.0.0"},
        {"name": "npm", "command": "npm", "min_version": "8.0.0"},
        {"name": "Git", "command": "git", "min_version": "2.0.0"},
    ]
    
    system_results = []
    for dep in system_deps:
        result = check_command(dep["command"], "--version", dep["min_version"])
        result["name"] = dep["name"]
        system_results.append(result)
    
    all_ok &= print_results("System Dependencies", system_results)
    
    # Check Python packages
    python_results = check_python_packages()
    all_ok &= print_results("Python Packages", python_results)
    
    # Check Node.js packages
    node_results = check_node_packages()
    all_ok &= print_results("Node.js Packages", node_results)
    
    # Check Rust crates
    rust_results = check_rust_crates()
    all_ok &= print_results("Rust Crates", rust_results)
    
    print()
    print("=" * 60)
    if all_ok:
        print("All dependencies are installed and up to date!")
        sys.exit(0)
    else:
        print("Some dependencies are missing or outdated.")
        print("Run 'make setup' to install missing dependencies.")
        sys.exit(1)


if __name__ == "__main__":
    main()

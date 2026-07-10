# SDK-IA Management Script

## Usage

The `sdkia.sh` script allows you to manage all SDK-IA services from a single command.

### Basic Commands

```bash
# Start all services (Rust backend, Python Secretario, UI Frontend)
./sdkia.sh start

# Stop all services
./sdkia.sh stop

# Show status of all services
./sdkia.sh status

# Restart all services
./sdkia.sh restart

# Show help
./sdkia.sh
```

### Services Overview

| Service | Port | Description | Start Command |
|---------|------|-------------|---------------|
| Rust Backend | 9090 | SDK-IA Rust backend API | `./target/debug/SDK-IA` |
| Python Secretario | 9000 | Mistral agents management service | `python -m secreto.main` |
| UI Frontend | 3000 | React/Vite web interface | `npm run dev` |

### Access URLs

Once all services are running:
- **Web Interface**: http://localhost:3000/
- **Rust API**: http://localhost:9090/api/v1/internal/health
- **Python API**: http://localhost:9000/api/v1/internal/health

### Log Files

All service logs are stored in:
- `/tmp/sdkia-logs/rust.log` - Rust backend logs
- `/tmp/sdkia-logs/python.log` - Python Secretario logs
- `/tmp/sdkia-logs/ui.log` - UI Frontend logs

### PID Files

Process IDs are tracked in:
- `/tmp/sdkia-pids/rust.pid`
- `/tmp/sdkia-pids/python.pid`
- `/tmp/sdkia-pids/ui.pid`

### Requirements

- Rust backend must be compiled (binary at `target/debug/SDK-IA`)
- Python virtual environment must be set up (in `python/venv/`)
- Node.js and npm must be installed for the UI
- The `index.html` file must be present in the `ui/` directory (not just in `ui/public/`)

### Auto-Fix Features

The script includes automatic fixes for common issues:
- **Missing 'ms' module**: Automatically installs the Node.js 'ms' module if not found
- **Port conflicts**: Detects and warns about occupied ports before starting
- **PID tracking**: Manages process IDs to ensure clean startup/shutdown

### Notes

- The script uses `nohup` to run services in the background
- All services are started from their respective directories
- Port conflicts are detected before starting services
- Health checks verify that services are responding correctly
- The `stop` command kills processes and removes PID files

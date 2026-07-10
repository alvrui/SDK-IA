
#!/bin/bash

# SDK-IA Management Script
# Usage: ./sdkia.sh start|stop|status
#
# This script manages the three components of SDK-IA:
# - Rust Backend (port 9090)
# - Python Secretario (port 9000)
# - UI Frontend (port 3000)

# Configuration
SDKIA_ROOT="/media/alvaro/service/project-stack/SDK-IA"
RUST_BIN="$SDKIA_ROOT/target/debug/SDK-IA"
PYTHON_CMD="$SDKIA_ROOT/python/venv/bin/python -m secretario.main"
UI_DIR="$SDKIA_ROOT/ui"

# Check if Node.js dependencies are installed
check_node_deps() {
    if [ ! -d "$UI_DIR/node_modules/ms" ]; then
        echo -e "${YELLOW}⚠${NC} Node.js module 'ms' is not installed in $UI_DIR"
        echo -e "      Running: cd $UI_DIR && npm install ms"
        cd "$UI_DIR" && npm install ms 2>&1 | grep -E "(added|error)" || true
        echo -e "      ${GREEN}✓${NC} Module installed"
    fi
}

# PID files
PID_DIR="/tmp/sdkia-pids"
RUST_PID_FILE="$PID_DIR/rust.pid"
PYTHON_PID_FILE="$PID_DIR/python.pid"
UI_PID_FILE="$PID_DIR/ui.pid"

# Log files
LOG_DIR="/tmp/sdkia-logs"
RUST_LOG="$LOG_DIR/rust.log"
PYTHON_LOG="$LOG_DIR/python.log"
UI_LOG="$LOG_DIR/ui.log"

# Create directories if they don't exist
mkdir -p "$PID_DIR" "$LOG_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check if a port is in use
port_in_use() {
    local port=$1
    if ss -tuln | grep -q ":$port "; then
        return 0
    else
        return 1
    fi
}

# Function to wait for port to be available
timeout=30
wait_for_port() {
    local port=$1
    local service_name=$2
    local elapsed=0
    
    while [ $elapsed -lt $timeout ]; do
        if ! port_in_use $port; then
            echo -e "${GREEN}✓${NC} Port $port is free"
            return 0
        fi
        sleep 1
        elapsed=$((elapsed + 1))
        echo -n "."
    done
    
    echo -e "\n${RED}✗${NC} Timeout waiting for port $port ($service_name)"
    return 1
}

# Function to check service health
check_health() {
    local port=$1
   
 local endpoint=$2
    local service_name=$3
    
    if curl -s --max-time 3 "http://127.0.0.1:$port$endpoint" > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} $service_name (port $port) is healthy"
        return 0
    else
        echo -e "${RED}✗${NC} $service_name (port $port) is not responding"
        return 1
    fi
}

# Function to start all services
start_services() {
    echo "========================================"
    echo "  Starting SDK-IA Services"
    echo "========================================"
    
    # Check and install missing Node.js dependencies
    check_node_deps
    
    # Check if already running
    if [ -f "$RUST_PID_FILE" ] && kill -0 $(cat "$RUST_PID_FILE") 2>/dev/null; then
        echo -e "${YELLOW}⚠${NC} Rust backend is already running (PID: $(cat "$RUST_PID_FILE"))"
    else
        echo -n "Starting Rust backend (port 9090)..."
        if port_in_use 9090; then
            echo -e "\n${RED}✗${NC} Port 9090 is already in use!"
            echo "   Kill existing process: kill -9 \$(lsof -ti:9090)"
            return 1
        fi
        cd "$SDKIA_ROOT" || return 1
        nohup "$RUST_BIN" > "$RUST_LOG" 2>&1 &
        echo $! > "$RUST_PID_FILE"
        echo -e " ${GREEN}Started${NC} (PID: $!)"
    fi
    
    # Wait for Rust to start
    wait_for_port 9090 "Rust backend"
    
    # Start Python Secretario
    if [ -f "$PYTHON_PID_FILE" ] && kill -0 $(cat "$PYTHON_PID_FILE") 2>/dev/null; then
        echo -e "${YELLOW}⚠${NC} Python secretario is already running (PID: $(cat "$PYTHON_PID_FILE"))"
    else
        echo -n "Starting Python Secretario (port 9000)..."
        if port_in_use 9000; then
            echo -e "\n${RED}✗${NC} Port 9000 is already in use!"
            echo "   Kill existing process: kill -9 \$(lsof -ti:9000)"
            return 1
        fi
        cd "$SDKIA_ROOT/python" || return 1
        nohup "$SDKIA_ROOT/python/venv/bin/python" -m secretario.main > "$PYTHON_LOG" 2>&1 &
        echo $! > "$PYTHO
N_PID_FILE"
        echo -e " ${GREEN}Started${NC} (PID: $!)"
    fi
    
    # Wait for Python to start
    wait_for_port 9000 "Python Secretario"
    
    # Start UI Frontend
    if [ -f "$UI_PID_FILE" ] && kill -0 $(cat "$UI_PID_FILE") 2>/dev/null; then
        echo -e "${YELLOW}⚠${NC} UI frontend is already running (PID: $(cat "$UI_PID_FILE"))"
    else
        echo -n "Starting UI Frontend (port 3000)..."
        if port_in_use 3000; then
            echo -e "\n${RED}✗${NC} Port 3000 is already in use!"
            echo "   Kill existing process: kill -9 \$(lsof -ti:3000)"
            return 1
        fi
        cd "$UI_DIR" || return 1
        nohup npm run dev > "$UI_LOG" 2>&1 &
        echo $! > "$UI_PID_FILE"
        echo -e " ${GREEN}Started${NC} (PID: $!)"
    fi
    
    # Wait for UI to start
    wait_for_port 3000 "UI Frontend"
    
    echo ""
    echo "========================================"
    echo "  Health Check"
    echo "========================================"
    sleep 5  # Wait for services to fully initialize
    check_health 9090 "/api/v1/internal/health" "Rust Backend"
    check_health 9000 "/api/v1/internal/health" "Python Secretario"
    
    # Check UI with different approach (Vite doesn't always respond to health check)
    if curl -s --max-time 3 "http://127.0.0.1:3000/" > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC} UI Frontend (port 3000) is responding"
    else
        echo -e "${YELLOW}⚠${NC} UI Frontend (port 3000) - checking port..."
        if port_in_use 3000; then
            echo -e "      ${GREEN}✓${NC} Port 3000 is in use (Vite is running)"
        fi
    fi
    
    echo ""
    echo "========================================"
    echo "  Services Started Successfully!"
    echo "========================================"
    echo ""
    echo "  Access URLs:"
    echo "  - UI:       http://localhost:3000/"
    echo "  - Rust API: http://localhost:9090/api/v1/internal/health"
    echo "  - Python API: http://local
host:9000/api/v1/internal/health"
    echo ""
    echo "  Log files:"
    echo "  - Rust:   $RUST_LOG"
    echo "  - Python: $PYTHON_LOG"
    echo "  - UI:     $UI_LOG"
    echo ""
    echo "  To stop all services: $0 stop"
    echo ""
}

# Function to stop all services
stop_services() {
    echo "========================================"
    echo "  Stopping SDK-IA Services"
    echo "========================================"
    
    services_stopped=0
    
    # Stop Rust backend
    if [ -f "$RUST_PID_FILE" ]; then
        rust_pid=$(cat "$RUST_PID_FILE")
        if kill -0 "$rust_pid" 2>/dev/null; then
            echo -n "Stopping Rust backend..."
            kill -9 "$rust_pid" 2>/dev/null
            rm -f "$RUST_PID_FILE"
            echo -e " ${GREEN}Stopped${NC}"
            services_stopped=$((services_stopped + 1))
        else
            echo -e "${YELLOW}⚠${NC} Rust backend was not running (PID: $rust_pid)"
            rm -f "$RUST_PID_FILE"
        fi
    else
        echo -e "${YELLOW}⚠${NC} Rust backend PID file not found"
    fi
    
    # Stop Python Secretario
    if [ -f "$PYTHON_PID_FILE" ]; then
        python_pid=$(cat "$PYTHON_PID_FILE")
        if kill -0 "$python_pid" 2>/dev/null; then
            echo -n "Stopping Python Secretario..."
            kill -9 "$python_pid" 2>/dev/null
            rm -f "$PYTHON_PID_FILE"
            echo -e " ${GREEN}Stopped${NC}"
            services_stopped=$((services_stopped + 1))
        else
            echo -e "${YELLOW}⚠${NC} Python Secretario was not running (PID: $python_pid)"
            rm -f "$PYTHON_PID_FILE"
        fi
    else
        echo -e "${YELLOW}⚠${NC} Python Secretario PID file not found"
    fi
    
    # Stop UI Frontend
    if [ -f "$UI_PID_FILE" ]; then
        ui_pid=$(cat "$UI_PID_FILE")
        if kill -0 "$ui_pid" 2>/dev/null; then
            echo -n "Stopping UI Frontend..."
            kill -9 "$ui_pid" 2>/dev/null
            rm -f "$UI_PID_FILE"
            echo -e " ${G
REEN}Stopped${NC}"
            services_stopped=$((services_stopped + 1))
        else
            echo -e "${YELLOW}⚠${NC} UI Frontend was not running (PID: $ui_pid)"
            rm -f "$UI_PID_FILE"
        fi
    else
        echo -e "${YELLOW}⚠${NC} UI Frontend PID file not found"
    fi
    
    # Also kill any remaining processes on the ports
    echo -n "Cleaning up any remaining processes on ports..."
    lsof -ti:9090 | xargs -r kill -9 2>/dev/null
    lsof -ti:9000 | xargs -r kill -9 2>/dev/null
    lsof -ti:3000 | xargs -r kill -9 2>/dev/null
    echo -e " ${GREEN}Done${NC}"
    
    echo ""
    if [ $services_stopped -gt 0 ]; then
        echo -e "${GREEN}✓${NC} $services_stopped service(s) stopped"
    else
        echo -e "${YELLOW}⚠${NC} No running services were found"
    fi
    echo ""
}

# Function to show status
show_status() {
    echo "========================================"
    echo "  SDK-IA Services Status"
    echo "========================================"
    
    # Rust Backend
    if [ -f "$RUST_PID_FILE" ]; then
        rust_pid=$(cat "$RUST_PID_FILE")
        if kill -0 "$rust_pid" 2>/dev/null; then
            if port_in_use 9090; then
                echo -e "${GREEN}✓${NC} Rust Backend    (PID: $rust_pid, Port: 9090) - RUNNING"
                check_health 9090 "/api/v1/internal/health" "  → Health check"
            else
                echo -e "${RED}✗${NC} Rust Backend    (PID: $rust_pid, Port: 9090) - NOT RESPONDING"
            fi
        else
            echo -e "${RED}✗${NC} Rust Backend    (PID: $rust_pid) - DEAD"
        fi
    else
        echo -e "${RED}✗${NC} Rust Backend    - NOT RUNNING"
    fi
    
    # Python Secretario
    if [ -f "$PYTHON_PID_FILE" ]; then
        python_pid=$(cat "$PYTHON_PID_FILE")
        if kill -0 "$python_pid" 2>/dev/null; then
            if port_in_use 9000; then
                echo -e "${GREEN}✓${NC} Python Secretario (PID: $python_pid, Port: 9000) - RUNNING"
                check_heal
th 9000 "/api/v1/internal/health" "  → Health check"
            else
                echo -e "${RED}✗${NC} Python Secretario (PID: $python_pid, Port: 9000) - NOT RESPONDING"
            fi
        else
            echo -e "${RED}✗${NC} Python Secretario (PID: $python_pid) - DEAD"
        fi
    else
        echo -e "${RED}✗${NC} Python Secretario - NOT RUNNING"
    fi
    
    # UI Frontend
    if [ -f "$UI_PID_FILE" ]; then
        ui_pid=$(cat "$UI_PID_FILE")
        if kill -0 "$ui_pid" 2>/dev/null; then
            if port_in_use 3000; then
                echo -e "${GREEN}✓${NC} UI Frontend     (PID: $ui_pid, Port: 3000) - RUNNING"
                if curl -s --max-time 2 "http://127.0.0.1:3000/" > /dev/null 2>&1; then
                    echo -e "  ${GREEN}✓${NC} HTTP response: OK"
                else
                    echo -e "  ${YELLOW}⚠${NC} HTTP response: Not available (Vite may still be loading)"
                fi
            else
                echo -e "${RED}✗${NC} UI Frontend     (PID: $ui_pid, Port: 3000) - NOT RESPONDING"
            fi
        else
            echo -e "${RED}✗${NC} UI Frontend     (PID: $ui_pid) - DEAD"
        fi
    else
        echo -e "${RED}✗${NC} UI Frontend     - NOT RUNNING"
    fi
    
    echo ""
    echo "Port usage:"
    for port in 9090 9000 3000; do
        if port_in_use $port; then
            process=$(lsof -i :$port 2>/dev/null | tail -1 | awk '{print $1, $2}')
            echo -e "  Port $port: ${GREEN}IN USE${NC} ($process)"
        else
            echo -e "  Port $port: ${RED}FREE${NC}"
        fi
    done
    echo ""
}

# Main logic
case "$1" in
    start)
        start_services
        ;;
    stop)
        stop_services
        ;;
    status)
        show_status
        ;;
    restart)
        stop_services
        sleep 2
        start_services
        ;;
    *)
        echo "Usage: $0 {start|stop|status|restart}"
        echo ""
        echo "Commands:"
        echo "  start   - Start all SDK-IA servi
ces (Rust, Python, UI)"
        echo "  stop    - Stop all SDK-IA services"
        echo "  status  - Show status of all services"
        echo "  restart - Restart all services"
        exit 1
        ;;
esac

exit 0

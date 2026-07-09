# SDK-IA - Unified Agent Console Setup

## Phase 3 Completion: Unified Agent Console

This document provides instructions to set up and test the unified agent console locally.

---

## Prerequisites

- Git
- Rust (latest stable)
- Node.js 18+ (for UI)
- Python 3.9+ (for Secretario service)
- SQLite

---

## 1. Clone the Repository

```bash
cd /home/user
git clone https://github.com/alvrui/SDK-IA.git
cd SDK-IA
```

---

## 2. Set Up Environment Variables

### For Rust Backend (port 9090)

Create .env file in the root directory:

```bash
SDK_IA_SERVER_HOST=127.0.0.1
SDK_IA_SERVER_PORT=9090
SDK_IA_PYTHON_SERVICE_URL=http://127.0.0.1:9000
SDK_IA_DATABASE_PATH=data/sdk-ia.db
SDK_IA_DEBUG=true
```

### For Python Service (port 9000)

Create .env file in python/secretario/ directory:

```bash
SERVER_HOST=127.0.0.1
SERVER_PORT=9000
DEBUG=true
MISTRAL_API_KEY=your_mistral_api_key_here
MISTRAL_API_URL=https://api.mistral.ai/v1
MISTRAL_MODEL=mistral-small
DATABASE_PATH=data/agents.db
```

**IMPORTANT**: You need a Mistral API key from https://console.mistral.ai/

---

## 3. Install Dependencies

### Rust Backend
```bash
cargo build
```

### Python Service
```bash
cd python/secretario
pip install -r ../requirements.txt
cd ../..
```

### UI Frontend
```bash
cd ui
npm install
cd ..
```

---

## 4. Create Data Directories

```bash
mkdir -p data/hollywood_animal
mkdir -p data/sdk-ia
```

---

## 5. Start Services

### Terminal 1: Python Secretario Service (port 9000)
```bash
cd python/secretario
python main.py
```

### Terminal 2: Rust Backend (port 9090)
```bash
cargo run
```

### Terminal 3: UI Frontend (port 3000)
```bash
cd ui
npm run dev
```

---

## 6. Access the Application

- UI: http://localhost:3000
- Rust API: http://localhost:9090/api/v1/internal/health
- Python API: http://localhost:9000/api/v1/internal/health

---

## 7. Test the Unified Agent Console

1. Go to http://localhost:3000
2. Click on Agents in the navigation
3. You should see the Agent Console

### Creating an Agent
1. Click + New Agent
2. Fill the form and click Create

### Testing Chat
1. Select an agent
2. Type a message and press Enter
3. The agent should respond

---

## 8. Run Tests

```bash
cargo test
```

---

## 9. API Endpoints

### Rust Backend
- GET /api/v1/internal/agents - List agents
- POST /api/v1/internal/agents - Create agent
- GET /api/v1/internal/agents/{id} - Get agent
- PUT /api/v1/internal/agents/{id} - Update agent
- DELETE /api/v1/internal/agents/{id} - Delete agent
- POST /api/v1/internal/agents/{id}/messages - Send message

### Python Service
- Same endpoints as above

---

## 10. Troubleshooting

- Python: Ensure MISTRAL_API_KEY is set in python/secretario/.env
- Rust: Check Python service is running on port 9000
- UI: Check Node.js is installed and dependencies are installed

---

## 11. Phase 3 Completion

All Phase 3 tasks are now complete:
- Complete project management
- AI-assisted generation (Hollywood Animal)
- Domain validation
- Logging system
- Unified agent console (NEW)

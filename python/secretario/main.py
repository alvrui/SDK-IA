"""
Main FastAPI application for Secretario module
"""

from datetime import datetime
from fastapi import FastAPI, HTTPException, WebSocket, WebSocketDisconnect, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import HTMLResponse, JSONResponse
from fastapi.staticfiles import StaticFiles
from typing import List, Dict, Any, Optional
import json
import os
import structlog

from .agents import AgentManager, router as agents_router
from .mistral_client import MistralClient
from .storage import ConversationStorage
from .config import settings

# WebSocket Manager
class ConnectionManager:
    def __init__(self):
        self.active_connections: List[WebSocket] = []

    async def connect(self, websocket: WebSocket):
        await websocket.accept()
        self.active_connections.append(websocket)

    def disconnect(self, websocket: WebSocket):
        self.active_connections.remove(websocket)

    async def send_to(self, message: str, websocket: WebSocket):
        await websocket.send_text(message)

manager = ConnectionManager()

# Configure logging
structlog.configure(
    processors=[
        structlog.processors.JSONRenderer()
    ],
    logger_factory=structlog.PrintLoggerFactory(),
)
logger = structlog.get_logger()

app = FastAPI(
    title="Secretario API",
    description="Mistral Agent Management Service for Cadiz12 Project",
    version="0.1.0",
    docs_url="/api/v1/internal/docs",
    redoc_url="/api/v1/internal/redoc",
    openapi_url="/api/v1/internal/openapi.json",
)

# CORS configuration
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:9090", "http://127.0.0.1:9090"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Initialize services
mistral_client = MistralClient()
storage = ConversationStorage(database_path=settings.DATABASE_PATH)
agent_manager = AgentManager(storage=storage, client=mistral_client)

# Include routers
app.include_router(agents_router, prefix="/api/v1/internal/agents")


@app.get("/api/v1/internal/health")
async def health_check():
    """Health check endpoint"""
    logger.info("Health check requested")
    return {
        "status": "healthy",
        "service": "secretario",
        "version": "0.1.0",
        "timestamp": "2026-07-08T00:00:00Z",
    }

# ===== Endpoints para el frontend React en /api/v1/agents =====

@app.get("/api/v1/agents")
async def list_agents():
    """Listar todos los agentes (para el frontend React)"""
    agents_list = []
    all_agents = await storage.list_agents()
    for agent in all_agents:
        agents_list.append({
            "id": agent.id,
            "name": agent.name,
            "status": "active",
            "model": agent.model,
            "description": agent.description,
            "temperature": agent.temperature,
            "max_tokens": agent.max_tokens,
            "system_prompt": agent.system_prompt,
        })
    return {"agents": agents_list}

@app.get("/api/v1/agents/{agent_id}")
async def get_agent(agent_id: str):
    """Obtener un agente específico"""
    agent = await storage.get_agent(agent_id)
    if not agent:
        raise HTTPException(status_code=404, detail="Agent not found")
    return {
        "id": agent.id,
        "name": agent.name,
        "status": "active",
        "model": agent.model,
        "description": agent.description,
        "temperature": agent.temperature,
        "max_tokens": agent.max_tokens,
        "system_prompt": agent.system_prompt,
    }

@app.post("/api/v1/agents")
async def create_agent(request: Request):
    """Crear un nuevo agente"""
    data = await request.json()
    agent_id = data.get("id") or data.get("agent_id")
    name = data.get("name")
    if not agent_id or not name:
        raise HTTPException(status_code=400, detail="agent_id and name are required")
    
    from .schemas import AgentCreate
    agent_data = AgentCreate(
        name=name,
        description=data.get("description", ""),
        model=data.get("model", "mistral_small"),
        system_prompt=data.get("system_prompt", ""),
        temperature=data.get("temperature", 0.7),
        max_tokens=data.get("max_tokens", 4096),
    )
    agent = await agent_manager.create_agent(agent_data)
    return {
        "id": agent.id,
        "name": agent.name,
        "status": "active",
        "model": agent.model,
        "description": agent.description,
        "temperature": agent.temperature,
        "max_tokens": agent.max_tokens,
        "system_prompt": agent.system_prompt,
    }

@app.put("/api/v1/agents/{agent_id}")
async def update_agent(agent_id: str, request: Request):
    """Actualizar un agente"""
    data = await request.json()
    agent = await storage.get_agent(agent_id)
    if not agent:
        raise HTTPException(status_code=404, detail="Agent not found")
    
    from .schemas import AgentUpdate
    update_data = AgentUpdate(
        name=data.get("name", agent.name),
        description=data.get("description", agent.description),
        model=data.get("model", agent.model),
        system_prompt=data.get("system_prompt", agent.system_prompt),
        temperature=data.get("temperature", agent.temperature),
        max_tokens=data.get("max_tokens", agent.max_tokens),
    )
    updated_agent = await agent_manager.update_agent(agent_id, update_data)
    return {
        "id": updated_agent.id,
        "name": updated_agent.name,
        "status": "active",
        "model": updated_agent.model,
        "description": updated_agent.description,
        "temperature": updated_agent.temperature,
        "max_tokens": updated_agent.max_tokens,
        "system_prompt": updated_agent.system_prompt,
    }

@app.delete("/api/v1/agents/{agent_id}")
async def delete_agent(agent_id: str):
    """Eliminar un agente"""
    agent = await storage.get_agent(agent_id)
    if not agent:
        raise HTTPException(status_code=404, detail="Agent not found")
    await storage.delete_agent(agent_id)
    return {"status": "success", "message": f"Agent {agent_id} deleted"}

@app.post("/api/v1/agents/{agent_id}/messages")
async def send_agent_message(agent_id: str, request: Request):
    """Enviar mensaje a un agente (para el frontend React)"""
    data = await request.json()
    message_content = data.get("message")
    conversation_id = data.get("conversation_id")
    
    if not message_content:
        raise HTTPException(status_code=400, detail="message is required")
    
    from .schemas import MessageCreate
    message = MessageCreate(
        agent_id=agent_id,
        conversation_id=conversation_id,
        content=message_content
    )
    result = await agent_manager.send_message(agent_id, message)
    if result:
        return {
            "response": result.content,
            "conversation_id": result.conversation_id,
            "agent_id": agent_id,
            "status": "success"
        }
    raise HTTPException(status_code=500, detail="Failed to get response from agent")


@app.on_event("startup")
async def startup_event():
    """Startup event handler"""
    logger.info("Secretario service starting", port=settings.SERVER_PORT)
    await storage.initialize()
    
    # Cargar agentes desde .env_agentes
    env_file = os.path.join(os.path.dirname(__file__), '.env_agentes')
    if os.path.exists(env_file):
        with open(env_file, 'r') as f:
            for line in f:
                line = line.strip()
                if line and '=' in line and not line.startswith('#'):
                    parts = line.split('=', 1)
                    if len(parts) == 2:
                        agent_id = parts[0].strip()
                        name = parts[1].strip().split(',')[0].strip()
                        from .models import Agent as AgentModel
                        agent = AgentModel(
                            id=agent_id,
                            name=name,
                            description="",
                            model="mistral_small",
                            system_prompt="",
                            temperature=0.7,
                            max_tokens=4096,
                        )
                        await storage.save_agent(agent)
                        logger.info("Agent loaded from .env_agentes", agent_id=agent_id, name=name)


@app.on_event("shutdown")
async def shutdown_event():
    """Shutdown event handler"""
    logger.info("Secretario service shutting down")

# --- WebSocket Endpoint para chat en tiempo real ---
@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await manager.connect(websocket)
    try:
        while True:
            data = await websocket.receive_text()
            message_data = json.loads(data)

            if message_data["type"] == "ping":
                from .schemas import MessageCreate
                message = MessageCreate(
                    agent_id=message_data.get("agent_id", ""),
                    conversation_id=None,
                    content="ping"
                )
                response = await agent_manager.send_message(message_data.get("agent_id", ""), message)
                if response:
                    response_dict = {"response": response.content, "conversation_id": response.conversation_id, "status": "success"}
                else:
                    response_dict = {"status": "error", "error": "No response"}
            elif message_data["type"] == "message":
                from .schemas import MessageCreate
                message = MessageCreate(
                    agent_id=message_data["agent_id"],
                    conversation_id=message_data.get("conversation_id"),
                    content=message_data["content"]
                )
                response = await agent_manager.send_message(message_data["agent_id"], message)
                if response:
                    response_dict = {"response": response.content, "conversation_id": response.conversation_id, "status": "success"}
                else:
                    response_dict = {"status": "error", "error": "No response"}
            else:
                response_dict = {"status": "error", "error": "Tipo de mensaje no soportado"}

            timestamp = datetime.now().strftime("%H:%M %d-%m-%y")
            response_message = {
                "type": "response",
                "timestamp": timestamp,
                "content": response_dict.get("response", "Error desconocido"),
                "conversation_id": response_dict.get("conversation_id", ""),
                "status": response_dict.get("status", "error")
            }
            await manager.send_to(json.dumps(response_message, ensure_ascii=False), websocket)
    except WebSocketDisconnect:
        manager.disconnect(websocket)

# --- Endpoints HTTP para gestión de agentes ---
@app.get("/agentes")
async def list_agentes():
    agents = await agent_manager.list_agents()
    return [{"nombre": a.name, "agent_id": a.id} for a in agents]

@app.post("/save_agent")
async def save_agent(request: Request):
    form_data = await request.form()
    agent_name = form_data.get("agent_name")
    agent_id = form_data.get("agent_id")
    if agent_name and agent_id:
        from .schemas import AgentCreate
        agent_data = AgentCreate(
            name=agent_name,
            description="",
            model="mistral_small",
            system_prompt="",
            temperature=0.7,
            max_tokens=4096
        )
        await agent_manager.create_agent(agent_data)
        return {"status": "success"}
    return {"status": "error", "error": "Faltan datos"}

@app.post("/send_message")
async def send_message(request: Request):
    form_data = await request.form()
    conversation_id = form_data.get("conversation_id")
    agent_id = form_data.get("agent_id")
    message = form_data.get("message")

    if not agent_id or not message:
        return {"status": "error", "error": "Faltan datos obligatorios"}

    from .schemas import MessageCreate
    message_data = MessageCreate(
        agent_id=agent_id,
        conversation_id=conversation_id,
        content=message
    )
    result = await agent_manager.send_message(agent_id, message_data)
    if result:
        return {"status": "success", "response": result.content, "conversation_id": result.conversation_id}
    return {"status": "error", "error": "Failed to send message"}


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(
        "secretario.main:app",
        host=settings.SERVER_HOST,
        port=settings.SERVER_PORT,
        reload=settings.DEBUG,
        log_config=None,
    )

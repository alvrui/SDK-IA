"""
Main FastAPI application for Secretario module
"""

from datetime import datetime
from fastapi import FastAPI, HTTPException, WebSocket, WebSocketDisconnect, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from typing import List
import json
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
mistral_client = MistralClient(api_key=settings.MISTRAL_API_KEY)
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


@app.on_event("startup")
async def startup_event():
    """Startup event handler"""
    logger.info("Secretario service starting", port=settings.SERVER_PORT)
    await storage.initialize()


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

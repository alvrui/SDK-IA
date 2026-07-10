"""
Main FastAPI application for Secretario module
"""

from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
import structlog

from .agents import AgentManager, router as agents_router
from .mistral_client import MistralClient
from .storage import ConversationStorage
from .config import settings

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
    docs_url="/api/v1/docs",
    redoc_url="/api/v1/redoc",
    openapi_url="/api/v1/openapi.json",
)

# CORS configuration - Allow frontend (3000), Rust backend (9090), and direct access
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:3000", "http://127.0.0.1:3000", "http://localhost:9090", "http://127.0.0.1:9090"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Initialize services
mistral_client = MistralClient(api_key=settings.MISTRAL_API_KEY)
storage = ConversationStorage(database_path=settings.DATABASE_PATH)
agent_manager = AgentManager(storage=storage, client=mistral_client)

# Include routers - Use /api/v1/agents (not /api/v1/internal/agents) to match frontend calls
app.include_router(agents_router, prefix="/api/v1/agents")


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


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(
        "secretario.main:app",
        host=settings.SERVER_HOST,
        port=settings.SERVER_PORT,
        reload=settings.DEBUG,
        log_config=None,
    )
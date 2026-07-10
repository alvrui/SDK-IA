"""
Pydantic schemas for request/response validation
"""

from typing import Optional, Dict, Any
from pydantic import BaseModel, Field, ConfigDict


class AgentCreate(BaseModel):
    """Schema for creating a new agent"""
    
    name: str = Field(..., description="Name of the agent", min_length=1, max_length=100)
    description: Optional[str] = Field(None, description="Description of the agent", max_length=500)
    model: str = Field("mistral-tiny", description="Mistral model identifier")
    system_prompt: Optional[str] = Field(None, description="System prompt for the agent", max_length=2000)
    temperature: float = Field(0.7, description="Temperature for generation", ge=0.0, le=2.0)
    max_tokens: int = Field(1000, description="Maximum tokens for generation", ge=1, le=32000)

    model_config = ConfigDict(json_schema_extra={
        "example": {
            "name": "Historical Expert",
            "description": "Expert in 19th century Spanish history",
            "model": "mistral-small",
            "system_prompt": "You are an expert in Spanish history, specifically the early 19th century.",
            "temperature": 0.3,
            "max_tokens": 2000,
        }
    })


class AgentUpdate(BaseModel):
    """Schema for updating an existing agent"""
    
    name: Optional[str] = Field(None, description="Name of the agent", min_length=1, max_length=100)
    description: Optional[str] = Field(None, description="Description of the agent", max_length=500)
    system_prompt: Optional[str] = Field(None, description="System prompt for the agent", max_length=2000)
    temperature: Optional[float] = Field(None, description="Temperature for generation", ge=0.0, le=2.0)
    max_tokens: Optional[int] = Field(None, description="Maximum tokens for generation", ge=1, le=32000)

    model_config = ConfigDict(json_schema_extra={
        "example": {
            "name": "Updated Historian",
            "temperature": 0.5,
        }
    })


class MessageCreate(BaseModel):
    """Schema for creating a new message"""
    
    conversation_id: str = Field(..., description="ID of the conversation")
    content: str = Field(..., description="Content of the message", min_length=1, max_length=10000)

    model_config = ConfigDict(json_schema_extra={
        "example": {
            "conversation_id": "conv_12345",
            "content": "Tell me about the Siege of Cadiz in 1812",
        }
    })


class ConversationCreate(BaseModel):
    """Schema for creating a new conversation"""
    
    agent_id: str = Field(..., description="ID of the agent")
    title: Optional[str] = Field(None, description="Title of the conversation", max_length=200)

    model_config = ConfigDict(json_schema_extra={
        "example": {
            "agent_id": "agent_12345",
            "title": "Discussion about Cadiz 1812",
        }
    })


class HealthResponse(BaseModel):
    """Schema for health check response"""
    
    status: str = Field(..., description="Health status")
    service: str = Field(..., description="Service name")
    version: str = Field(..., description="Service version")
    timestamp: str = Field(..., description="Timestamp of the check")


class ErrorResponse(BaseModel):
    """Schema for error responses"""
    
    status: str = Field(..., description="Error status")
    error: str = Field(..., description="Error message")
    details: Optional[Dict[str, Any]] = Field(None, description="Additional error details")

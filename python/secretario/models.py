"""
Data models for Secretario module
"""

from typing import Dict, Any, Optional, List
from pydantic import BaseModel, Field
from datetime import datetime


class Agent(BaseModel):
    """Represents a Mistral AI agent"""
    
    id: str = Field(..., description="Unique identifier for the agent")
    name: str = Field(..., description="Name of the agent")
    description: Optional[str] = Field(None, description="Description of the agent")
    model: str = Field(..., description="Mistral model identifier")
    system_prompt: Optional[str] = Field(None, description="System prompt for the agent")
    temperature: float = Field(0.7, description="Temperature for generation")
    max_tokens: int = Field(1000, description="Maximum tokens for generation")
    created_at: Optional[str] = Field(None, description="Creation timestamp")
    updated_at: Optional[str] = Field(None, description="Last update timestamp")


class Conversation(BaseModel):
    """Represents a conversation with an agent"""
    
    id: str = Field(..., description="Unique identifier for the conversation")
    agent_id: str = Field(..., description="ID of the agent in this conversation")
    title: Optional[str] = Field(None, description="Title of the conversation")
    created_at: Optional[str] = Field(None, description="Creation timestamp")
    updated_at: Optional[str] = Field(None, description="Last update timestamp")


class Message(BaseModel):
    """Represents a message in a conversation"""
    
    id: str = Field(..., description="Unique identifier for the message")
    conversation_id: str = Field(..., description="ID of the conversation")
    agent_id: str = Field(..., description="ID of the agent")
    content: str = Field(..., description="Content of the message")
    role: str = Field(..., description="Role of the message sender (user/assistant/system)")
    timestamp: str = Field(..., description="Timestamp of the message")
    metadata: Optional[Dict[str, Any]] = Field(None, description="Additional metadata")


class LogEntry(BaseModel):
    """Represents a log entry"""
    
    id: str = Field(..., description="Unique identifier for the log entry")
    timestamp: str = Field(..., description="Timestamp of the log entry")
    level: str = Field(..., description="Log level (info, warning, error, debug)")
    service: str = Field(..., description="Service name")
    message: str = Field(..., description="Log message")
    details: Optional[Dict[str, Any]] = Field(None, description="Additional details")

'""'
Secretario Module for Mistral Agent Management

This module provides functionality for managing Mistral AI agents,
storing conversations, and exposing an HTTP interface for
communication with the SDK-eventos-cadiz12 Rust backend.
"""

__version__ = "0.1.0"
__author__ = "alvrui"

from .main import app
from .agents import AgentManager
from .mistral_client import MistralClient
from .storage import ConversationStorage
from .models import Agent, Conversation, Message
from .schemas import (
    AgentCreate,
    AgentUpdate,
    MessageCreate,
    ConversationCreate,
)

__all__ = [
    "app",
    "AgentManager",
    "MistralClient",
    "ConversationStorage",
    "Agent",
    "Conversation",
    "Message",
    "AgentCreate",
    "AgentUpdate",
    "MessageCreate",
    "ConversationCreate",
]

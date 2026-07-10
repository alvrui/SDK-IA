url: https://raw.githubusercontent.com/alvrui/SDK-IA/main/python/secretario/agents.py

"""
Agent management for Secretario module
"""

from fastapi import APIRouter, Depends, HTTPException, status
from typing import List, Optional
import structlog

from .models import Agent, Conversation, Message
from .schemas import AgentCreate, AgentUpdate, MessageCreate
from .storage import ConversationStorage
from .mistral_client import MistralClient

logger = structlog.get_logger()

router = APIRouter(tags=["agents"])


class AgentManager:
    """Manages Mistral AI agents and their conversations"""

    def __init__(self, storage: ConversationStorage, client: MistralClient):
        self.storage = storage
        self.client = client

    async def create_agent(self, agent_data: AgentCreate) -> Agent:
        """Create a new agent"""
        logger.info("Creating new agent", name=agent_data.name)
        
        agent = Agent(
            id=f"agent_{self.storage.generate_id()}",
            name=agent_data.name,
            description=agent_data.description,
            model=agent_data.model,
            system_prompt=agent_data.system_prompt,
            temperature=agent_data.temperature,
            max_tokens=agent_data.max_tokens,
        )
        
        await self.storage.save_agent(agent)
        logger.info("Agent created", agent_id=agent.id)
        return agent

    async def get_agent(self, agent_id: str) -> Optional[Agent]:
        """Get an agent by ID"""
        logger.debug("Getting agent", agent_id=agent_id)
        return await self.storage.get_agent(agent_id)

    async def list_agents(self) -> List[Agent]:
        """List all agents"""
        logger.debug("Listing all agents")
        return await self.storage.list_agents()

    async def update_agent(self, agent_id: str, agent_data: AgentUpdate) -> Optional[Agent]:
        """Update an existing agent"""
        logger.info("Updating agent", agent_id=agent_id)
        
        agent = await self.storage.get_agent(agent_id)
        if not agent:
            logger.warni
ng("Agent not found", agent_id=agent_id)
            return None
        
        if agent_data.name is not None:
            agent.name = agent_data.name
        if agent_data.description is not None:
            agent.description = agent_data.description
        if agent_data.system_prompt is not None:
            agent.system_prompt = agent_data.system_prompt
        if agent_data.temperature is not None:
            agent.temperature = agent_data.temperature
        if agent_data.max_tokens is not None:
            agent.max_tokens = agent_data.max_tokens
        
        await self.storage.save_agent(agent)
        logger.info("Agent updated", agent_id=agent_id)
        return agent

    async def delete_agent(self, agent_id: str) -> bool:
        """Delete an agent"""
        logger.info("Deleting agent", agent_id=agent_id)
        
        result = await self.storage.delete_agent(agent_id)
        if result:
            logger.info("Agent deleted", agent_id=agent_id)
        else:
            logger.warning("Agent not found for deletion", agent_id=agent_id)
        return result

    async def send_message(self, agent_id: str, message_data: MessageCreate) -> Optional[Message]:
        """Send a message to an agent and get response"""
        logger.info("Sending message to agent", agent_id=agent_id)
        
        agent = await self.storage.get_agent(agent_id)
        if not agent:
            logger.warning("Agent not found", agent_id=agent_id)
            return None
        
        # Create user message
        user_message = Message(
            id=f"msg_{self.storage.generate_id()}",
            conversation_id=message_data.conversation_id,
            agent_id=agent_id,
            content=message_data.content,
            role="user",
            timestamp="2026-07-08T00:00:00Z",
        )
        
        await self.storage.save_message(user_message)
        
        # Get response from Mistral
        response_content = await self.client.chat(
   
         model=agent.model,
            messages=[
                {"role": "system", "content": agent.system_prompt},
                {"role": "user", "content": message_data.content},
            ],
            temperature=agent.temperature,
            max_tokens=agent.max_tokens,
        )
        
        # Create assistant message
        assistant_message = Message(
            id=f"msg_{self.storage.generate_id()}",
            conversation_id=message_data.conversation_id,
            agent_id=agent_id,
            content=response_content,
            role="assistant",
            timestamp="2026-07-08T00:00:00Z",
        )
        
        await self.storage.save_message(assistant_message)
        logger.info("Message sent and response received", agent_id=agent_id)
        return assistant_message

    async def get_conversation(self, conversation_id: str) -> Optional[Conversation]:
        """Get a conversation by ID"""
        logger.debug("Getting conversation", conversation_id=conversation_id)
        return await self.storage.get_conversation(conversation_id)

    async def list_conversations(self, agent_id: Optional[str] = None) -> List[Conversation]:
        """List conversations, optionally filtered by agent"""
        logger.debug("Listing conversations", agent_id=agent_id)
        return await self.storage.list_conversations(agent_id)


# Dependency for AgentManager
def get_agent_manager(storage: ConversationStorage = Depends(), client: MistralClient = Depends()) -> AgentManager:
    return AgentManager(storage=storage, client=client)


@router.post("/", response_model=Agent, status_code=status.HTTP_201_CREATED)
async def create_agent_endpoint(
    agent_data: AgentCreate,
    manager: AgentManager = Depends(get_agent_manager),
) -> Agent:
    """Create a new agent"""
    agent = await manager.create_agent(agent_data)
    if not agent:
        raise HTTPException(status_code=400, detail="Failed to create agent")
    return agent


@router.get("/{
agent_id}", response_model=Agent)
async def get_agent_endpoint(
    agent_id: str,
    manager: AgentManager = Depends(get_agent_manager),
) -> Agent:
    """Get an agent by ID"""
    agent = await manager.get_agent(agent_id)
    if not agent:
        raise HTTPException(status_code=404, detail="Agent not found")
    return agent


@router.get("/", response_model=List[Agent])
async def list_agents_endpoint(
    manager: AgentManager = Depends(get_agent_manager),
) -> List[Agent]:
    """List all agents"""
    return await manager.list_agents()


@router.put("/{agent_id}", response_model=Agent)
async def update_agent_endpoint(
    agent_id: str,
    agent_data: AgentUpdate,
    manager: AgentManager = Depends(get_agent_manager),
) -> Agent:
    """Update an existing agent"""
    agent = await manager.update_agent(agent_id, agent_data)
    if not agent:
        raise HTTPException(status_code=404, detail="Agent not found")
    return agent


@router.delete("/{agent_id}", status_code=status.HTTP_204_NO_CONTENT)
async def delete_agent_endpoint(
    agent_id: str,
    manager: AgentManager = Depends(get_agent_manager),
) -> None:
    """Delete an agent"""
    result = await manager.delete_agent(agent_id)
    if not result:
        raise HTTPException(status_code=404, detail="Agent not found")


@router.post("/{agent_id}/messages", response_model=Message, status_code=status.HTTP_201_CREATED)
async def send_message_endpoint(
    agent_id: str,
    message_data: MessageCreate,
    manager: AgentManager = Depends(get_agent_manager),
) -> Message:
    """Send a message to an agent"""
    message = await manager.send_message(agent_id, message_data)
    if not message:
        raise HTTPException(status_code=404, detail="Agent not found")
    return message

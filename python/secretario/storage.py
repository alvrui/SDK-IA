"""
Conversation storage for Secretario module
"""

import aiosqlite
import json
import structlog
from typing import List, Optional, Dict, Any
from pathlib import Path
import uuid

from .models import Agent, Conversation, Message

logger = structlog.get_logger()


class ConversationStorage:
    """SQLite-based storage for agents, conversations, and messages"""

    def __init__(self, database_path: str = "data/agents.db"):
        self.database_path = database_path
        self._connection: Optional[aiosqlite.Connection] = None

    async def initialize(self) -> None:
        """Initialize the database and create tables"""
        logger.info("Initializing database", path=self.database_path)
        
        # Ensure data directory exists
        Path(self.database_path).parent.mkdir(parents=True, exist_ok=True)
        
        async with aiosqlite.connect(self.database_path) as conn:
            # Create agents table
            await conn.execute("""
                CREATE TABLE IF NOT EXISTS agents (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    description TEXT,
                    model TEXT NOT NULL,
                    system_prompt TEXT,
                    temperature REAL DEFAULT 0.7,
                    max_tokens INTEGER DEFAULT 1000,
                    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
                )
            """)
            
            # Create conversations table
            await conn.execute("""
                CREATE TABLE IF NOT EXISTS conversations (
                    id TEXT PRIMARY KEY,
                    agent_id TEXT NOT NULL,
                    title TEXT,
                    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (agent_id) REFERENCES agents(id)
                )
            """)
            
            # Create messages table
            await conn.execute("""
                CREATE TABLE IF NOT EXISTS messages (
                    id TEXT PRIMARY KEY,
                    conversation_id TEXT NOT NULL,
                    agent_id TEXT NOT NULL,
                    content TEXT NOT NULL,
                    role TEXT NOT NULL,
                    timestamp TEXT DEFAULT CURRENT_TIMESTAMP,
                    metadata TEXT,
                    FOREIGN KEY (conversation_id) REFERENCES conversations(id),
                    FOREIGN KEY (agent_id) REFERENCES agents(id)
                )
            """)
            
            # Create indexes
            await conn.execute("CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(conversation_id)")
            await conn.execute("CREATE INDEX IF NOT EXISTS idx_messages_agent ON messages(agent_id)")
            
            await conn.commit()
        
        logger.info("Database initialized successfully")

    def generate_id(self) -> str:
        """Generate a unique ID"""
        return str(uuid.uuid4())

    async def save_agent(self, agent: Agent) -> None:
        """Save an agent to the database"""
        logger.debug("Saving agent", agent_id=agent.id)
        
        async with aiosqlite.connect(self.database_path) as conn:
            await conn.execute("""
                INSERT OR REPLACE INTO agents 
                (id, name, description, model, system_prompt, temperature, max_tokens, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            """, (
                agent.id,
                agent.name,
                agent.description,
                agent.model,
                agent.system_prompt,
                agent.temperature,
                agent.max_tokens,
            ))
            await conn.commit()

    async def get_agent(self, agent_id: str) -> Optional[Agent]:
        """Get an agent by ID"""
        logger.debug("Getting agent", agent_id=agent_id)
        
        async with aiosqlite.connect(self.database_path) as conn:
            conn.row_factory = aiosqlite.Row
            async with conn.execute(
                "SELECT * FROM agents WHERE id = ?", (agent_id,)
            ) as cursor:
                row = await cursor.fetchone()
                if row:
                    return Agent(
                        id=row["id"],
                        name=row["name"],
                        description=row["description"],
                        model=row["model"],
                        system_prompt=row["system_prompt"],
                        temperature=row["temperature"],
                        max_tokens=row["max_tokens"],
                    )
        return None

    async def list_agents(self) -> List[Agent]:
        """List all agents"""
        logger.debug("Listing all agents")
        
        agents = []
        async with aiosqlite.connect(self.database_path) as conn:
            conn.row_factory = aiosqlite.Row
            async with conn.execute("SELECT * FROM agents ORDER BY name") as cursor:
                async for row in cursor:
                    agents.append(Agent(
                        id=row["id"],
                        name=row["name"],
                        description=row["description"],
                        model=row["model"],
                        system_prompt=row["system_prompt"],
                        temperature=row["temperature"],
                        max_tokens=row["max_tokens"],
                    ))
        return agents

    async def delete_agent(self, agent_id: str) -> bool:
        """Delete an agent and all its conversations and messages"""
        logger.info("Deleting agent", agent_id=agent_id)
        
        async with aiosqlite.connect(self.database_path) as conn:
            # Delete messages first
            await conn.execute("DELETE FROM messages WHERE agent_id = ?", (agent_id,))
            
            # Delete conversations
            await conn.execute("DELETE FROM conversations WHERE agent_id = ?", (agent_id,))
            
            # Delete agent
            cursor = await conn.execute("DELETE FROM agents WHERE id = ?", (agent_id,))
            await conn.commit()
            return cursor.rowcount > 0

    async def save_conversation(self, conversation: Conversation) -> None:
        """Save a conversation to the database"""
        logger.debug("Saving conversation", conversation_id=conversation.id)
        
        async with aiosqlite.connect(self.database_path) as conn:
            await conn.execute("""
                INSERT OR REPLACE INTO conversations 
                (id, agent_id, title, created_at, updated_at)
                VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            """, (
                conversation.id,
                conversation.agent_id,
                conversation.title,
            ))
            await conn.commit()

    async def get_conversation(self, conversation_id: str) -> Optional[Conversation]:
        """Get a conversation by ID"""
        logger.debug("Getting conversation", conversation_id=conversation_id)
        
        async with aiosqlite.connect(self.database_path) as conn:
            conn.row_factory = aiosqlite.Row
            async with conn.execute(
                "SELECT * FROM conversations WHERE id = ?", (conversation_id,)
            ) as cursor:
                row = await cursor.fetchone()
                if row:
                    return Conversation(
                        id=row["id"],
                        agent_id=row["agent_id"],
                        title=row["title"],
                        created_at=row["created_at"],
                        updated_at=row["updated_at"],
                    )
        return None

    async def list_conversations(self, agent_id: Optional[str] = None) -> List[Conversation]:
        """List conversations, optionally filtered by agent"""
        logger.debug("Listing conversations", agent_id=agent_id)
        
        conversations = []
        async with aiosqlite.connect(self.database_path) as conn:
            conn.row_factory = aiosqlite.Row
            
            if agent_id:
                query = "SELECT * FROM conversations WHERE agent_id = ? ORDER BY updated_at DESC"
                async with conn.execute(query, (agent_id,)) as cursor:
                    async for row in cursor:
                        conversations.append(Conversation(
                            id=row["id"],
                            agent_id=row["agent_id"],
                            title=row["title"],
                            created_at=row["created_at"],
                            updated_at=row["updated_at"],
                        ))
            else:
                query = "SELECT * FROM conversations ORDER BY updated_at DESC"
                async with conn.execute(query) as cursor:
                    async for row in cursor:
                        conversations.append(Conversation(
                            id=row["id"],
                            agent_id=row["agent_id"],
                            title=row["title"],
                            created_at=row["created_at"],
                            updated_at=row["updated_at"],
                        ))
        return conversations

    async def save_message(self, message: Message) -> None:
        """Save a message to the database"""
        logger.debug("Saving message", message_id=message.id)
        
        async with aiosqlite.connect(self.database_path) as conn:
            metadata_json = json.dumps(message.metadata) if message.metadata else None
            
            await conn.execute("""
                INSERT INTO messages 
                (id, conversation_id, agent_id, content, role, timestamp, metadata)
                VALUES (?, ?, ?, ?, ?, ?, ?)
            """, (
                message.id,
                message.conversation_id,
                message.agent_id,
                message.content,
                message.role,
                message.timestamp,
                metadata_json,
            ))
            
            # Update conversation timestamp
            await conn.execute("""
                UPDATE conversations SET updated_at = CURRENT_TIMESTAMP WHERE id = ?
            """, (message.conversation_id,))
            
            await conn.commit()

    async def get_messages(self, conversation_id: str) -> List[Message]:
        """Get all messages for a conversation"""
        logger.debug("Getting messages", conversation_id=conversation_id)
        
        messages = []
        async with aiosqlite.connect(self.database_path) as conn:
            conn.row_factory = aiosqlite.Row
            async with conn.execute(
                "SELECT * FROM messages WHERE conversation_id = ? ORDER BY timestamp",
                (conversation_id,)
            ) as cursor:
                async for row in cursor:
                    metadata = json.loads(row["metadata"]) if row["metadata"] else {}
                    messages.append(Message(
                        id=row["id"],
                        conversation_id=row["conversation_id"],
                        agent_id=row["agent_id"],
                        content=row["content"],
                        role=row["role"],
                        timestamp=row["timestamp"],
                        metadata=metadata,
                    ))
        return messages

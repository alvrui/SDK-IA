# Secretario Module Usage

## Installation

pip install -r ../requirements.txt

## Configuration

Create .env file:
MISTRAL_API_KEY=your_key
SERVER_PORT=9000
DATABASE_PATH=data/agents.db

## Running

python -m secreto.main

## API Endpoints

### Legacy (compatible with SDK-eventos-cadiz12)
- GET /agentes - List agents
- GET /logs - Get logs
- GET /estado - System status
- POST /enviar_mensaje - Send message

### Modern (v1)
- GET /api/v1/internal/health
- GET /api/v1/internal/agents
- POST /api/v1/internal/agents
- POST /api/v1/internal/agents/{id}/messages

## Using as Library

from secreto import AgentManager, MistralClient, ConversationStorage

storage = ConversationStorage()
client = MistralClient(api_key="key")
manager = AgentManager(storage, client)

agent = await manager.create_agent(AgentCreate(name="Test", model="mistral-tiny"))
response = await manager.send_message(agent.id, MessageCreate(content="Hello"))

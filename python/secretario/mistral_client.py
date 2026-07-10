import json
import os
from typing import Dict, List, Optional, Any
from mistralai.client import Mistral

class MistralClient:
    def __init__(self):
        self.client = Mistral(api_key=os.environ.get("MISTRAL_API_KEY"))

    def send_message(
        self,
        agent_id: str,
        conversation_id: Optional[str] = None,
        message: str = ""
    ) -> Dict[str, Any]:
        try:
            # Si no hay conversation_id, crear una nueva conversación
            if not conversation_id:
                conversation = self.client.beta.conversations.start(
                    agent_id=agent_id,
                    inputs=[{"role": "user", "content": message}]
                )
                conversation_id = conversation.id

            # Enviar el mensaje a la conversación existente
            response = self.client.beta.conversations.messages.create(
                conversation_id=conversation_id,
                agent_id=agent_id,
                content=message,
                role="user"
            )

            # Obtener la respuesta del agente
            messages = self.client.beta.conversations.messages.list(
                conversation_id=conversation_id
            )
            agent_response = messages.data[-1].content if messages.data else "No response"

            return {
                "conversation_id": conversation_id,
                "response": agent_response,
                "status": "success"
            }
        except Exception as e:
            return {"error": str(e), "status": "error"}

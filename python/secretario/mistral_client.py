'""'
Mistral API client for Secretario module
"""

import aiohttp
import json
import structlog
from typing import List, Dict, Any, Optional

logger = structlog.get_logger()


class MistralClient:
    """Client for interacting with Mistral AI API"""

    def __init__(self, api_key: str, api_url: str = "https://api.mistral.ai/v1"):
        self.api_key = api_key
        self.api_url = api_url
        self.headers = {
            "Authorization": f"Bearer {self.api_key}",
            "Content-Type": "application/json",
        }

    async def chat(
        self,
        model: str,
        messages: List[Dict[str, str]],
        temperature: float = 0.7,
        max_tokens: int = 1000,
    ) -> str:
        """Send a chat completion request to Mistral API"""
        logger.info("Sending chat request", model=model, num_messages=len(messages))
        
        if not self.api_key:
            logger.warning("No Mistral API key configured, returning mock response")
            return "Mock response from Mistral AI"
        
        payload = {
            "model": model,
            "messages": messages,
            "temperature": temperature,
            "max_tokens": max_tokens,
        }
        
        try:
            async with aiohttp.ClientSession() as session:
                async with session.post(
                    f"{self.api_url}/chat/completions",
                    headers=self.headers,
                    json=payload,
                ) as response:
                    if response.status == 200:
                        data = await response.json()
                        content = data.get("choices", [{}])[0].get("message", {}).get("content", "")
                        logger.info("Chat response received", model=model)
                        return content
                    else:
                        logger.error(
                            "Chat request failed",
                            status=response.status,
                            response=await response.text(),
                        )
                        return f"Error: API request failed with status {response.status}"
        except Exception as e:
            logger.error("Chat request exception", error=str(e))
            return f"Error: {str(e)}"

    async def embeddings(
        self,
        model: str,
        input: str,
    ) -> List[float]:
        """Get embeddings for input text"""
        logger.info("Sending embeddings request", model=model)
        
        if not self.api_key:
            logger.warning("No Mistral API key configured, returning mock embeddings")
            return [0.0] * 768
        
        payload = {
            "model": model,
            "input": input,
        }
        
        try:
            async with aiohttp.ClientSession() as session:
                async with session.post(
                    f"{self.api_url}/embeddings",
                    headers=self.headers,
                    json=payload,
                ) as response:
                    if response.status == 200:
                        data = await response.json()
                        embeddings = data.get("data", [{}])[0].get("embedding", [])
                        logger.info("Embeddings response received", model=model)
                        return embeddings
                    else:
                        logger.error(
                            "Embeddings request failed",
                            status=response.status,
                            response=await response.text(),
                        )
                        return []
        except Exception as e:
            logger.error("Embeddings request exception", error=str(e))
            return []

    async def models(self) -> List[Dict[str, Any]]:
        """List available models"""
        logger.info("Sending models request")
        
        if not self.api_key:
            logger.warning("No Mistral API key configured, returning mock models")
            return [{"id": "mistral-tiny", "object": "model"}]
        
        try:
            async with aiohttp.ClientSession() as session:
                async with session.get(
                    f"{self.api_url}/models",
                    headers=self.headers,
                ) as response:
                    if response.status == 200:
                        data = await response.json()
                        models = data.get("data", [])
                        logger.info("Models response received", count=len(models))
                        return models
                    else:
                        logger.error(
                            "Models request failed",
                            status=response.status,
                            response=await response.text(),
                        )
                        return []
        except Exception as e:
            logger.error("Models request exception", error=str(e))
            return []

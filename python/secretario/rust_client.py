"""
Rust service client for internal communication
This module provides HTTP client functionality to call the Rust SDK backend
"""

import aiohttp
import json
import structlog
from typing import Any, Dict, Optional

logger = structlog.get_logger()


class RustClient:
    """Client for communicating with the Rust SDK backend"""

    def __init__(self, base_url: str = "http://127.0.0.1:9090"):
        self.base_url = base_url.rstrip("/")
        self.session: Optional[aiohttp.ClientSession] = None

    async def __aenter__(self):
        self.session = aiohttp.ClientSession()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        if self.session:
            await self.session.close()

    async def _request(
        self,
        method: str,
        endpoint: str,
        data: Optional[Dict[str, Any]] = None,
    ) -> Dict[str, Any]:
        """Make an HTTP request to the Rust backend"""
        url = f"{self.base_url}/api/v1/internal/{endpoint}"
        
        if not self.session:
            self.session = aiohttp.ClientSession()
        
        try:
            async with self.session.request(
                method,
                url,
                json=data,
                headers={"Content-Type": "application/json"},
            ) as response:
                if response.status >= 400:
                    error_text = await response.text()
                    logger.error(
                        "Rust API error",
                        method=method,
                        url=url,
                        status=response.status,
                        error=error_text,
                    )
                    return {
                        "status": "error",
                        "error": f"HTTP {response.status}: {error_text}",
                    }
                
                return await response.json()
        except Exception as e:
            logger.error("Rust API request failed", method=method, url=url, error=str(e))
            return {"status": "error", "error": str(e)}

    async def health_check(self) -> Dict[str, Any]:
        """Check Rust backend health"""
        logger.debug("Checking Rust backend health")
        return await self._request("GET", "health")

    async def get_projects(self) -> Dict[str, Any]:
        """Get all projects from Rust backend"""
        logger.debug("Getting projects from Rust backend")
        return await self._request("GET", "projects")

    async def get_project(self, project_id: str) -> Dict[str, Any]:
        """Get a specific project from Rust backend"""
        logger.debug("Getting project from Rust backend", project_id=project_id)
        return await self._request("GET", f"projects/{project_id}")

    async def create_project(
        self,
        name: str,
        description: Optional[str] = None,
    ) -> Dict[str, Any]:
        """Create a new project in Rust backend"""
        logger.debug("Creating project in Rust backend", name=name)
        return await self._request(
            "POST",
            "projects",
            {"name": name, "description": description},
        )

    async def generate_narrative(
        self,
        project_id: str,
        prompt: str,
    ) -> Dict[str, Any]:
        """Generate a narrative using Rust backend"""
        logger.debug("Generating narrative", project_id=project_id)
        return await self._request(
            "POST",
            f"projects/{project_id}/narratives",
            {"prompt": prompt},
        )

    async def generate_story_element(
        self,
        project_id: str,
        element_type: str,
        prompt: str,
    ) -> Dict[str, Any]:
        """Generate a story element using Rust backend"""
        logger.debug(
            "Generating story element",
            project_id=project_id,
            element_type=element_type,
        )
        return await self._request(
            "POST",
            f"projects/{project_id}/story-elements/{element_type}",
            {"prompt": prompt},
        )

    async def generate_event(
        self,
        project_id: str,
        prompt: str,
    ) -> Dict[str, Any]:
        """Generate an event using Rust backend"""
        logger.debug("Generating event", project_id=project_id)
        return await self._request(
            "POST",
            f"projects/{project_id}/events",
            {"prompt": prompt},
        )

    async def validate_project(self, project_id: str) -> Dict[str, Any]:
        """Validate a project using Rust backend"""
        logger.debug("Validating project", project_id=project_id)
        return await self._request("GET", f"projects/{project_id}/validate")

    async def get_system_status(self) -> Dict[str, Any]:
        """Get system status from Rust backend"""
        logger.debug("Getting system status from Rust backend")
        return await self._request("GET", "system-status")

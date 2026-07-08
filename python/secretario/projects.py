'""'
Project management for Secretario module
Handles project-related operations and proxies to Rust backend when needed
"""

from fastapi import APIRouter, Depends, HTTPException, status
from typing import List, Optional, Dict, Any
import structlog

from .models import Project, StoryElement, Event, Narrative
from .schemas import ProjectCreate, ProjectUpdate, ProjectResponse
from .storage import ConversationStorage
from .rust_client import RustClient
from .config import settings

logger = structlog.get_logger()

router = APIRouter(prefix="/api/v1/internal/projects", tags=["projects"])


class ProjectManager:
    """Manages projects and proxies to Rust backend for domain operations"""

    def __init__(self, storage: ConversationStorage, rust_client: RustClient):
        self.storage = storage
        self.rust_client = rust_client

    async def create_project(self, project_data: ProjectCreate) -> Project:
        """Create a new project (proxies to Rust backend)"""
        logger.info("Creating new project", name=project_data.name)
        
        response = await self.rust_client.create_project(
            name=project_data.name,
            description=project_data.description,
        )
        
        if response.get("status") != "success":
            logger.error("Failed to create project in Rust backend", error=response.get("error"))
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail=response.get("error", "Failed to create project")
            )
        
        project_data = response.get("data", {})
        return Project(
            id=project_data.get("id", ""),
            name=project_data.get("name", ""),
            description=project_data.get("description"),
            created_at=project_data.get("created_at"),
            updated_at=project_data.get("updated_at"),
            status=project_data.get("status", "draft"),
        )

    async def get_project(self, project_id: str) -> Optional[Project]:
        """Get a specific project (proxies to Rust backend)"""
        logger.debug("Getting project", project_id=project_id)
        
        response = await self.rust_client.get_project(project_id)
        
        if response.get("status") != "success":
            if response.get("status") == "error" and "not found" in response.get("error", "").lower():
                return None
            logger.error("Failed to get project from Rust backend", error=response.get("error"))
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail=response.get("error", "Failed to get project")
            )
        
        project_data = response.get("data", {})
        return Project(
            id=project_data.get("id", ""),
            name=project_data.get("name", ""),
            description=project_data.get("description"),
            created_at=project_data.get("created_at"),
            updated_at=project_data.get("updated_at"),
            status=project_data.get("status", "draft"),
        )

    async def list_projects(self) -> List[Project]:
        """List all projects (proxies to Rust backend)"""
        logger.debug("Listing all projects")
        
        response = await self.rust_client.get_projects()
        
        if response.get("status") != "success":
            logger.error("Failed to list projects from Rust backend", error=response.get("error"))
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail=response.get("error", "Failed to list projects")
            )
        
        projects_data = response.get("data", [])
        return [
            Project(
                id=p.get("id", ""),
                name=p.get("name", ""),
                description=p.get("description"),
                created_at=p.get("created_at"),
                updated_at=p.get("updated_at"),
                status=p.get("status", "draft"),
            )
            for p in projects_data
        ]

    async def update_project(
        self,
        project_id: str,
        project_data: ProjectUpdate,
    ) -> Optional[Project]:
        """Update a project (proxies to Rust backend)"""
        logger.info("Updating project", project_id=project_id)
        
        update_data: Dict[str, Any] = {}
        if project_data.name is not None:
            update_data["name"] = project_data.name
        if project_data.description is not None:
            update_data["description"] = project_data.description
        if project_data.status is not None:
            update_data["status"] = project_data.status
        
        response = await self.rust_client.update_project(project_id, update_data)
        
        if response.get("status") != "success":
            if response.get("status") == "error" and "not found" in response.get("error", "").lower():
                return None
            logger.error("Failed to update project in Rust backend", error=response.get("error"))
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail=response.get("error", "Failed to update project")
            )
        
        project_data = response.get("data", {})
        return Project(
            id=project_data.get("id", ""),
            name=project_data.get("name", ""),
            description=project_data.get("description"),
            created_at=project_data.get("created_at"),
            updated_at=project_data.get("updated_at"),
            status=project_data.get("status", "draft"),
        )

    async def delete_project(self, project_id: str) -> bool:
        """Delete a project (proxies to Rust backend)"""
        logger.info("Deleting project", project_id=project_id)
        
        response = await self.rust_client.delete_project(project_id)
        
        if response.get("status") == "error":
            if "not found" in response.get("error", "").lower():
                return False
            logger.error("Failed to delete project from Rust backend", error=response.get("error"))
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail=response.get("error", "Failed to delete project")
            )
        
        return True

    async def generate_narrative(
        self,
        project_id: str,
        prompt: str,
    ) -> Dict[str, Any]:
        """Generate a narrative for a project (proxies to Rust backend)"""
        logger.info("Generating narrative", project_id=project_id)
        
        response = await self.rust_client.generate_narrative(project_id, prompt)
        
        if response.get("status") != "success":
            logger.error("Failed to generate narrative", error=response.get("error"))
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail=response.get("error", "Failed to generate narrative")
            )
        
        return response.get("data", {})

    async def generate_story_element(
        self,
        project_id: str,
        element_type: str,
        prompt: str,
    ) -> Dict[str, Any]:
        """Generate a story element for a project (proxies to Rust backend)"""
        logger.info(
            "Generating story element",
            project_id=project_id,
            element_type=element_type,
        )
        
        response = await self.rust_client.generate_story_element(
            project_id, element_type, prompt
        )
        
        if response.get("status") != "success":
            logger.error("Failed to generate story element", error=response.get("error"))
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail=response.get("error", "Failed to generate story element")
            )
        
        return response.get("data", {})

    async def generate_event(
        self,
        project_id: str,
        prompt: str,
    ) -> Dict[str, Any]:
        """Generate an event for a project (proxies to Rust backend)"""
        logger.info("Generating event", project_id=project_id)
        
        response = await self.rust_client.generate_event(project_id, prompt)
        
        if response.get("status") != "success":
            logger.error("Failed to generate event", error=response.get("error"))
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail=response.get("error", "Failed to generate event")
            )
        
        return response.get("data", {})


# Dependency for ProjectManager
def get_project_manager(
    storage: ConversationStorage = Depends(),
    rust_client: RustClient = Depends(),
) -> ProjectManager:
    return ProjectManager(storage=storage, rust_client=rust_client)


# API endpoints
@router.get("/", response_model=List[ProjectResponse])
async def list_projects_endpoint(
    manager: ProjectManager = Depends(get_project_manager),
) -> List[ProjectResponse]:
    """List all projects"""
    projects = await manager.list_projects()
    return [
        ProjectResponse(
            id=p.id,
            name=p.name,
            description=p.description or "",
            status=p.status,
        )
        for p in projects
    ]


@router.post("/", response_model=ProjectResponse, status_code=status.HTTP_201_CREATED)
async def create_project_endpoint(
    project_data: ProjectCreate,
    manager: ProjectManager = Depends(get_project_manager),
) -> ProjectResponse:
    """Create a new project"""
    project = await manager.create_project(project_data)
    return ProjectResponse(
        id=project.id,
        name=project.name,
        description=project.description or "",
        status=project.status,
    )


@router.get("/{project_id}", response_model=ProjectResponse)
async def get_project_endpoint(
    project_id: str,
    manager: ProjectManager = Depends(get_project_manager),
) -> ProjectResponse:
    """Get a specific project"""
    project = await manager.get_project(project_id)
    if not project:
        raise HTTPException(status_code=404, detail="Project not found")
    return ProjectResponse(
        id=project.id,
        name=project.name,
        description=project.description or "",
        status=project.status,
    )


@router.put("/{project_id}", response_model=ProjectResponse)
async def update_project_endpoint(
    project_id: str,
    project_data: ProjectUpdate,
    manager: ProjectManager = Depends(get_project_manager),
) -> ProjectResponse:
    """Update a project"""
    project = await manager.update_project(project_id, project_data)
    if not project:
        raise HTTPException(status_code=404, detail="Project not found")
    return ProjectResponse(
        id=project.id,
        name=project.name,
        description=project.description or "",
        status=project.status,
    )


@router.delete("/{project_id}", status_code=status.HTTP_204_NO_CONTENT)
async def delete_project_endpoint(
    project_id: str,
    manager: ProjectManager = Depends(get_project_manager),
) -> None:
    """Delete a project"""
    result = await manager.delete_project(project_id)
    if not result:
        raise HTTPException(status_code=404, detail="Project not found")


@router.post("/{project_id}/narratives")
async def generate_narrative_endpoint(
    project_id: str,
    body: Dict[str, Any],
    manager: ProjectManager = Depends(get_project_manager),
) -> Dict[str, Any]:
    """Generate a narrative for a project"""
    prompt = body.get("prompt", "")
    if not prompt:
        raise HTTPException(status_code=400, detail="Prompt is required")
    return await manager.generate_narrative(project_id, prompt)


@router.post("/{project_id}/story-elements/{element_type}")
async def generate_story_element_endpoint(
    project_id: str,
    element_type: str,
    body: Dict[str, Any],
    manager: ProjectManager = Depends(get_project_manager),
) -> Dict[str, Any]:
    """Generate a story element for a project"""
    prompt = body.get("prompt", "")
    if not prompt:
        raise HTTPException(status_code=400, detail="Prompt is required")
    return await manager.generate_story_element(project_id, element_type, prompt)


@router.post("/{project_id}/events")
async def generate_event_endpoint(
    project_id: str,
    body: Dict[str, Any],
    manager: ProjectManager = Depends(get_project_manager),
) -> Dict[str, Any]:
    """Generate an event for a project"""
    prompt = body.get("prompt", "")
    if not prompt:
        raise HTTPException(status_code=400, detail="Prompt is required")
    return await manager.generate_event(project_id, prompt)

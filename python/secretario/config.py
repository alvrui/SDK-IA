"""
Configuration settings for Secretario module
"""

from pydantic_settings import BaseSettings
from typing import Optional


class Settings(BaseSettings):
    # Server configuration
    SERVER_HOST: str = "127.0.0.1"
    SERVER_PORT: int = 9000
    DEBUG: bool = False

    # Mistral API configuration
    MISTRAL_API_KEY: str = ""
    MISTRAL_API_URL: str = "https://api.mistral.ai/v1"

    # Database configuration
    DATABASE_PATH: str = "data/agents.db"

    # SDK service configuration
    SDK_SERVICE_URL: str = "http://127.0.0.1:9090"

    # Logging configuration
    LOG_LEVEL: str = "info"

    class Config:
        env_file = ".env"
        env_file_encoding = "utf-8"
        case_sensitive = False


settings = Settings()

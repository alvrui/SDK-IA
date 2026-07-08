"""
Centralized logging configuration for Secretario Python module.
Uses structlog for structured logging with JSON output option.
"""

import logging
import sys
import os
from pathlib import Path
from typing import Optional, Dict, Any
import structlog


class LoggerConfig:
    def __init__(self, level="INFO", log_file=None, json_format=False, service_name="secretario"):
        self.level = getattr(logging, level.upper(), logging.INFO)
        self.log_file = Path(log_file) if log_file else None
        self.json_format = json_format
        self.service_name = service_name
        self._logger = None

    @classmethod
    def from_env(cls):
        return cls(
            level=os.getenv("PYTHON_LOG_LEVEL", "INFO"),
            log_file=os.getenv("PYTHON_LOG_FILE", "data/logs/python.log"),
            json_format=os.getenv("PYTHON_LOG_JSON", "false").lower() == "true",
            service_name=os.getenv("SERVICE_NAME", "secretario")
        )

    def configure(self):
        if self.log_file:
            self.log_file.parent.mkdir(parents=True, exist_ok=True)

        shared_processors = [
            structlog.contextvars.merge_contextvars,
            structlog.stdlib.add_log_level,
            structlog.stdlib.PositionalArgumentsFormatter(),
            structlog.processors.TimeStamper(fmt="iso"),
            structlog.processors.StackInfoRenderer(),
            structlog.processors.format_exc_info,
        ]

        if self.json_format:
            shared_processors.append(structlog.processors.JSONRenderer())
        else:
            shared_processors.append(structlog.dev.ConsoleRenderer())

        def add_service_name(logger, method_name, event_dict):
            event_dict["service"] = self.service_name
            return event_dict

        shared_processors.insert(0, add_service_name)

        logging.basicConfig(
            level=self.level,
            handlers=[logging.StreamHandler(sys.stdout)] if not self.log_file else [
                logging.FileHandler(self.log_file),
                logging.StreamHandler(sys.stdout)
            ],
            format="%(message)s"
        )

        structlog.configure(
            processors=shared_processors,
            wrapper_class=structlog.stdlib.BoundLogger,
            logger_factory=structlog.PrintLoggerFactory(),
            cache_logger_on_first_use=True
        )
        structlog.stdlib.INSTALL_DEFAULT_LOG_LEVEL(self.level)
        self._logger = structlog.get_logger()
        self._logger.info("Python logging initialized", level=self.level)

    def get_logger(self, name=None):
        if name:
            return structlog.get_logger(name)
        if self._logger is None:
            self.configure()
        return self._logger


_logger_config = None

def configure_logging():
    global _logger_config
    if _logger_config is None:
        _logger_config = LoggerConfig.from_env()
        _logger_config.configure()
    return _logger_config

def get_logger(name=None):
    if _logger_config is None:
        configure_logging()
    return _logger_config.get_logger(name)


def log_request(logger, method, path, **kwargs):
    logger.info("Request started", method=method, path=path, **kwargs)

def log_response(logger, method, path, status, duration_ms, **kwargs):
    logger.info("Request completed", method=method, path=path, status=status, duration_ms=duration_ms, **kwargs)

def log_error(logger, error, context=None):
    context = context or {}
    logger.error("Error occurred", error=str(error), **context)

def log_db_operation(logger, operation, table, success, **kwargs):
    logger.debug("Database operation", operation=operation, table=table, success=success, **kwargs)
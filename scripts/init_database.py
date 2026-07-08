#!/usr/bin/env python3
"""
Database initialization script for SDK-IA.
This script ensures that the SQLite database is properly initialized
and can be run from either the Rust or Python side.
"""

import sqlite3
from pathlib import Path

DB_PATH = Path("data/sdk_ia.db")


def init_database():
    """Initialize the SQLite database with all required tables."""
    DB_PATH.parent.mkdir(parents=True, exist_ok=True)
    
    conn = sqlite3.connect(str(DB_PATH))
    cursor = conn.cursor()
    
    # Create all tables
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'draft',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
    """)
    
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS narratives (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            title TEXT NOT NULL,
            content TEXT,
            status TEXT NOT NULL DEFAULT 'draft',
            created_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id)
        )
    """)
    
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS story_elements (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            narrative_id TEXT,
            element_type TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            properties TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id),
            FOREIGN KEY (narrative_id) REFERENCES narratives(id)
        )
    """)
    
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS events (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            narrative_id TEXT,
            title TEXT NOT NULL,
            description TEXT,
            timestamp TEXT NOT NULL,
            location TEXT,
            participants TEXT,
            status TEXT NOT NULL DEFAULT 'draft',
            created_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id),
            FOREIGN KEY (narrative_id) REFERENCES narratives(id)
        )
    """)
    
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS agents (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            model TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'active',
            created_at TEXT NOT NULL
        )
    """)
    
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS conversations (
            id TEXT PRIMARY KEY,
            agent_id TEXT NOT NULL,
            project_id TEXT,
            title TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (agent_id) REFERENCES agents(id),
            FOREIGN KEY (project_id) REFERENCES projects(id)
        )
    """)
    
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            agent_id TEXT NOT NULL,
            conversation_id TEXT NOT NULL,
            content TEXT NOT NULL,
            role TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            metadata TEXT,
            FOREIGN KEY (agent_id) REFERENCES agents(id),
            FOREIGN KEY (conversation_id) REFERENCES conversations(id)
        )
    """)
    
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS generation_results (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            content TEXT NOT NULL,
            element_type TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            created_at TEXT NOT NULL,
            completed_at TEXT,
            FOREIGN KEY (project_id) REFERENCES projects(id)
        )
    """)
    
    conn.commit()
    conn.close()
    print(f"Database initialized successfully at {DB_PATH}")


if __name__ == "__main__":
    init_database()

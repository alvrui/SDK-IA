// Persistence service for Cadiz12 project

use crate::domain::structures::{Project, StoryElement, Event, Narrative};
use rusqlite::{Connection, Result};

pub struct PersistenceService {
    conn: Connection,
}

impl PersistenceService {
    pub fn new(database_path: &str) -> Result<Self> {
        let conn = Connection::open(database_path)?;
        Self::initialize_database(&conn)?;
        Ok(Self { conn })
    }

    fn initialize_database(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                status TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS story_elements (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                element_type TEXT NOT NULL,
                content TEXT NOT NULL,
                metadata TEXT,
                order INTEGER NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                date TEXT,
                location TEXT,
                participants TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS narratives (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                story_elements TEXT,
                events TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            )",
            [],
        )?;

        Ok(())
    }

    pub fn save_project(&self, project: &Project) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO projects (id, name, description, created_at, updated_at, status) VALUES (?, ?, ?, ?, ?, ?)",
            (
                &project.id,
                &project.name,
                &project.description.as_deref().unwrap_or(""),
                &project.created_at,
                &project.updated_at,
                &project.status,
            ),
        )?;
        Ok(())
    }

    pub fn get_project(&self, project_id: &str) -> Result<Option<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, created_at, updated_at, status FROM projects WHERE id = ?",
        )?;
        
        let mut rows = stmt.query([project_id])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get::<_, Option<String>>(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                status: row.get(5)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list_projects(&self) -> Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, created_at, updated_at, status FROM projects",
        )?;
        
        let mut projects = Vec::new();
        let mut rows = stmt.query([])?;
        
        while let Some(row) = rows.next()? {
            projects.push(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get::<_, Option<String>>(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                status: row.get(5)?,
            });
        }
        
        Ok(projects)
    }
}
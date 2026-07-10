// Persistence service for SDK-IA project
// Manages Projects, Narratives, StoryElements, and GameEvents in SQLite

use crate::domain::{Project, Narrative, StoryElement, GameEvent, ProjectStatus, NarrativeStatus, StoryElementType, EventType};
use rusqlite::{Connection, Result, params, Row};
use std::path::Path;
use std::sync::Mutex;
use uuid::Uuid;
use chrono::{DateTime, Utc, FixedOffset};
use serde_json;

pub struct PersistenceService {
    conn: Mutex<Connection>,
}

impl PersistenceService {
    pub fn new(database_path: &str) -> Result<Self> {
        let path = Path::new(database_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                let code = e.raw_os_error().unwrap_or(0);
                rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error::new(code),
                    Some(e.to_string())
                )
            })?;
        }
        let conn = Connection::open(database_path)?;
        Self::initialize_database(&conn)?;
        Ok(Self { conn: Mutex::new(conn) })
    }

    fn initialize_database(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                author TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                version TEXT NOT NULL,
                status TEXT NOT NULL,
                tags TEXT,
                settings TEXT,
                metadata TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS narratives (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                title TEXT NOT NULL,
                synopsis TEXT,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                version TEXT NOT NULL,
                theme_ids TEXT,
                compatibility_score REAL DEFAULT 0.0,
                context_summary TEXT,
                metadata TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS story_elements (
                id TEXT PRIMARY KEY,
                narrative_id TEXT NOT NULL,
                element_type TEXT NOT NULL,
                hollywood_element_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                attributes TEXT,
                created_at TEXT NOT NULL,
                compatibility_score REAL DEFAULT 0.0,
                FOREIGN KEY (narrative_id) REFERENCES narratives(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS game_events (
                id TEXT PRIMARY KEY,
                narrative_id TEXT NOT NULL,
                event_type TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT,
                text TEXT,
                character_ids TEXT,
                location_ids TEXT,
                images TEXT,
                hollywood_event_id TEXT,
                timestamp TEXT,
                order_index INTEGER DEFAULT 0,
                attributes TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (narrative_id) REFERENCES narratives(id)
            )",
            [],
        )?;

        conn.execute("CREATE INDEX IF NOT EXISTS idx_narratives_project_id ON narratives(project_id)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_story_elements_narrative_id ON story_elements(narrative_id)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_game_events_narrative_id ON game_events(narrative_id)", [])?;

        Ok(())
    }

    fn parse_datetime_utc(s: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(s)
            .map(|dt: DateTime<FixedOffset>| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now())
    }

    fn parse_optional_datetime_utc(s: Option<String>) -> Option<DateTime<Utc>> {
        s.and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt: DateTime<FixedOffset>| dt.with_timezone(&Utc))
    }

    pub fn create_project(&self, project: &Project) -> Result<Uuid> {
        let tags_json = serde_json::to_string(&project.tags).unwrap_or("[]".to_string());
        let settings_json = serde_json::to_string(&project.settings).unwrap_or("{{}}".to_string());
        let metadata_json = serde_json::to_string(&project.metadata).unwrap_or("{{}}".to_string());

        let mut conn = self.conn.lock().unwrap();


        conn.execute(
            "INSERT INTO projects (id, name, description, author, created_at, updated_at, version, status, tags, settings, metadata) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                project.id.to_string(),
                &project.name,
                &project.description,
                &project.author,
                project.created_at.to_rfc3339(),
                project.updated_at.to_rfc3339(),
                &project.version,
                format!("{:?}", project.status),
                tags_json,
                settings_json,
                metadata_json,
            ],
        )?;
        Ok(project.id)
    }

    pub fn get_project(&self, id: &Uuid) -> Result<Option<Project>> {
        let mut conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, name, description, author, created_at, updated_at, version, status, tags, settings, metadata 
             FROM projects WHERE id = ?1"
        )?;

        let mut rows = stmt.query_map(params![id.to_string()], |row| {
            Ok(Project {
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap_or(*id),
                name: row.get(1)?,
                description: row.get(2)?,
                author: row.get(3)?,
                created_at: Self::parse_datetime_utc(&row.get::<_, String>(4)?),
                updated_at: Self::parse_datetime_utc(&row.get::<_, String>(5)?),
                version: row.get(6)?,
                status: serde_json::from_str(&format!("{:?}", row.get::<_, String>(7)?)).unwrap_or(ProjectStatus::Draft),
                tags: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                settings: serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or_default(),
                metadata: serde_json::from_str(&row.get::<_, String>(10)?).unwrap_or_default(),
            })
        })?;

        if let Some(row) = rows.next().transpose()? {
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }

    pub fn list_projects(&self, page: u32, page_size: u32) -> Result<Vec<Project>> {
        let offset = (page - 1) * page_size;
        let mut conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, name, description, author, created_at, updated_at, version, status, tags, settings, metadata 
             FROM projects ORDER BY created_at DESC LIMIT ?1 OFFSET ?2"
        )?;

        let mut projects = Vec::new();
        let mut rows = stmt.query_map(params![page_size, offset], |row| {
            Ok(Project {
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap(),
                name: row.get(1)?,
                description: row.get(2)?,
                author: row.get(3)?,
                created_at: Self::parse_datetime_utc(&row.get::<_, String>(4)?),
                updated_at: Self::parse_datetime_utc(&row.get::<_, String>(5)?),
                version: row.get(6)?,
                status: serde_json::from_str(&format!("{:?}", row.get::<_, String>(7)?)).unwrap_or(ProjectStatus::Draft),
                tags: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                settings: serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or_default(),
                metadata: serde_json::from_str(&row.get::<_, String>(10)?).unwrap_or_default(),
            })
        })?;

        while let Some(row) = rows.next().transpose()? {
            projects.push(row);
        }

        Ok(projects)
    }

    pub fn get_all_projects(&self) -> Result<Vec<Project>> {
        let mut conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, author, created_at, updated_at, version, status, tags, settings, metadata 
             FROM projects ORDER BY created_at DESC"
        )?;

        let mut projects = Vec::new();
        let mut rows = stmt.query_map([], |row| {
            Ok(Project {
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap(),
                name: row.get(1)?,
                description: row.get(2)?,
                author: row.get(3)?,
                created_at: Self::parse_datetime_utc(&row.get::<_, String>(4)?),
                updated_at: Self::parse_datetime_utc(&row.get::<_, String>(5)?),
                version: row.get(6)?,
                status: serde_json::from_str(&format!("{:?}", row.get::<_, String>(7)?)).unwrap_or(ProjectStatus::Draft),
                tags: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                settings: serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or_default(),
                metadata: serde_json::from_str(&row.get::<_, String>(10)?).unwrap_or_default(),
            })
        })?;

        while let Some(row) = rows.next().transpose()? {
            projects.push(row);
        }

        Ok(projects)
    }

    pub fn update_project(&self, project: &Project) -> Result<()> {
        let tags_json = serde_json::to_string(&project.tags).unwrap_or("[]".to_string());
        let settings_json = serde_json::to_string(&project.settings).unwrap_or("{{}}".to_string());
        let metadata_json = serde_json::to_string(&project.metadata).unwrap_or("{{}}".to_string());

        let mut conn = self.conn.lock().unwrap();


        conn.execute(
            "UPDATE projects SET 
                name = ?1, description = ?2, author = ?3, updated_at = ?4, 
                version = ?5, status = ?6, tags = ?7, settings = ?8, metadata = ?9 
             WHERE id = ?10",
            params![
                &project.name,
                &project.description,
                &project.author,
                project.updated_at.to_rfc3339(),
                &project.version,
                format!("{:?}", project.status),
                tags_json,
                settings_json,
                metadata_json,
                project.id.to_string(),
            ],
        )?;
        Ok(())
    }

    pub fn delete_project(&self, id: &Uuid) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();

        let mut tx = conn.transaction()?;

        tx.execute("DELETE FROM game_events WHERE narrative_id IN (SELECT id FROM narratives WHERE project_id = ?1)", params![id.to_string()])?;
        tx.execute("DELETE FROM story_elements WHERE narrative_id IN (SELECT id FROM narratives WHERE project_id = ?1)", params![id.to_string()])?;
        tx.execute("DELETE FROM narratives WHERE project_id = ?1", params![id.to_string()])?;
        tx.execute("DELETE FROM projects WHERE id = ?1", params![id.to_string()])?;

        tx.commit()?;
        Ok(())
    }

    pub fn count_projects(&self) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM projects",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    pub fn create_narrative(&self, narrative: &Narrative) -> Result<Uuid> {
        let theme_ids_json = serde_json::to_string(&narrative.theme_ids).unwrap_or("[]".to_string());
        let metadata_json = serde_json::to_string(&narrative.metadata).unwrap_or("{{}}".to_string());

        let mut conn = self.conn.lock().unwrap();


        conn.execute(
            "INSERT INTO narratives (id, project_id, title, synopsis, status, created_at, updated_at, version, theme_ids, compatibility_score, context_summary, metadata) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                narrative.id.to_string(),
                narrative.project_id.to_string(),
                &narrative.title,
                &narrative.synopsis,
                format!("{:?}", narrative.status),
                narrative.created_at.to_rfc3339(),
                narrative.updated_at.to_rfc3339(),
                &narrative.version,
                theme_ids_json,
                narrative.compatibility_score,
                &narrative.context_summary,
                metadata_json,
            ],
        )?;
        Ok(narrative.id)
    }

    pub fn get_narrative(&self, id: &Uuid) -> Result<Option<Narrative>> {
        let mut conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, project_id, title, synopsis, status, created_at, updated_at, version, theme_ids, compatibility_score, context_summary, metadata 
             FROM narratives WHERE id = ?1"
        )?;

        let mut rows = stmt.query_map(params![id.to_string()], |row| {
            Ok(Narrative {
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap_or(*id),
                project_id: Uuid::parse_str(row.get::<_, String>(1)?.as_str()).unwrap(),
                title: row.get(2)?,
                synopsis: row.get(3)?,
                status: serde_json::from_str(&format!("{:?}", row.get::<_, String>(4)?)).unwrap_or(NarrativeStatus::Outline),
                created_at: Self::parse_datetime_utc(&row.get::<_, String>(5)?),
                updated_at: Self::parse_datetime_utc(&row.get::<_, String>(6)?),
                version: row.get(7)?,
                theme_ids: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                compatibility_score: row.get(9)?,
                context_summary: row.get(10)?,
                metadata: serde_json::from_str(&row.get::<_, String>(11)?).unwrap_or_default(),
            })
        })?;

        if let Some(row) = rows.next().transpose()? {
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }

    pub fn list_narratives_by_project(&self, project_id: &Uuid) -> Result<Vec<Narrative>> {
        let mut conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, project_id, title, synopsis, status, created_at, updated_at, version, theme_ids, compatibility_score, context_summary, metadata 
             FROM narratives WHERE project_id = ?1 ORDER BY created_at DESC"
        )?;

        let mut narratives = Vec::new();
        let mut rows = stmt.query_map(params![project_id.to_string()], |row| {
            Ok(Narrative {
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap(),
                project_id: Uuid::parse_str(row.get::<_, String>(1)?.as_str()).unwrap(),
                title: row.get(2)?,
                synopsis: row.get(3)?,
                status: serde_json::from_str(&format!("{:?}", row.get::<_, String>(4)?)).unwrap_or(NarrativeStatus::Outline),
                created_at: Self::parse_datetime_utc(&row.get::<_, String>(5)?),
                updated_at: Self::parse_datetime_utc(&row.get::<_, String>(6)?),
                version: row.get(7)?,
                theme_ids: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                compatibility_score: row.get(9)?,
                context_summary: row.get(10)?,
                metadata: serde_json::from_str(&row.get::<_, String>(11)?).unwrap_or_default(),
            })
        })?;

        while let Some(row) = rows.next().transpose()? {
            narratives.push(row);
        }

        Ok(narratives)
    }

    pub fn update_narrative(&self, narrative: &Narrative) -> Result<()> {
        let theme_ids_json = serde_json::to_string(&narrative.theme_ids).unwrap_or("[]".to_string());
        let metadata_json = serde_json::to_string(&narrative.metadata).unwrap_or("{{}}".to_string());

        let mut conn = self.conn.lock().unwrap();


        conn.execute(
            "UPDATE narratives SET 
                title = ?1, synopsis = ?2, status = ?3, updated_at = ?4, 
                version = ?5, theme_ids = ?6, compatibility_score = ?7, context_summary = ?8, metadata = ?9 
             WHERE id = ?10",
            params![
                &narrative.title,
                &narrative.synopsis,
                format!("{:?}", narrative.status),
                narrative.updated_at.to_rfc3339(),
                &narrative.version,
                theme_ids_json,
                narrative.compatibility_score,
                &narrative.context_summary,
                metadata_json,
                narrative.id.to_string(),
            ],
        )?;
        Ok(())
    }

    pub fn delete_narrative(&self, id: &Uuid) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();

        let mut tx = conn.transaction()?;

        tx.execute("DELETE FROM game_events WHERE narrative_id = ?1", params![id.to_string()])?;
        tx.execute("DELETE FROM story_elements WHERE narrative_id = ?1", params![id.to_string()])?;
        tx.execute("DELETE FROM narratives WHERE id = ?1", params![id.to_string()])?;

        tx.commit()?;
        Ok(())
    }

    pub fn create_story_element(&self, element: &StoryElement) -> Result<Uuid> {
        let attributes_json = serde_json::to_string(&element.attributes).unwrap_or("{{}}".to_string());

        let mut conn = self.conn.lock().unwrap();


        conn.execute(
            "INSERT INTO story_elements (id, narrative_id, element_type, hollywood_element_id, name, description, attributes, created_at, compatibility_score) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                element.id.to_string(),
                element.narrative_id.to_string(),
                format!("{:?}", element.element_type),
                &element.hollywood_element_id,
                &element.name,
                &element.description,
                attributes_json,
                element.created_at.to_rfc3339(),
                element.compatibility_score,
            ],
        )?;
        Ok(element.id)
    }

    pub fn get_story_element(&self, id: &Uuid) -> Result<Option<StoryElement>> {
        let mut conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, narrative_id, element_type, hollywood_element_id, name, description, attributes, created_at, compatibility_score 
             FROM story_elements WHERE id = ?1"
        )?;

        let mut rows = stmt.query_map(params![id.to_string()], |row| {
            Ok(StoryElement {
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap_or(*id),
                narrative_id: Uuid::parse_str(row.get::<_, String>(1)?.as_str()).unwrap(),
                element_type: serde_json::from_str(&format!("{:?}", row.get::<_, String>(2)?)).unwrap_or(StoryElementType::Protagonist),
                hollywood_element_id: row.get(3)?,
                name: row.get(4)?,
                description: row.get(5)?,
                attributes: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
                created_at: Self::parse_datetime_utc(&row.get::<_, String>(7)?),
                compatibility_score: row.get(8)?,
            })
        })?;

        if let Some(row) = rows.next().transpose()? {
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }

    pub fn list_story_elements_by_narrative(&self, narrative_id: &Uuid) -> Result<Vec<StoryElement>> {
        let mut conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, narrative_id, element_type, hollywood_element_id, name, description, attributes, created_at, compatibility_score 
             FROM story_elements WHERE narrative_id = ?1 ORDER BY created_at DESC"
        )?;

        let mut elements = Vec::new();
        let mut rows = stmt.query_map(params![narrative_id.to_string()], |row| {
            Ok(StoryElement {
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap(),
                narrative_id: Uuid::parse_str(row.get::<_, String>(1)?.as_str()).unwrap(),
                element_type: serde_json::from_str(&format!("{:?}", row.get::<_, String>(2)?)).unwrap_or(StoryElementType::Protagonist),
                hollywood_element_id: row.get(3)?,
                name: row.get(4)?,
                description: row.get(5)?,
                attributes: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
                created_at: Self::parse_datetime_utc(&row.get::<_, String>(7)?),
                compatibility_score: row.get(8)?,
            })
        })?;

        while let Some(row) = rows.next().transpose()? {
            elements.push(row);
        }

        Ok(elements)
    }

    pub fn list_story_elements_by_type(&self, narrative_id: &Uuid, element_type: StoryElementType) -> Result<Vec<StoryElement>> {
        let type_str = format!("{:?}", element_type);
        let element_type_for_closure = element_type.clone();
        let mut conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, narrative_id, element_type, hollywood_element_id, name, description, attributes, created_at, compatibility_score 
             FROM story_elements WHERE narrative_id = ?1 AND element_type = ?2 ORDER BY created_at DESC"
        )?;

        let mut elements = Vec::new();
        let element_type_clone = element_type_for_closure.clone();
        let mut rows = stmt.query_map(params![narrative_id.to_string(), type_str], |row| {
            Ok(StoryElement {
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap(),
                narrative_id: Uuid::parse_str(row.get::<_, String>(1)?.as_str()).unwrap(),
                element_type: serde_json::from_str(&format!("{:?}", row.get::<_, String>(2)?)).unwrap_or(element_type_clone.clone()),
                hollywood_element_id: row.get(3)?,
                name: row.get(4)?,
                description: row.get(5)?,
                attributes: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
                created_at: Self::parse_datetime_utc(&row.get::<_, String>(7)?),
                compatibility_score: row.get(8)?,
            })
        })?;

        while let Some(row) = rows.next().transpose()? {
            elements.push(row);
        }

        Ok(elements)
    }

    pub fn update_story_element(&self, element: &StoryElement) -> Result<()> {
        let attributes_json = serde_json::to_string(&element.attributes).unwrap_or("{{}}".to_string());

        let mut conn = self.conn.lock().unwrap();


        conn.execute(
            "UPDATE story_elements SET 
                narrative_id = ?1, element_type = ?2, hollywood_element_id = ?3, 
                name = ?4, description = ?5, attributes = ?6, compatibility_score = ?7 
             WHERE id = ?8",
            params![
                element.narrative_id.to_string(),
                format!("{:?}", element.element_type),
                &element.hollywood_element_id,
                &element.name,
                &element.description,
                attributes_json,
                element.compatibility_score,
                element.id.to_string(),
            ],
        )?;
        Ok(())
    }

    pub fn delete_story_element(&self, id: &Uuid) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();

        conn.execute(
            "DELETE FROM story_elements WHERE id = ?1",
            params![id.to_string()],
        )?;
        Ok(())
    }

    pub fn create_game_event(&self, event: &GameEvent) -> Result<Uuid> {
        let character_ids_json = serde_json::to_string(&event.character_ids).unwrap_or("[]".to_string());
        let location_ids_json = serde_json::to_string(&event.location_ids).unwrap_or("[]".to_string());
        let images_json = serde_json::to_string(&event.images).unwrap_or("[]".to_string());
        let attributes_json = serde_json::to_string(&event.attributes).unwrap_or("{{}}".to_string());

        let mut conn = self.conn.lock().unwrap();


        conn.execute(
            "INSERT INTO game_events (id, narrative_id, event_type, title, description, text, character_ids, location_ids, images, hollywood_event_id, timestamp, order_index, attributes, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                event.id.to_string(),
                event.narrative_id.to_string(),
                format!("{:?}", event.event_type),
                &event.title,
                &event.description,
                &event.text,
                character_ids_json,
                location_ids_json,
                images_json,
                event.hollywood_event_id.as_deref().unwrap_or(""),
                event.timestamp.map(|t| t.to_rfc3339()).unwrap_or_else(|| "".to_string()),
                event.order_index,
                attributes_json,
                event.created_at.to_rfc3339(),
                event.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(event.id)
    }

    pub fn get_game_event(&self, id: &Uuid) -> Result<Option<GameEvent>> {
        let mut conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, narrative_id, event_type, title, description, text, character_ids, location_ids, images, hollywood_event_id, timestamp, order_index, attributes, created_at, updated_at 
             FROM game_events WHERE id = ?1"
        )?;

        let mut rows = stmt.query_map(params![id.to_string()], |row| {
            Ok(GameEvent {
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap_or(*id),
                narrative_id: Uuid::parse_str(row.get::<_, String>(1)?.as_str()).unwrap(),
                event_type: serde_json::from_str(&format!("{:?}", row.get::<_, String>(2)?)).unwrap_or(EventType::Scene),
                title: row.get(3)?,
                description: row.get(4)?,
                text: row.get(5)?,
                character_ids: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
                location_ids: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
                images: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                hollywood_event_id: row.get::<_, Option<String>>(9)?,
                timestamp: Self::parse_optional_datetime_utc(row.get::<_, Option<String>>(10)?),
                order_index: row.get(11)?,
                attributes: serde_json::from_str(&row.get::<_, String>(12)?).unwrap_or_default(),
                created_at: Self::parse_datetime_utc(&row.get::<_, String>(13)?),
                updated_at: Self::parse_datetime_utc(&row.get::<_, String>(14)?),
            })
        })?;

        if let Some(row) = rows.next().transpose()? {
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }

    pub fn list_game_events_by_narrative(&self, narrative_id: &Uuid) -> Result<Vec<GameEvent>> {
        let mut conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, narrative_id, event_type, title, description, text, character_ids, location_ids, images, hollywood_event_id, timestamp, order_index, attributes, created_at, updated_at 
             FROM game_events WHERE narrative_id = ?1 ORDER BY order_index ASC, created_at DESC"
        )?;

        let mut events = Vec::new();
        let mut rows = stmt.query_map(params![narrative_id.to_string()], |row| {
            Ok(GameEvent {
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap(),
                narrative_id: Uuid::parse_str(row.get::<_, String>(1)?.as_str()).unwrap(),
                event_type: serde_json::from_str(&format!("{:?}", row.get::<_, String>(2)?)).unwrap_or(EventType::Scene),
                title: row.get(3)?,
                description: row.get(4)?,
                text: row.get(5)?,
                character_ids: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
                location_ids: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
                images: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                hollywood_event_id: row.get::<_, Option<String>>(9)?,
                timestamp: Self::parse_optional_datetime_utc(row.get::<_, Option<String>>(10)?),
                order_index: row.get(11)?,
                attributes: serde_json::from_str(&row.get::<_, String>(12)?).unwrap_or_default(),
                created_at: Self::parse_datetime_utc(&row.get::<_, String>(13)?),
                updated_at: Self::parse_datetime_utc(&row.get::<_, String>(14)?),
            })
        })?;

        while let Some(row) = rows.next().transpose()? {
            events.push(row);
        }

        Ok(events)
    }

    pub fn update_game_event(&self, event: &GameEvent) -> Result<()> {
        let character_ids_json = serde_json::to_string(&event.character_ids).unwrap_or("[]".to_string());
        let location_ids_json = serde_json::to_string(&event.location_ids).unwrap_or("[]".to_string());
        let images_json = serde_json::to_string(&event.images).unwrap_or("[]".to_string());
        let attributes_json = serde_json::to_string(&event.attributes).unwrap_or("{{}}".to_string());

        let mut conn = self.conn.lock().unwrap();


        conn.execute(
            "UPDATE game_events SET 
                narrative_id = ?1, event_type = ?2, title = ?3, description = ?4, text = ?5, 
                character_ids = ?6, location_ids = ?7, images = ?8, hollywood_event_id = ?9, 
                timestamp = ?10, order_index = ?11, attributes = ?12, updated_at = ?13 
             WHERE id = ?14",
            params![
                event.narrative_id.to_string(),
                format!("{:?}", event.event_type),
                &event.title,
                &event.description,
                &event.text,
                character_ids_json,
                location_ids_json,
                images_json,
                event.hollywood_event_id.as_deref().unwrap_or(""),
                event.timestamp.map(|t| t.to_rfc3339()).unwrap_or_else(|| "".to_string()),
                event.order_index,
                attributes_json,
                event.updated_at.to_rfc3339(),
                event.id.to_string(),
            ],
        )?;
        Ok(())
    }

    pub fn delete_game_event(&self, id: &Uuid) -> Result<()> {
        let mut conn = self.conn.lock().unwrap();

        conn.execute(
            "DELETE FROM game_events WHERE id = ?1",
            params![id.to_string()],
        )?;
        Ok(())
    }

    pub fn search_projects(
        &self,
        name: Option<&str>,
        author: Option<&str>,
        status: Option<ProjectStatus>,
        tags: Option<&[String]>,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<Project>> {
        let mut query = "SELECT id, name, description, author, created_at, updated_at, version, status, tags, settings, metadata FROM projects".to_string();
        let mut conditions: Vec<String> = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(name) = name {
            conditions.push("name LIKE ?1".to_string());
            params.push(Box::new(format!("%{}%", name)));
        }
        if let Some(author) = author {
            conditions.push("author LIKE ?2".to_string());
            params.push(Box::new(format!("%{}%", author)));
        }
        if let Some(status) = status {
            conditions.push("status = ?3".to_string());
            params.push(Box::new(format!("{:?}", status)));
        }
        if let Some(tags) = tags {
            for (i, tag) in tags.iter().enumerate() {
                conditions.push(format!("tags LIKE ?{}", i + 4));
                params.push(Box::new(format!("%{}%", tag)));
            }
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(" ORDER BY created_at DESC LIMIT ?99 OFFSET ?100");
        params.push(Box::new(page_size));
        params.push(Box::new((page - 1) * page_size));

        let mut conn = self.conn.lock().unwrap();


        let mut stmt = conn.prepare(&query)?;
        let mut projects = Vec::new();
        let mut rows = stmt.query_map(params.iter().map(|p| p.as_ref()).collect::<Vec<_>>().as_slice(), |row| {
            Ok(Project {
                id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap(),
                name: row.get(1)?,
                description: row.get(2)?,
                author: row.get(3)?,
                created_at: Self::parse_datetime_utc(&row.get::<_, String>(4)?),
                updated_at: Self::parse_datetime_utc(&row.get::<_, String>(5)?),
                version: row.get(6)?,
                status: serde_json::from_str(&format!("{:?}", row.get::<_, String>(7)?)).unwrap_or(ProjectStatus::Draft),
                tags: serde_json::from_str(&row.get::<_, String>(8)?).unwrap_or_default(),
                settings: serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or_default(),
                metadata: serde_json::from_str(&row.get::<_, String>(10)?).unwrap_or_default(),
            })
        })?;

        while let Some(row) = rows.next().transpose()? {
            projects.push(row);
        }

        Ok(projects)
    }

    pub fn count_projects_search(
        &self,
        name: Option<&str>,
        author: Option<&str>,
        status: Option<ProjectStatus>,
        tags: Option<&[String]>,
    ) -> Result<i64> {
        let mut query = "SELECT COUNT(*) FROM projects".to_string();
        let mut conditions: Vec<String> = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(name) = name {
            conditions.push("name LIKE ?1".to_string());
            params.push(Box::new(format!("%{}%", name)));
        }
        if let Some(author) = author {
            conditions.push("author LIKE ?2".to_string());
            params.push(Box::new(format!("%{}%", author)));
        }
        if let Some(status) = status {
            conditions.push("status = ?3".to_string());
            params.push(Box::new(format!("{:?}", status)));
        }
        if let Some(tags) = tags {
            for (i, tag) in tags.iter().enumerate() {
                conditions.push(format!("tags LIKE ?{}", i + 4));
                params.push(Box::new(format!("%{}%", tag)));
            }
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        let mut conn = self.conn.lock().unwrap();


        let mut stmt = conn.prepare(&query)?;
        let count: i64 = stmt.query_row(params.iter().map(|p| p.as_ref()).collect::<Vec<_>>().as_slice(), |row| row.get(0))?;
        Ok(count)
    }
}
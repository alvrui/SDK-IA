/// Database configuration for the application
pub const DB_PATH: &str = "data/sdk_ia.db";

/// Initialize database directory
pub fn init_db_directory() -> std::io::Result<()> {
    use std::path::Path;
    let path = Path::new(DB_PATH);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}
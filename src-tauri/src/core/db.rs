use chrono::Utc;
use duckdb::{params, Connection, Result, RowIndex};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

pub struct DatabaseManager {
    db: Arc<Mutex<Connection>>,
    current_chunk_id: i64,
    last_frame_id: i64,
    current_frame_offset: i64,
    recent_frames_threshold: usize,
}

/// I don't know if duckdb is safe for async...
impl DatabaseManager {
    // Initialize the DatabaseManager
    pub fn new() -> Result<Self> {
        let db = Connection::open("db.duckdb")?; // Using in-memory for this example, adjust as needed
        let db = Arc::new(Mutex::new(db));

        let mut manager = DatabaseManager {
            db,
            current_chunk_id: 0,
            last_frame_id: 0,
            current_frame_offset: 0,
            recent_frames_threshold: 15,
        };

        manager.create_tables()?;
        manager.current_chunk_id = manager.get_current_chunk_id()?;
        manager.last_frame_id = manager.get_last_frame_id()?;

        Ok(manager)
    }

    // Function to create tables
    fn create_tables(&mut self) -> Result<()> {
        let db = self.db.lock().unwrap();

        db.execute("CREATE TABLE IF NOT EXISTS video_chunks (id INTEGER PRIMARY KEY AUTOINCREMENT, filePath TEXT);", params![])?;
        db.execute("CREATE TABLE IF NOT EXISTS frames (id INTEGER PRIMARY KEY AUTOINCREMENT, chunkId INTEGER, offsetIndex INTEGER, timestamp TIMESTAMP, activeApplicationName TEXT, FOREIGN KEY(chunkId) REFERENCES video_chunks(id));", params![])?;

        Ok(())
    }

    // Function to get the current chunk ID
    fn get_current_chunk_id(&self) -> Result<i64> {
        let db = self.db.lock().unwrap();

        db.query_row(
            "SELECT MAX(id) FROM video_chunks",
            [],
            |row| Ok(row.get(0)?),
        )
    }

    // Function to get the last frame ID
    fn get_last_frame_id(&self) -> Result<i64> {
        let db = self.db.lock().unwrap();

        match db.query_row("SELECT MAX(id) FROM frames", [], |row| row.get(0)) {
            Ok(Some(id)) => Ok(id),
            Ok(None) | Err(_) => Ok(0),
        }
    }

    // Method to purge the database (drop and recreate tables)
    pub fn purge(&mut self) -> Result<()> {
        {
            let db = self.db.lock().unwrap();

            db.execute("DROP TABLE IF EXISTS video_chunks;", params![])?;
            db.execute("DROP TABLE IF EXISTS frames;", params![])?;
            db.execute("DROP TABLE IF EXISTS allText;", params![])?;
        }

        self.create_tables()?;
        self.current_chunk_id = self.get_current_chunk_id()?;
        self.last_frame_id = self.get_last_frame_id()?;

        Ok(())
    }

    // Method to insert a new video chunk and return its ID
    pub fn start_new_video_chunk(&mut self, file_path: &str) -> Result<i64> {
        let db = self.db.lock().unwrap();

        let id = db.query_row(
            "INSERT INTO video_chunks (filePath) VALUES (?) RETURNING (id);",
            [file_path],
            |row| row.get(0),
        )?;

        self.current_chunk_id = id;
        self.current_frame_offset = 0;

        Ok(id)
    }

    // Method to insert a frame
    pub fn insert_frame(&mut self, active_application_name: Option<&str>) -> Result<i64> {
        let db = self.db.lock().unwrap();

        let time = Utc::now();
        let id = db.query_row(
            "INSERT INTO frames (chunkId, offsetIndex, timestamp, activeApplicationName) VALUES (?, ?, ?, ?) RETURNING (id);",
            params![self.current_chunk_id, self.current_frame_offset, time, active_application_name.unwrap_or("")],
            |row| row.get(0),
        )?;
        self.current_frame_offset += 1;
        self.last_frame_id = id;

        Ok(id)
    }

    // Method to insert text for a frame
    pub fn insert_text_for_frame(&self, frame_id: i64, text: &str) -> Result<()> {
        let db = self.db.lock().unwrap();

        db.execute(
            "INSERT INTO allText (frameId, text) VALUES (?, ?);",
            params![frame_id, text],
        )?;

        Ok(())
    }

    // Method to get a frame by index
    pub fn get_frame(&self, index: i64) -> Result<(i64, String)> {
        let db = self.db.lock().unwrap();

        db.query_row(
            "SELECT f.offsetIndex, vc.filePath FROM frames f JOIN video_chunks vc ON f.chunkId = vc.id WHERE f.id = ? LIMIT 1;",
            [index],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
    }
}

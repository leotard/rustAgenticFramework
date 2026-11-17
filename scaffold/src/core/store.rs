//! Factor 6 — launch, pause, resume. A thread is trivially serializable, so
//! pausing is just `save`; resuming is just `load` and continue.

use std::path::PathBuf;

use super::error::AgentError;
use super::thread::Thread;

/// Persistence for threads. Swap `FileStore` for Postgres/SQLite/etc. without
/// touching the runtime loop.
pub trait Store {
    fn save(&self, id: &str, thread: &Thread) -> Result<(), AgentError>;
    fn load(&self, id: &str) -> Result<Thread, AgentError>;
}

/// Reference implementation backed by JSON files.
#[derive(Clone)]
pub struct FileStore {
    pub dir: PathBuf,
}

impl FileStore {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    fn path(&self, id: &str) -> PathBuf {
        self.dir.join(format!("{id}.json"))
    }
}

impl Store for FileStore {
    fn save(&self, id: &str, thread: &Thread) -> Result<(), AgentError> {
        std::fs::create_dir_all(&self.dir).map_err(|e| AgentError::Store(e.to_string()))?;
        let json = thread
            .to_json()
            .map_err(|e| AgentError::Store(e.to_string()))?;
        std::fs::write(self.path(id), json).map_err(|e| AgentError::Store(e.to_string()))
    }

    fn load(&self, id: &str) -> Result<Thread, AgentError> {
        let json =
            std::fs::read_to_string(self.path(id)).map_err(|e| AgentError::Store(e.to_string()))?;
        serde_json::from_str(&json).map_err(|e| AgentError::Store(e.to_string()))
    }
}

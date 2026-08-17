//! The Week 12 capstone core library (`ClipNote`) — implemented as a module here
//! so every example is testable with plain `cargo test`. In the real capstone
//! these live in a separate `clipnote-core` crate with **no** Tauri imports.

#![warn(clippy::pedantic)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub created_at: std::time::SystemTime,
}

#[derive(Debug, thiserror::Error)]
pub enum ClipError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid note: {0}")]
    InvalidNote(String),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("note {0} not found")]
    NotFound(u64),
}

impl Note {
    /// Validation returns `Result`, not panics — impossible states are
    /// unrepresentable.
    ///
    /// # Errors
    ///
    /// Returns [`ClipError::InvalidNote`] when the title or body violates the
    /// length or non-empty requirements.
    pub fn new(title: &str, body: &str) -> Result<Note, ClipError> {
        if title.trim().is_empty() {
            return Err(ClipError::InvalidNote("title cannot be empty".into()));
        }
        if title.chars().count() > 120 {
            return Err(ClipError::InvalidNote(
                "title too long (max 120 chars)".into(),
            ));
        }
        if body.trim().is_empty() {
            return Err(ClipError::InvalidNote("body cannot be empty".into()));
        }
        if body.chars().count() > 10_000 {
            return Err(ClipError::InvalidNote(
                "body too long (max 10000 chars)".into(),
            ));
        }
        Ok(Note {
            id: 0, // assigned by the store
            title: title.to_string(),
            body: body.to_string(),
            created_at: std::time::SystemTime::now(),
        })
    }
}

/// File-backed storage: one JSON file per note.
pub struct Store {
    dir: PathBuf,
    next_id: u64,
}

impl Store {
    /// Creates the storage directory if it does not already exist.
    ///
    /// # Errors
    ///
    /// Returns [`ClipError::Io`] when the directory cannot be created.
    pub fn new(dir: PathBuf) -> Result<Store, ClipError> {
        std::fs::create_dir_all(&dir)?;
        Ok(Store { dir, next_id: 1 })
    }

    fn path_for(&self, id: u64) -> PathBuf {
        self.dir.join(format!("{id}.json"))
    }

    /// Save a note, assign it an id, write `{id}.json`, return the id.
    ///
    /// # Errors
    ///
    /// Returns [`ClipError::Json`] if serialization fails or [`ClipError::Io`]
    /// if the note cannot be written.
    pub fn save(&mut self, note: &Note) -> Result<u64, ClipError> {
        let id = self.next_id;
        self.next_id += 1;
        let mut note = note.clone();
        note.id = id;
        let json = serde_json::to_string_pretty(&note)?;
        std::fs::write(self.path_for(id), json)?;
        Ok(id)
    }

    /// Loads a note by id, returning `None` when no file exists.
    ///
    /// # Errors
    ///
    /// Returns [`ClipError::Json`] if the file is malformed or [`ClipError::Io`]
    /// if it cannot be read.
    pub fn get(&self, id: u64) -> Result<Option<Note>, ClipError> {
        let path = self.path_for(id);
        if !path.exists() {
            return Ok(None);
        }
        let json = std::fs::read_to_string(path)?;
        Ok(Some(serde_json::from_str(&json)?))
    }

    /// Loads and sorts every note by creation time.
    ///
    /// # Errors
    ///
    /// Returns [`ClipError::Io`] if a directory entry or file cannot be read,
    /// or [`ClipError::Json`] if a file is malformed.
    pub fn load_all(&self) -> Result<Vec<Note>, ClipError> {
        let mut notes: Vec<Note> = Vec::new();
        for entry in std::fs::read_dir(&self.dir)? {
            let path = entry?.path();
            let json = std::fs::read_to_string(path)?;
            notes.push(serde_json::from_str(&json)?);
        }
        notes.sort_by_key(|n| n.created_at);
        Ok(notes)
    }

    /// Deletes a note by id.
    ///
    /// # Errors
    ///
    /// Returns [`ClipError::NotFound`] when the note does not exist, or
    /// [`ClipError::Io`] when the file cannot be deleted.
    pub fn delete(&self, id: u64) -> Result<(), ClipError> {
        let path = self.path_for(id);
        if path.exists() {
            std::fs::remove_file(path)?;
            Ok(())
        } else {
            Err(ClipError::NotFound(id))
        }
    }
}

/// Case-insensitive substring match on title + body, via iterators.
#[must_use]
pub fn search<'a>(notes: &'a [Note], query: &str) -> Vec<&'a Note> {
    let q = query.to_lowercase();
    notes
        .iter()
        .filter(|n| n.title.to_lowercase().contains(&q) || n.body.to_lowercase().contains(&q))
        .collect()
}

/// Most common words across all bodies, sorted by frequency (desc), then word.
#[must_use]
pub fn word_frequency(notes: &[Note]) -> Vec<(String, u32)> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    for note in notes {
        for word in note.body.split_whitespace() {
            let w = word
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();
            if !w.is_empty() {
                *counts.entry(w).or_insert(0) += 1;
            }
        }
    }
    let mut v: Vec<(String, u32)> = counts.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    v
}

/// The command layer (what the `#[tauri::command]` wrappers call). Kept as
/// plain functions here so the logic is testable without the Tauri runtime.
pub struct AppState {
    pub store: std::sync::Mutex<Store>,
}

/// Creates and persists a validated note.
///
/// # Errors
///
/// Returns a user-facing error when validation, locking, or persistence fails.
pub fn create_note(state: &AppState, title: &str, body: &str) -> Result<u64, String> {
    let note = Note::new(title, body).map_err(|e| e.to_string())?;
    let mut store = state.store.lock().map_err(|e| e.to_string())?;
    store.save(&note).map_err(|e| e.to_string())
}

/// Lists all persisted notes.
///
/// # Errors
///
/// Returns a user-facing error when locking or loading fails.
pub fn list_notes(state: &AppState) -> Result<Vec<Note>, String> {
    let store = state.store.lock().map_err(|e| e.to_string())?;
    store.load_all().map_err(|e| e.to_string())
}

/// Gets one note by id.
///
/// # Errors
///
/// Returns a user-facing error when locking, loading, or lookup fails.
pub fn get_note(state: &AppState, id: u64) -> Result<Note, String> {
    let store = state.store.lock().map_err(|e| e.to_string())?;
    store
        .get(id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("note {id} not found"))
}

/// Deletes one note by id.
///
/// # Errors
///
/// Returns a user-facing error when locking or deletion fails.
pub fn delete_note(state: &AppState, id: u64) -> Result<(), String> {
    let store = state.store.lock().map_err(|e| e.to_string())?;
    store.delete(id).map_err(|e| e.to_string())
}

/// Calculates word-frequency statistics over all notes.
///
/// # Errors
///
/// Returns a user-facing error when locking or loading fails.
pub fn note_stats(state: &AppState) -> Result<Vec<(String, u32)>, String> {
    let store = state.store.lock().map_err(|e| e.to_string())?;
    let notes = store.load_all().map_err(|e| e.to_string())?;
    Ok(word_frequency(&notes))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns the Store AND its tempdir — the tempdir must stay alive for
    /// the whole test, or the files are deleted before the store uses them.
    fn test_store() -> (tempfile::TempDir, Store) {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::new(dir.path().to_path_buf()).unwrap();
        (dir, store)
    }

    #[test]
    fn note_validation_rejects_bad_input() {
        assert!(matches!(
            Note::new("", "body"),
            Err(ClipError::InvalidNote(_))
        ));
        assert!(matches!(
            Note::new(&"x".repeat(121), "body"),
            Err(ClipError::InvalidNote(_))
        ));
        assert!(matches!(
            Note::new("title", ""),
            Err(ClipError::InvalidNote(_))
        ));
        assert!(matches!(
            Note::new("title", &"x".repeat(10_001)),
            Err(ClipError::InvalidNote(_))
        ));
        assert!(Note::new("title", "body").is_ok());
    }

    #[test]
    fn store_roundtrip_save_get_delete() {
        let (_dir, mut s) = test_store();
        let id = s.save(&Note::new("hello", "world").unwrap()).unwrap();
        let got = s.get(id).unwrap().unwrap();
        assert_eq!(got.title, "hello");
        assert_eq!(got.body, "world");
        assert_eq!(got.id, id);

        s.delete(id).unwrap();
        assert_eq!(s.get(id).unwrap(), None);
    }

    #[test]
    fn store_delete_missing_errors() {
        let (_dir, s) = test_store();
        assert!(matches!(s.delete(999), Err(ClipError::NotFound(999))));
    }

    #[test]
    fn store_load_all() {
        let (_dir, mut s) = test_store();
        s.save(&Note::new("a", "one").unwrap()).unwrap();
        s.save(&Note::new("b", "two").unwrap()).unwrap();
        assert_eq!(s.load_all().unwrap().len(), 2);
    }

    #[test]
    fn search_matches_case_insensitive() {
        let notes = vec![
            Note::new("Rust Notes", "about ownership").unwrap(),
            Note::new("Shopping", "buy milk and eggs").unwrap(),
        ];
        assert_eq!(search(&notes, "RUST").len(), 1);
        assert_eq!(search(&notes, "milk").len(), 1);
        assert_eq!(search(&notes, "zzz").len(), 0);
    }

    #[test]
    fn word_frequency_sorted_descending() {
        let notes = vec![
            Note::new("a", "the cat the dog the").unwrap(),
            Note::new("b", "cat").unwrap(),
        ];
        let freq = word_frequency(&notes);
        assert_eq!(freq[0], ("the".to_string(), 3));
        assert_eq!(freq[1], ("cat".to_string(), 2));
    }

    #[test]
    fn commands_work_end_to_end() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::new(dir.path().to_path_buf()).unwrap();
        let state = AppState {
            store: std::sync::Mutex::new(store),
        };

        let id = create_note(&state, "t", "hello world").unwrap();
        assert_eq!(list_notes(&state).unwrap().len(), 1);
        assert_eq!(get_note(&state, id).unwrap().title, "t");
        assert!(get_note(&state, 999).is_err());

        delete_note(&state, id).unwrap();
        assert_eq!(list_notes(&state).unwrap().len(), 0);
    }

    #[test]
    fn stats_command_reports_frequencies() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::new(dir.path().to_path_buf()).unwrap();
        let state = AppState {
            store: std::sync::Mutex::new(store),
        };
        create_note(&state, "a", "rust rust").unwrap();
        let frequencies = note_stats(&state).unwrap();
        assert_eq!(frequencies[0], ("rust".to_string(), 2));
    }
}

//! Integration test for the capstone core library — exercises the public
//! `Store` API from outside, like the capstone's Tauri app would.

use course_examples::capstone::{ClipError, Note, Store};
use tempfile::tempdir;

#[test]
fn store_roundtrip_from_outside() {
    let dir = tempdir().unwrap();
    let mut store = Store::new(dir.path().to_path_buf()).unwrap();

    let id = store
        .save(&Note::new("integration", "test").unwrap())
        .unwrap();
    let note = store.get(id).unwrap().unwrap();
    assert_eq!(note.title, "integration");
    assert_eq!(note.body, "test");

    store.delete(id).unwrap();
    assert_eq!(store.get(id).unwrap(), None);
}

#[test]
fn missing_note_is_a_typed_error() {
    let dir = tempdir().unwrap();
    let store = Store::new(dir.path().to_path_buf()).unwrap();
    assert!(matches!(store.delete(42), Err(ClipError::NotFound(42))));
}

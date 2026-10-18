use crate::common::harness::{EditorTestHarness, HarnessOptions};
use crossterm::event::{KeyCode, KeyModifiers};

#[test]
fn test_send_selection_keeps_focus_in_editor() {
    let mut harness = EditorTestHarness::create(
        80,
        24,
        HarnessOptions::new().with_project_root(),
    ).unwrap();

    let file_path = harness.project_dir().unwrap().join("test.txt");
    std::fs::write(&file_path, "test text\n").unwrap();
    
    harness.open_file(&file_path).unwrap();
    harness.render().unwrap();

    harness.send_key(KeyCode::F(5), KeyModifiers::NONE).unwrap();
    harness.render().unwrap();

    // Check directly on the editor instance
    assert!(!harness.editor().is_terminal_mode());
}
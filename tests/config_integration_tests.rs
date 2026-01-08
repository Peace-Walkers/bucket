use std::path::PathBuf;

use bucket::config::load_config;

#[test]
fn load_config_with_custom_editor() {
    let old_editor = std::env::var("EDITOR").ok();
    let old_notes = std::env::var("NOTE_DIR").ok();

    unsafe {
        std::env::set_var("EDITOR", "/usr/bin/vim");
        std::env::set_var("NOTE_DIR", "/tmp/notes");
    }

    let config = load_config().expect("load config should succeed");

    assert_eq!(config.editor, PathBuf::from("/usr/bin/vim"));
    assert_eq!(config.note_dir, PathBuf::from("/tmp/notes"));

    // restore initial value
    unsafe {
        match old_editor {
            Some(val) => std::env::set_var("EDITOR", val),
            None => std::env::remove_var("EDITOR"),
        }

        match old_notes {
            Some(val) => std::env::set_var("NOTE_DIR", val),
            None => std::env::remove_var("NOTE_DIR"),
        }
    }
}

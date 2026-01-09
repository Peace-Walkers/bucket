# Storage

Provides filesystem-backed loading of Note objects grouped into Group structs.

## Overview

Storage scans a directory tree one level deep:
- Files located directly in the `root` directory are loaded into a special group named `all`.
- Each subdirectory of `root` becomes a group named after the directory; files inside that subdirectory become notes of that group.
- If the `root` path does not exist, it is created.

This module intentionally only supports a single level of groups (no nested subgroups).

## Public API

- `Storage::load_groups(root: &Path) -> anyhow::Result<Vec<Group>>`  
    Loads all notes and groups from `root`. Returns a Vec of `Group` where the first element is the `all` group (notes from `root`), followed by one `Group` per subdirectory. Errors are wrapped in `anyhow::Error` with descriptive messages on failure to open or create directories.

## Internal

- `Storage::load_notes(group_path: &Path) -> anyhow::Result<Vec<Note>>`  
    Reads directory entries under `group_path` and constructs `Note` instances for each regular file. The note's group name is set to the directory name (if present). Returns a Vec of notes.

## Behavior & Implementation Notes

- Root directory creation: if `root` does not exist, `load_groups` creates it using `fs::create_dir`.
- Only regular files are treated as notes; directories are skipped.
- Only a single directory level is inspected; nested folders are ignored.
- Group ordering is not guaranteed; callers may sort groups by name if deterministic ordering is required.

## Example

```rust
use std::path::Path;
let groups = Storage::load_groups(Path::new("/path/to/notes"))?;
for g in groups {
        println!("Group: {}", g.name());
        for n in g.notes() {
                println!(" - {}", n.name());
        }
}
```

## Tests

The module includes unit tests verifying:
- Loading a single group directory with one note.
- Loading multiple group directories.
- Loading only orphan notes in the root directory (the `all` group).

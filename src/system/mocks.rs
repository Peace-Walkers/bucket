#![cfg(test)]
use crate::system::traits::Editor;
use std::cell;

struct MockEditor {
    pub called_with: cell::RefCell<Vec<std::path::PathBuf>>,
}

impl MockEditor {
    fn new() -> Self {
        Self {
            called_with: cell::RefCell::new(vec![]),
        }
    }
}

impl Editor for MockEditor {
    fn open(&self, _editor: &std::path::Path, file: &std::path::Path) -> anyhow::Result<()> {
        self.called_with.borrow_mut().push(file.to_path_buf());
        Ok(())
    }
}

mod editor_test {
    use crate::{
        cli::Args,
        config::Config,
        system::{editor::open_editor, mocks::MockEditor},
    };

    #[test]
    fn open_editor_calls_editor_with_correct_path() -> anyhow::Result<()> {
        let mock = MockEditor::new();
        let config = Config::from("/usr/bin/vim".into(), "/tmp/notes".into());

        let args = Args {
            name: Some("test.bck".into()),
            group: None,
        };

        open_editor(&mock, &config, &args)?;
        let calls = mock.called_with.borrow();
        assert_eq!(calls.len(), 1);
        assert!(calls[0].ends_with("test.bck"));

        Ok(())
    }
}

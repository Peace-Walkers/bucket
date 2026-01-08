use crate::core::notes::Note;

pub struct Group {
    name: String,
    notes: Vec<Note>,
}

impl Group {
    pub fn new(name: &str, notes: Option<Vec<Note>>) -> Self {
        let notes = notes.unwrap_or_default();

        Self {
            name: name.to_string(),
            notes,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn notes(&self) -> &Vec<Note> {
        &self.notes
    }

    pub fn assign_note(&mut self, mut note: Note) {
        note.assign_group(&self.name);
        self.notes.push(note);
    }
}

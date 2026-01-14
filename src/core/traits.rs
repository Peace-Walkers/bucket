pub trait NoteInfos {
    fn name(&self) -> Option<&str>;
    fn group(&self) -> Option<&str>;
}

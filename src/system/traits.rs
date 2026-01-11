pub trait Editor {
    fn open(&self, editor: &std::path::Path, file: &std::path::Path) -> anyhow::Result<()>;
}

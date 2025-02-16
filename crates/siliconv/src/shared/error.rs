#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to load replay")]
    LoadError,
}

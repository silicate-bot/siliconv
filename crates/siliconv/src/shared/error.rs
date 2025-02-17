#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to load replay")]
    LoadError,

    #[error("Failed to perform IO: {0}")]
    IOError(#[from] std::io::Error),
}

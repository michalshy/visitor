#[derive(thiserror::Error, Debug)]
pub enum VError {
    #[error("failed to read: {e}")]
    Read { e: std::io::Error },

    #[error("failed to create: {e}")]
    Create { e: std::io::Error },

    #[error("failed to copy: {e}")]
    Copy { e: std::io::Error },

    #[error("failed to delete: {e}")]
    Delete { e: std::io::Error },

    #[error("failed to delete: {e}")]
    Rename { e: std::io::Error }
}
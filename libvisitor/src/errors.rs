#[derive(thiserror::Error, Debug)]
pub enum VError {
    #[error("failed to read file: {e}")]
    FileReadError { e: std::io::Error },

    #[error("failed to create directory: {e}")]
    DirCreateError { e: std::io::Error },

    #[error("failed to create file: {e}")]
    FileCreateError { e: std::io::Error },

    #[error("failed to copy: {e}")]
    FileCopyError { e: std::io::Error }
}
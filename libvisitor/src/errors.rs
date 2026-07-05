#[derive(thiserror::Error, Debug)]
pub enum VError {
    #[error("failed to read file: {e}")]
    FileReadError { e: std::io::Error },
}
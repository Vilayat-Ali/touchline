use thiserror::Error;

pub type Result<T> = std::result::Result<T, SeederError>;

#[derive(Debug, Error)]
pub enum SeederError {
    #[error("[Seeder Error] API Error: {0}")]
    APIError(#[from] reqwest::Error),

    #[error("[Seeder Error] IO Error: {0}")]
    IOError(#[from] std::io::Error),
}

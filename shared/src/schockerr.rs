use thiserror::*;

pub type Result<T> = std::result::Result<T, Schockerr>;

#[derive(Debug, Error)]
pub enum Schockerr {
    #[error("Could not bind")]
    ServerBindError(#[from] std::io::Error),
}

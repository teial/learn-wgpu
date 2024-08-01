use thiserror::Error;
use winit::error::EventLoopError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Application error")]
    ApplicationError(#[from] EventLoopError),
}

pub type Result<T> = std::result::Result<T, Error>;

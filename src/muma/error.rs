use thiserror::Error;
use crate::muma::Id;


#[derive(Debug, Error)]
pub enum MumaError
{
    #[error("{0}")]
    MalformedId(#[from] uuid::Error),

    #[error("The Task with the id '{0:?}' does not exist")]
    TaskDoesNotExist(Id),

    #[error("{0}")]
    IO(#[from] std::io::Error),

    #[error("{0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("{0}")]
    TomlDeError(#[from] toml::de::Error),
}


pub type MumaResult<T> = Result<T, MumaError>;


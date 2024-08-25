use std::{error, fmt};
use std::env::VarError;
use reqwest::Error;

#[derive(Debug)]
pub enum ApplicationError {
    None,
    VarError(VarError),
    ReqwestError(Error),
    OriginalError(String),
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationError::None => {
                write!(f, "Application Error: None")
            },
            ApplicationError::VarError(e) => {
                write!(f, "Application Error of type VarError: {}", e)
            },
            ApplicationError::ReqwestError(e) => {
                write!(f, "Application Error of type ReqwestError: {}", e)
            },
            ApplicationError::OriginalError(e) => {
                write!(f, "Application Error of type OriginalError: {}", e)
            }
        }
    }
}

impl error::Error for ApplicationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ApplicationError::None => None,
            ApplicationError::VarError(e) => Some(e),
            ApplicationError::ReqwestError(e) => Some(e),
            ApplicationError::OriginalError(e) => None
        }
    }
}

impl From<VarError> for ApplicationError {
    fn from(err: VarError) -> ApplicationError {
        ApplicationError::VarError(err)
    }
}

impl From<reqwest::Error> for ApplicationError {
    fn from(err: reqwest::Error) -> ApplicationError {
        ApplicationError::ReqwestError(err)
    }
}
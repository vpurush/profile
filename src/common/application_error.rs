use std::{error, fmt};
use std::env::VarError;
use std::str::FromStr;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ApplicationError {
    None,
    VarError(VarError),
    ReqwestError(Error),
    OriginalError(String),
    NotFound,
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
            },
            ApplicationError::NotFound => {
                write!(f, "Not found")
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
            ApplicationError::OriginalError(e) => None,
            ApplicationError::NotFound => None,
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

impl FromStr for ApplicationError {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ApplicationError::OriginalError(String::from(s)))
    }
}
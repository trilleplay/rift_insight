use std::{fmt, io::Error as IOError};

use reqwest::Error as RequestError;

#[derive(Debug)]
pub struct RiftApiRequestError {
    _error: RequestError
}

impl RiftApiRequestError {
    pub(crate) fn new(error: RequestError) -> RiftApiRequestError {
        return RiftApiRequestError {_error: error};
    }
}

impl fmt::Display for RiftApiRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum RiftInitializationError {
    GenericLockfile(LockFileError),
    FailedToOpenLockfile(FailedToOpenLockfileError),
    UnknownError(String)
}


#[derive(Debug)]
pub struct LockFileError {
    _error: LockfileErrorTypes
}

impl LockFileError {
    pub(crate) fn new(error: LockfileErrorTypes) -> LockFileError {
        return LockFileError {_error: error};
    }
}

#[derive(Debug)]
pub enum LockfileErrorTypes {
    LeagueNotRunning,
    LockfileUnexpectedFormat
}

impl fmt::Display for LockFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct FailedToOpenLockfileError {
    _error: IOError
}

impl FailedToOpenLockfileError {
    pub(crate) fn new(error: IOError) -> FailedToOpenLockfileError {
        return FailedToOpenLockfileError {_error: error};
    }
}

impl fmt::Display for FailedToOpenLockfileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
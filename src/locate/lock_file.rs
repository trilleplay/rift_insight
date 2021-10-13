use crate::errors::{LockFileError, RiftInitializationError, LockfileErrorTypes, FailedToOpenLockfileError};

use std::fs::read_to_string;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct LockfileContents {
    pub port: u16,
    pub password: String,
    pub protocol: String
}

fn find_lockfile_location() -> Result<String, RiftInitializationError> {
    let out = Command::new("wmic")
        .args(["PROCESS",  "WHERE", "name='LeagueClientUx.exe'", "GET", "ExecutablePath"])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let out_string = String::from_utf8(out.stdout).unwrap();
    let mut stdout = out_string.lines();
    if !stdout.next().unwrap_or("").starts_with("ExecutablePath") {
        return Err(RiftInitializationError::GenericLockfile(LockFileError::new(LockfileErrorTypes::LeagueNotRunning)));
    }
    Ok(stdout.next().unwrap().to_string())
}

pub fn get_lockfile() -> Result<LockfileContents, RiftInitializationError> {
    if cfg!(windows) {
        let location = find_lockfile_location();
        match location {
            Ok(l) => {
                let mut file_path = l.replace("LeagueClientUx.exe", "lockfile").replace("\r", "").replace("\\", "\\\\");
                    while file_path.ends_with(" ") {
                    file_path.pop();
                }
                let file = read_to_string(file_path);
                match file {
                    Ok(f) => {
                        let split: Vec<&str> = f.split(":").collect();
                        if split.len() != 5 {
                            return Err(RiftInitializationError::GenericLockfile(LockFileError::new(LockfileErrorTypes::LockfileUnexpectedFormat)))
                        }
                        return Ok(LockfileContents {
                            port: split[2].parse::<u16>().unwrap_or(0),
                            password: split[3].to_string(),
                            protocol: split[4].to_string()
                        });
                    },
                    Err(e) => {return Err(RiftInitializationError::FailedToOpenLockfile(FailedToOpenLockfileError::new(e)))}
                }
            }
            Err(e) => {return Err(e)}
        }
    } else {
        return Err(RiftInitializationError::UnknownError("Unsupported platform.".to_string()))
    }
    
}
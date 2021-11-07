use crate::errors::{LockFileError, RiftInitializationError, LockfileErrorTypes, FailedToOpenLockfileError};

use std::fs::read_to_string;
use std::process::{Command, Stdio};

use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref REMOTING_AUTH_TOKEN: Regex = Regex::new("--remoting-auth-token=([\\w-]*)").unwrap();
    static ref APP_PORT: Regex = Regex::new("--app-port=([0-9]*)").unwrap();
}

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
        return match location {
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
                        Ok(LockfileContents {
                            port: split[2].parse::<u16>().unwrap_or(0),
                            password: split[3].to_string(),
                            protocol: split[4].to_string()
                        })
                    },
                    Err(e) => { Err(RiftInitializationError::FailedToOpenLockfile(FailedToOpenLockfileError::new(e))) }
                }
            }
            Err(e) => { Err(e) }
        }
    } else {
        if cfg!(target_os = "macos") {
            let out = Command::new("ps")
                .args(["-A"])
                .stdout(Stdio::piped())
                .output()
                .unwrap();
            let out_string = String::from_utf8(out.stdout).unwrap();
            let mut stdout = out_string.lines()
                .filter(|line| line.find("LeagueClientUx").is_some());
            let first_line = stdout.next().unwrap_or("");
            if !first_line.contains("LeagueClientUx") {
                return Err(RiftInitializationError::GenericLockfile(LockFileError::new(LockfileErrorTypes::LeagueNotRunning)));
            }
            match REMOTING_AUTH_TOKEN.find(first_line) {
                Some(remote_auth) => {
                    match APP_PORT.find(first_line) {
                        Some(port) => {
                            return Ok(LockfileContents {
                                
                                port: port.as_str().split("=").collect::<Vec<&str>>()[1].parse::<u16>().unwrap(),
                                password: remote_auth.as_str().split("=").collect::<Vec<&str>>()[1].to_string(),
                                protocol: "https".to_string()
                            })
                        }
                        None => {
                            return Err(RiftInitializationError::GenericLockfile(LockFileError::new(LockfileErrorTypes::LeagueNotRunning)));
                        }
                    }
                },
                None => {
                    return Err(RiftInitializationError::GenericLockfile(LockFileError::new(LockfileErrorTypes::LeagueNotRunning)));
                }
            }
        } else {
            return Err(RiftInitializationError::UnknownError("Unsupported platform.".to_string()))
        }
    }
    
}
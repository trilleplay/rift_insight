use crate::{locate::lock_file::LockfileContents, errors::RiftApiRequestError};
use super::Methods;

use log::{debug};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize as SerializeDerive;
use serde::ser::Serialize;

pub struct RequestClient {
    pub http: Client,
    pub lockfile: LockfileContents
}

// This is a very stupid fix, but considering that Rust does not have never types on stable, nor is setting a default generic type allowed, I'm left with no choice.
#[derive(SerializeDerive)]
struct EmptyGetRequestBody {
}

impl RequestClient {
    /// A generic request method, keep in mind that `path` should not start with a /, since the library already puts it in there.
    pub async fn request<T: DeserializeOwned, S: Serialize + Sized>(&mut self, method: Methods, path: &str, body: Option<S>) -> Result<T, RiftApiRequestError> {
        let url = format!("{}://127.0.0.1:{}/{}", self.lockfile.protocol, self.lockfile.port, path);
        debug!("Constructed url for {:?} request: {}", method, url);
        let request = self.http.request(method.as_reqwest(), url)
                        .basic_auth("riot", Some(self.lockfile.password.clone()));
        let response = match body {
            Some(b) => request.json(&b).send().await,
            None => request.send().await,
        };
        return match response {
            Ok(r) => {
                let parsed = r.json::<T>().await;
                match parsed {
                    Ok(p) => Ok(p),
                    Err(e) => Err(RiftApiRequestError::new(e)),
                }
            }
            Err(e) => {
                Err(RiftApiRequestError::new(e))
            }
        }
    }

    // Perform a GET request, keep in mind that `path` should not start with a /, since the library already puts it in there.
    pub async fn get<T: DeserializeOwned>(&mut self, path: &str) -> Result<T, RiftApiRequestError> {
        return self.request::<T, EmptyGetRequestBody>(Methods::Get, path, None).await;
    }

    // Perform a POST request, keep in mind that `path` should not start with a /, since the library already puts it in there.
    pub async fn post<T: DeserializeOwned, S: Serialize + Sized>(&mut self, path: &str, body: Option<S>) -> Result<T, RiftApiRequestError> {
        return self.request::<T, S>(Methods::Post, path, body).await;
    }

    // Perform a PUT request, keep in mind that `path` should not start with a /, since the library already puts it in there.
    pub async fn put<T: DeserializeOwned, S: Serialize + Sized>(&mut self, path: &str, body: Option<S>) -> Result<T, RiftApiRequestError> {
        return self.request::<T, S>(Methods::Put, path, body).await;
    }

    // Perform a DELETE request, keep in mind that `path` should not start with a /, since the library already puts it in there.
    pub async fn delete<T: DeserializeOwned, S: Serialize + Sized>(&mut self, path: &str, body: Option<S>) -> Result<T, RiftApiRequestError> {
        return self.request::<T, S>(Methods::Delete, path, body).await;
    }

    // Perform a HEAD request, keep in mind that `path` should not start with a /, since the library already puts it in there.
    pub async fn head<T: DeserializeOwned, S: Serialize + Sized>(&mut self, path: &str) -> Result<T, RiftApiRequestError> {
        return self.request::<T, S>(Methods::Head, path, None).await;
    }
}

pub fn get_request_client(lockfile: LockfileContents) -> RequestClient {
    let http = Client::builder()
        .add_root_certificate(super::security::get_certificate())
        .build()
        .unwrap();
    return RequestClient { http, lockfile };
}

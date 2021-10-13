use crate::{locate::lock_file::LockfileContents, errors::RiftApiRequestError};
use super::Methods;

use log::{debug};
use reqwest::Client;
use serde_json::value::Value as AnyValue;



pub struct RequestClient {
    pub http: Client,
    pub lockfile: LockfileContents
}

impl RequestClient {
    /// A generic request method, keep in mind that `path` should not start with a /, since the library already puts it in there.
    pub async fn request(&mut self, method: Methods, path: &str, body: Option<AnyValue>) -> Result<AnyValue, RiftApiRequestError> {
        let url = format!("{}://127.0.0.1:{}/{}", self.lockfile.protocol, self.lockfile.port, path);
        debug!("Constructed url for {:?} request: {}", method, url);
        let request = self.http.request(method.as_reqwest(), url)
                        .basic_auth("riot", Some(self.lockfile.password.clone()));
        let response = match body {
            Some(b) => request.json(&b).send().await,
            None => request.send().await,
        };
        match response {
            Ok(r) => {
                let parsed = r.json::<AnyValue>().await;
                match parsed {
                    Ok(p) => return Ok(p),
                    Err(e) => return Err(RiftApiRequestError::new(e)),
                };
            }
            Err(e) => {
                return Err(RiftApiRequestError::new(e));
            }
        }
    } 

    // Perform a GET request, keep in mind that `path` should not start with a /, since the library already puts it in there.
    pub async fn get(&mut self, path: &str) -> Result<AnyValue, RiftApiRequestError> {
        return self.request(Methods::Get, path, None).await;
    }

    // Perform a POST request, keep in mind that `path` should not start with a /, since the library already puts it in there.
    pub async fn post(&mut self, path: &str, body: AnyValue) -> Result<AnyValue, RiftApiRequestError> {
        return self.request(Methods::Post, path, Some(body)).await;
    }

    // Perform a DELETE request, keep in mind that `path` should not start with a /, since the library already puts it in there.
    pub async fn delete(&mut self, path: &str, body: AnyValue) -> Result<AnyValue, RiftApiRequestError> {
        return self.request(Methods::Delete, path, Some(body)).await;
    }

    // Perform a HEAD request, keep in mind that `path` should not start with a /, since the library already puts it in there.
    pub async fn head(&mut self, path: &str) -> Result<AnyValue, RiftApiRequestError> {
        return self.request(Methods::Head, path, None).await;
    }
}

pub fn get_request_client(lockfile: LockfileContents) -> RequestClient {
    let http = Client::builder()
        .add_root_certificate(super::security::get_certificate())
        .build()
        .unwrap();
    return RequestClient { http: http, lockfile: lockfile};
}

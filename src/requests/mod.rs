pub mod lcu;
pub mod live_client_data;

use reqwest::Method as ReqwestMethod;

#[derive(Debug, Clone, Copy)]
/// Available Request Methods
pub enum Methods {
    Get,
    Post,
    Delete,
    Head
}

impl Methods {
    pub(crate) const fn as_reqwest(self) -> ReqwestMethod {
        match self {
            Self::Get => ReqwestMethod::GET,
            Self::Post => ReqwestMethod::POST,
            Self::Delete => ReqwestMethod::DELETE,
            Self::Head => ReqwestMethod::PATCH,
        }
    }
}

mod security;
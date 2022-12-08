use crate::hostcalls::hostcall_networking_request;
use serde::{Deserialize, Serialize};
use std::slice;
use std::str;

#[derive(Debug, Serialize)]
pub enum NetworkingRequestMethod {
    Get,
    Post,
    Delete,
    Put,
    Patch,
    Head,
    Options,
}

pub type NetworkingHeaders = Vec<(String, String)>;
pub type NetworkingBody = Option<Vec<u8>>;

#[derive(Debug, Serialize)]
pub struct NetworkingRequest {
    pub method: NetworkingRequestMethod,
    pub url: String,
    pub headers: NetworkingHeaders,
    pub body: NetworkingBody,
}
impl NetworkingRequest {
    pub fn get(url: String, headers: NetworkingHeaders) -> Self {
        Self {
            method: NetworkingRequestMethod::Get,
            url,
            headers,
            body: None,
        }
    }
    pub fn post(url: String, headers: NetworkingHeaders, body: NetworkingBody) -> Self {
        Self {
            method: NetworkingRequestMethod::Post,
            url,
            headers,
            body,
        }
    }
    pub fn put(url: String, headers: NetworkingHeaders, body: NetworkingBody) -> Self {
        Self {
            method: NetworkingRequestMethod::Put,
            url,
            headers,
            body,
        }
    }
    pub fn delete(url: String, headers: NetworkingHeaders, body: NetworkingBody) -> Self {
        Self {
            method: NetworkingRequestMethod::Delete,
            url,
            headers,
            body,
        }
    }
    pub fn patch(url: String, headers: NetworkingHeaders, body: NetworkingBody) -> Self {
        Self {
            method: NetworkingRequestMethod::Patch,
            url,
            headers,
            body,
        }
    }
    pub fn head(url: String, headers: NetworkingHeaders) -> Self {
        Self {
            method: NetworkingRequestMethod::Head,
            url,
            headers,
            body: None,
        }
    }
    pub fn options(url: String, headers: NetworkingHeaders) -> Self {
        Self {
            method: NetworkingRequestMethod::Get,
            url,
            headers,
            body: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NetworkingResponse {
    pub status: u16,
    pub headers: NetworkingHeaders,
    pub body: NetworkingBody,
}

impl FromStr for NetworkingResponse {
    type Err = NetworkingError;

    fn from_str(response_str: &str) -> Result<Self, Self::Err> {
        let response_data: NetworkingResponseData = serde_json::from_str(response_str).unwrap();
        if let Some(response) = response_data.response{
            Ok(response)
        }else if let Some(message) = response_data.message{
            Err(NetworkingError { code: response_data.code, message })
        }else{
            panic!("Unknown response")
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NetworkingError {
    pub code: u8,
    pub message: String,
}

#[derive(Debug, Deserialize)]
struct NetworkingResponseData {
    pub code: u8,
    pub response: Option<NetworkingResponse>,
    pub message: Option<String>,
}

pub fn request(request: NetworkingRequest) -> Result<NetworkingResponse, NetworkingError> {
    let request_binary = serde_json::to_string(&request).unwrap();
    let mut response_str = "";
    response_str = unsafe {
        let len = hostcall_networking_request(
            request_binary.as_ptr(),
            request_binary.len(),
            response_str.as_ptr(),
        );
        let slice = slice::from_raw_parts(response_str.as_ptr(), len);
        str::from_utf8(slice).unwrap()
    };
    NetworkingResponse::from_str(response_str)
}
use crate::hostcalls::hostcall_networking_request;
use core::str;
use core::{slice, str::FromStr};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub enum NetworkingRequestMethod {
    Get,
    Post,
    Delete,
    Put,
    Patch,
    Head,
    Options,
}

impl Serialize for NetworkingRequestMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Get => serializer.serialize_str("get"),
            Self::Post => serializer.serialize_str("post"),
            Self::Delete => serializer.serialize_str("delete"),
            Self::Put => serializer.serialize_str("put"),
            Self::Patch => serializer.serialize_str("patch"),
            Self::Head => serializer.serialize_str("head"),
            Self::Options => serializer.serialize_str("options"),
        }
    }
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
        match serde_json::from_str::<Value>(response_str) {
            Ok(value) => match value.get("code").unwrap().as_u64() {
                Some(0) => serde_json::from_value::<NetworkingResponse>(
                    value.get("response").unwrap().to_owned(),
                )
                .map_err(|_err| panic!("Bad Response")),
                Some(code) => Err(NetworkingError {
                    code: code as u8,
                    message: value.get("message").unwrap().as_str().unwrap().to_owned(),
                }),
                None => panic!("Bad Response"),
            },
            Err(_err) => {
                panic!("Bad Response")
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NetworkingError {
    pub code: u8,
    pub message: String,
}

pub fn request(request_data: &NetworkingRequest) -> Result<NetworkingResponse, NetworkingError> {
    let request_binary = serde_json::to_string(request_data).unwrap();

    let response_str = Vec::with_capacity(20000);

    let response_str = unsafe {
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

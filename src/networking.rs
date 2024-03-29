use crate::hostcalls::{hostcall_networking_request, hostcall_networking_retrieve_response};
use crate::memory::__wasm_malloc;

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
//Due to the vec to str conversion after reading the data (see line 149 for details), the type of the body is changed from vec<u8> to String so that the deserialization can succeed
pub type NetworkingBody = Option<String>;

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

pub fn request(resquest: &NetworkingRequest) -> Result<NetworkingResponse, NetworkingError> {
    let request_binary = serde_json::to_string(&resquest).unwrap();
    let response_len =
        unsafe { hostcall_networking_request(request_binary.as_ptr(), request_binary.len()) };
    let response_ptr = __wasm_malloc(response_len);
    let response_str = unsafe {
        hostcall_networking_retrieve_response(response_ptr);
        let slice = slice::from_raw_parts(response_ptr, response_len);
        str::from_utf8(slice).unwrap()
    };
    NetworkingResponse::from_str(response_str)
}

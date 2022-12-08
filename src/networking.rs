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
    Patch, //Additions to the put method
    Head,  //same as get
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
    pub fn get(url: String) -> Self {
        Self {
            method: NetworkingRequestMethod::Get,
            url,
            headers: Vec::new(),
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
    pub fn options(url: String) -> Self {
        Self {
            method: NetworkingRequestMethod::Options,
            url,
            headers: Vec::new(),
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

#[derive(Debug, Deserialize)]
pub struct NetworkingError {
    pub code: u8,
    pub message: String,
}

#[derive(Debug, Deserialize)]
struct NetworkingResponseData {
    code: u8,
    response: Option<NetworkingResponse>,
    message: Option<String>,
}
impl NetworkingResponseData {
    pub fn response_or_error(self) -> Result<NetworkingResponse, NetworkingError> {
        if let Some(response) = self.response {
            Ok(response)
        } else if let Some(message) = self.message {
            Err(NetworkingError {
                code: self.code,
                message,
            })
        } else {
            panic!("")
        }
    }
}
pub fn request(request: NetworkingRequest) -> Result<NetworkingResponse, NetworkingError> {
    let request_binary = serde_json::to_string(&request).unwrap();
    let mut response_binary = "";
    response_binary = unsafe {
        let len = hostcall_networking_request(
            request_binary.as_ptr(),
            request_binary.len(),
            response_binary.as_ptr(),
        );
        let slice = slice::from_raw_parts(response_binary.as_ptr(), len);
        str::from_utf8(slice).unwrap()
    };
    let response_data: NetworkingResponseData = serde_json::from_str(response_binary).unwrap();
    //师傅 需要判断是否存在，这里感觉是不能使用Value的，因为NetworkingError不是直接string，他是一个新的在这边定义的数据类型，所以我们需要获得数据然后重新组合
    response_data.response_or_error()
}

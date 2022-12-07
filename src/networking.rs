use crate::hostcalls::hostcall_networking_request;
use serde::{Deserialize, Serialize};
use std::slice;
use std::str;


#[derive(Serialize)]
pub struct NetworkingRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl NetworkingRequest {
    pub fn get(url: String, headers: Vec<(String, String)>) -> NetworkingRequest {
        NetworkingRequest {
            method: "GET".to_string(),
            url,
            headers,
            body: None,
        }
    }
    pub fn post(
        url: String,
        headers: Vec<(String, String)>,
        body: Option<String>,
    ) -> NetworkingRequest {
        NetworkingRequest {
            method: "POST".to_string(),
            url,
            headers,
            body,
        }
    }
    pub fn put(
        url: String,
        headers: Vec<(String, String)>,
        body: Option<String>,
    ) -> NetworkingRequest {
        NetworkingRequest {
            method: "PUT".to_string(),
            url,
            headers,
            body,
        }
    }
    pub fn delete(
        url: String,
        headers: Vec<(String, String)>,
        body: Option<String>,
    ) -> NetworkingRequest {
        NetworkingRequest {
            method: "DELETE".to_string(),
            url,
            headers,
            body,
        }
    }
    pub fn head(
        url: String,
        headers: Vec<(String, String)>,
        body: Option<String>,
    ) -> NetworkingRequest {
        NetworkingRequest {
            method: "HEAD".to_string(),
            url,
            headers,
            body,
        }
    }
    pub fn patch(
        url: String,
        headers: Vec<(String, String)>,
        body: Option<String>,
    ) -> NetworkingRequest {
        NetworkingRequest {
            method: "PATCH".to_string(),
            url,
            headers,
            body,
        }
    }
    pub fn options(
        url: String,
        headers: Vec<(String, String)>,
        body: Option<String>,
    ) -> NetworkingRequest {
        NetworkingRequest {
            method: "OPTIONS".to_string(),
            url,
            headers,
            body,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct NetworkingResponse {
    pub code: Option<u8>,
    pub response: Option<Response>,
    pub message: Option<String>,
}

impl NetworkingResponse {
    pub fn response_data(self) -> ResponseData {
        if let Some(_code) = self.code {
            if let Some(response) = self.response {
                ResponseData::Response(response)
            } else if let Some(message) = self.message {
                ResponseData::Message(message)
            } else {
                panic!("UnexpectedResponse")
            }
        } else {
            panic!("UnexpectedResponse")
        }
    }
}
#[derive(Debug)]
pub enum ResponseData {
    Message(String),
    Response(Response),
}
#[derive(Deserialize, Debug)]
pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

pub fn request(request_data: NetworkingRequest) -> &'static str {
    let request_data = serde_json::to_string(&request_data).unwrap();
    let response_data = "";
    unsafe {
        let len = hostcall_networking_request(
            request_data.as_ptr(),
            request_data.len(),
            response_data.as_ptr(),
        );
        let slice = slice::from_raw_parts(response_data.as_ptr(), len);
        str::from_utf8(slice).unwrap()
    }
}

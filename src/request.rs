use serde::{Deserialize, Serialize};
use std::slice;
use std::str;
use crate::hostcalls::hostcall_networking_request;
#[derive(Serialize)]
pub struct RequestData {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl RequestData {
    pub fn new(
        method: String,
        url: String,
        headers: Vec<(String, String)>,
        body: Option<String>,
    ) -> RequestData {
        RequestData {
            method,
            url,
            headers,
            body,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ResponseData {
    pub code: Option<u8>,
    pub response: Option<Response>,
    pub message: Option<String>,
}

impl ResponseData {
    pub fn print_response(self) -> Response {
        if let Some(_code) = self.code {
            if let Some(response) = self.response {
                response
            } else if let Some(message) = self.message {
                panic!("{}", message);
            } else {
                panic!("Response And Message Value is null");
            }
        } else {
            panic!("UnexpectedResponse");
        }
    }
    pub fn deserde(response_data: &str) -> Self{
        serde_json::from_str(response_data).unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

pub fn network(request_data: RequestData) -> &'static str {
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
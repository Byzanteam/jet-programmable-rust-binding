use crate::hostcalls::hostcall_networking_request;
use serde::{Deserialize, Serialize};
use std::slice;
use std::str;


pub enum NetworkingRequest {
    Get(Get),
    Post(Post),
    Delete(Delete),
    Put(Put),
    Patch(Patch), //Additions to the put method
    Head(Head), //same as get
    Options(Options),
}
#[derive(Serialize)]
pub struct Get {
    pub method: String,
    pub url: String,
}
impl Get {
    pub fn request_body(url: String) -> Self {
        Get {
            method: "GET".to_string(),
            url,
        }
    }
}
#[derive(Serialize)]
pub struct Post {
    pub method: String,
    pub url: String,
    pub body: String,
    pub headers: Vec<(String, String)>,
}
impl Post {
    pub fn request_body(url: String, body: String, headers: Vec<(String, String)>) -> Self {
        Post {
            method: "POST".to_string(),
            url,
            body,
            headers,
        }
    }
}
#[derive(Serialize)]
pub struct Put {
    pub method: String,
    pub url: String,
    pub body: String,
    pub headers: Vec<(String, String)>,
}
impl Put {
    pub fn request_body(url: String, body: String, headers: Vec<(String, String)>) -> Self {
        Put {
            method: "PUT".to_string(),
            url,
            body,
            headers,
        }
    }
}
#[derive(Serialize)]
pub struct Delete {
    pub method: String,
    pub url: String,
    pub body: String,
    pub headers: Vec<(String, String)>,
}
impl Delete {
    pub fn request_body(url: String, body: String, headers: Vec<(String, String)>) -> Self {
        Delete {
            method: "DELETE".to_string(),
            url,
            body,
            headers,
        }
    }
}
#[derive(Serialize)]
pub struct Head {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
}
impl Head {
    pub fn request_body(url: String, headers: Vec<(String, String)>) -> Self {
        Head {
            method: "HEAD".to_string(),
            url,
            headers,
        }
    }
}
#[derive(Serialize)]
pub struct Patch {
    pub method: String,
    pub url: String,
    pub body: String,
    pub headers: Vec<(String, String)>,
}
impl Patch {
    pub fn request_body(url: String, body: String, headers: Vec<(String, String)>) -> Self {
        Patch {
            method: "PATCH".to_string(),
            url,
            body,
            headers,
        }
    }
}
#[derive(Serialize)]
pub struct Options {
    pub method: String,
    pub url: String,
}
impl Options {
    pub fn request_body(url: String) -> Self {
        Options {
            method: "OPTIONS".to_string(),
            url,
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
pub fn request(request_data: NetworkingRequest) -> ResponseData {
    let request_binary = match request_data {
        NetworkingRequest::Get(get) => serde_json::to_string(&get).unwrap(),
        NetworkingRequest::Post(post) => serde_json::to_string(&post).unwrap(),
        NetworkingRequest::Put(put) => serde_json::to_string(&put).unwrap(),
        NetworkingRequest::Delete(delete) => serde_json::to_string(&delete).unwrap(),
        NetworkingRequest::Head(head) => serde_json::to_string(&head).unwrap(),
        NetworkingRequest::Patch(patch) => serde_json::to_string(&patch).unwrap(),
        NetworkingRequest::Options(options) => serde_json::to_string(&options).unwrap(),
    };
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
    let response_data: NetworkingResponse = serde_json::from_str(response_binary).unwrap();
    response_data.response_data()
}

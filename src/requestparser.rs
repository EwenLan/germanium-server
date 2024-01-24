use log::{debug, error};

#[derive(Debug)]
pub enum HTTPRequestMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Debug)]
pub enum HTTPVersion {
    HTTP1_1,
}

#[derive(Debug)]
pub struct HTTPRequest {
    pub request_method: HTTPRequestMethod,
    pub http_version: HTTPVersion,
    pub path: String,
}

impl HTTPRequest {
    pub fn new(request_line: &str) -> Self {
        let mut parts = request_line.split(" ");
        let method = Self::convert_request_method(parts.next().unwrap());
        let path = parts.next().unwrap();
        let version = Self::convert_http_request_version(parts.next().unwrap());
        HTTPRequest {
            request_method: method,
            http_version: version,
            path: path.to_string(),
        }
    }

    fn convert_request_method(method: &str) -> HTTPRequestMethod {
        match method {
            "GET" => HTTPRequestMethod::Get,
            "POST" => HTTPRequestMethod::Post,
            "PUT" => HTTPRequestMethod::Put,
            "DELETE" => HTTPRequestMethod::Delete,
            f => {
                error!("unknown HTTP request method {f}, use GET as default method");
                HTTPRequestMethod::Get
            }
        }
    }

    fn convert_http_request_version(_: &str) -> HTTPVersion {
        debug!("only support http 1.1");
        HTTPVersion::HTTP1_1
    }
}

use crate::requestparser::HTTPVersion;
use log::error;

pub struct HTTPResponse {
    http_version: HTTPVersion,
    status_code: i32,
    body: String,
}

impl HTTPResponse {
    pub fn new(http_version: HTTPVersion, status_code: i32, body: &str) -> Self {
        HTTPResponse {
            http_version: http_version,
            status_code: status_code,
            body: body.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        String::from(format!(
            "{4} {0} {1}\r\nContent-Length: {2}\r\n\r\n{3}",
            self.status_code,
            self.get_status_title(),
            self.body.len(),
            self.body,
            self.get_http_version(),
        ))
    }

    fn get_status_title(&self) -> &str {
        match self.status_code {
            200 => "OK",
            t => {
                error!("unsupport status code {t}");
                "Bad Request"
            }
        }
    }

    fn get_http_version(&self) -> &str {
        match &self.http_version {
            HTTPVersion::HTTP1_1 => "HTTP/1.1",
        }
    }
}

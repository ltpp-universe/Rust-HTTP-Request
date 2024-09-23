use crate::error::error::Error;
use crate::r#type::r#type::{HttpRequest, HttpRequestBuilder, HTTP_BR, HTTP_DOUBLE_BR};
use crate::{methods::methods::Methods, protocol::protocol::Protocol};
use global_type::r#type::r#type::{Body, Header};
use request_url::r#type::r#type::Url;
use std::io::{Read, Write};
use std::sync::Arc;
use std::{collections::HashMap, net::TcpStream};

impl HttpRequest {
    fn get_protocol(&self) -> Protocol {
        self.protocol.as_ref().clone()
    }

    fn get_methods(&self) -> Methods {
        self.methods.as_ref().clone()
    }

    fn get_url(&self) -> String {
        self.url.as_ref().clone()
    }

    fn get_header(&self) -> Header {
        self.header.as_ref().clone()
    }

    fn get_body(&self) -> Body {
        self.body.as_ref().clone()
    }

    fn parse_url(&self) -> Result<Url, Error> {
        if let Ok(parse_res) = Url::parse(&self.get_url()) {
            Ok(parse_res)
        } else {
            Err(Error::InvalidUrl)
        }
    }

    fn get_header_str(&self) -> String {
        let header: HashMap<String, String> = self.get_header();
        let mut header_string: String = String::new();
        for (key, value) in header {
            let line: String = format!("{}: {}{}", key, value, HTTP_BR);
            header_string.push_str(&line);
        }
        header_string
    }

    fn get_body_str(&self) -> String {
        let body: HashMap<String, String> = self.get_body();
        let body_str: String = body.iter().map(|(k, v)| format!("{}={}&", k, v)).collect();
        let body_str: &str = body_str.trim_end_matches('&');
        body_str.to_owned()
    }

    fn send_get_request(&self, stream: &mut TcpStream, url_obj: &Url) -> String {
        let path: String = url_obj.path.clone().unwrap_or("/".to_string());
        let mut request: String = format!("GET {} HTTP/1.1{}", path, HTTP_BR);
        request.push_str(&format!(
            "Host: {}{}",
            url_obj.host.as_ref().unwrap_or(&"".to_string()),
            HTTP_BR
        ));
        request.push_str(&self.get_header_str());
        request.push_str(HTTP_BR);
        stream.write_all(request.as_bytes()).unwrap();
        self.read_response(stream)
    }

    fn send_post_request(&self, stream: &mut TcpStream, url_obj: &Url) -> String {
        let path: String = url_obj.path.clone().unwrap_or("/".to_string());
        let mut request: String = format!("POST {} HTTP/1.1{}", path, HTTP_BR);
        request.push_str(&format!(
            "Host: {}{}",
            url_obj.host.as_ref().unwrap_or(&"".to_string()),
            HTTP_BR
        ));
        request.push_str(&self.get_header_str());
        let body_str: String = self.get_body_str();
        request.push_str(&format!("Content-Length: {}{}", body_str.len(), HTTP_BR));
        request.push_str(HTTP_BR);
        request.push_str(&format!("{}{}", body_str, HTTP_BR));
        stream.write_all(request.as_bytes()).unwrap();
        self.read_response(stream)
    }

    fn read_response(&self, stream: &mut TcpStream) -> String {
        let mut buffer: [u8; 10240] = [0; 10240];
        let mut response: String = String::new();
        let mut headers_done: bool = false;
        let mut content_length: usize = 0;
        while let Ok(n) = stream.read(&mut buffer) {
            if n == 0 {
                break;
            }
            response.push_str(&String::from_utf8_lossy(&buffer[..n]));
            if !headers_done {
                if let Some(pos) = response.find(HTTP_DOUBLE_BR) {
                    headers_done = true;
                    if let Some(length_pos) = response.to_lowercase().find("content-length:") {
                        let start = length_pos + "content-length:".len();
                        if let Some(end) = response[start..].find(HTTP_BR) {
                            content_length =
                                response[start..start + end].trim().parse().unwrap_or(0);
                        }
                    }
                    response = response.split_off(pos + 4);
                }
            }
            if headers_done && response.len() >= content_length {
                break;
            }
        }
        response
    }

    fn get_port(&self, port: u16) -> u16 {
        if port != 0 {
            return port;
        }
        let protocol = self.get_protocol();
        protocol.get_port()
    }

    pub fn send(&self) -> Result<String, Error> {
        if let Ok(url_obj) = self.parse_url() {
            let methods: Methods = self.get_methods();
            let host: String = url_obj.host.clone().unwrap_or_default();
            let port: u16 = self.get_port(url_obj.port.clone().unwrap_or_default());
            if let Ok(mut stream) = TcpStream::connect((host, port)) {
                let response: Result<String, Error> = match methods {
                    _methods if _methods.is_get() => {
                        Ok(self.send_get_request(&mut stream, &url_obj.clone()))
                    }
                    _methods if _methods.is_post() => {
                        Ok(self.send_post_request(&mut stream, &url_obj))
                    }
                    _ => Err(Error::RequestError),
                };
                response
            } else {
                Err(Error::TcpStreamConnectError)
            }
        } else {
            Err(Error::InvalidUrl)
        }
    }
}

impl Default for HttpRequest {
    fn default() -> HttpRequest {
        HttpRequest {
            methods: Arc::new(Methods::new()),
            url: Arc::new(String::new()),
            protocol: Arc::new(Protocol::new()),
            header: Arc::new(HashMap::new()),
            body: Arc::new(HashMap::new()),
        }
    }
}

impl Default for HttpRequestBuilder {
    fn default() -> HttpRequestBuilder {
        HttpRequestBuilder {
            tmp: HttpRequest::default(),
            builder: HttpRequest::default(),
        }
    }
}

impl HttpRequestBuilder {
    pub fn set_protocol(&mut self, protocol: Protocol) -> &mut Self {
        self.tmp.protocol = Arc::new(protocol);
        self
    }

    pub fn new() -> Self {
        HttpRequestBuilder::default()
    }

    pub fn set_methods(&mut self, methods: Methods) -> &mut Self {
        self.tmp.methods = Arc::new(methods);
        self
    }

    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.tmp.url = Arc::new(url.to_owned());
        self
    }

    pub fn set_header(&mut self, header: &Header) -> &mut Self {
        if let Some(tmp_header) = Arc::get_mut(&mut self.tmp.header) {
            for (key, value) in header {
                tmp_header.insert(key.clone(), value.clone());
            }
        }
        self
    }

    pub fn set_body(&mut self, body: &Body) -> &mut Self {
        if let Some(tmp_body) = Arc::get_mut(&mut self.tmp.body) {
            for (key, value) in body {
                tmp_body.insert(key.clone(), value.clone());
            }
        }
        self
    }

    pub fn builder(&mut self) -> HttpRequest {
        self.builder = self.tmp.clone();
        self.tmp = HttpRequest::default();
        self.builder.clone()
    }
}

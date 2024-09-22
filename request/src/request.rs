use super::{
    error::Error,
    r#type::{HttpRequest, HttpRequestBuilder, Methods, Protocol, HTTP_BR},
};
use global_type::r#type::{Body, Header};
use request_url::r#type::Url;
use std::io::{Read, Write};
use std::{collections::HashMap, net::TcpStream};

impl HttpRequest {
    fn get_protocol(&self) -> Protocol {
        self.protocol.clone()
    }

    fn get_methods(&self) -> Methods {
        self.methods.clone()
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }

    fn get_header(&self) -> Header {
        self.header.clone()
    }

    fn get_body(&self) -> Body {
        self.body.clone()
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
        let mut response: String = String::new();
        stream.read_to_string(&mut response).unwrap();
        response
    }

    pub fn send(&self) -> Result<String, Error> {
        if let Ok(url_obj) = self.parse_url() {
            let methods = self.get_methods();
            let host: String = url_obj.host.clone().unwrap_or_default();
            let port: u16 = url_obj.port.clone().unwrap_or_default();
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
            methods: Methods::new(),
            url: String::new(),
            protocol: Protocol::new(),
            header: HashMap::new(),
            body: HashMap::new(),
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
        self.tmp.protocol = protocol;
        self
    }

    pub fn new() -> Self {
        HttpRequestBuilder::default()
    }

    pub fn set_methods(&mut self, methods: Methods) -> &mut Self {
        self.tmp.methods = methods;
        self
    }

    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.tmp.url = url.to_owned();
        self
    }

    pub fn set_header(&mut self, header: &Header) -> &mut Self {
        for (key, value) in header {
            self.tmp.header.insert(key.clone(), value.clone());
        }
        self
    }

    pub fn set_body(&mut self, body: &Body) -> &mut Self {
        for (key, value) in body {
            self.tmp.body.insert(key.clone(), value.clone());
        }
        self
    }

    pub fn builder(&mut self) -> HttpRequest {
        self.builder = self.tmp.clone();
        self.tmp = HttpRequest::default();
        self.builder.clone()
    }
}

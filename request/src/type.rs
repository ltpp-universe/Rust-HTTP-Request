use global_type::r#type::{Body, Header};

pub(crate) static HTTP_BR: &str = "\r\n";

#[derive(Debug, Clone, PartialEq)]
pub enum Protocol {
    HTTP,
    HTTPS,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::HTTP
    }
}

impl Protocol {
    pub fn new() -> Self {
        Protocol::default()
    }

    pub fn value(&self) -> &str {
        match self {
            Protocol::HTTP => "http",
            Protocol::HTTPS => "https",
        }
    }

    pub fn is_http(&self) -> bool {
        self.value() == Protocol::HTTP.value()
    }

    pub fn is_https(&self) -> bool {
        self.value() == Protocol::HTTPS.value()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Methods {
    GET,
    POST,
}

impl Default for Methods {
    fn default() -> Self {
        Methods::GET
    }
}

impl Methods {
    pub fn new() -> Self {
        Methods::default()
    }

    pub fn value(&self) -> &str {
        match self {
            Methods::GET => "GET",
            Methods::POST => "POST",
        }
    }

    pub fn is_get(&self) -> bool {
        self.value() == Methods::GET.value()
    }

    pub fn is_post(&self) -> bool {
        self.value() == Methods::POST.value()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    pub methods: Methods,
    pub url: String,
    pub protocol: Protocol,
    pub header: Header,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequestBuilder {
    pub(super) tmp: HttpRequest,
    pub(super) builder: HttpRequest,
}

use crate::r#type::r#type::{Body, Header};

pub(crate) static HTTP_BR: &str = "\r\n";

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    pub methods: String,
    pub url: String,
    pub protocol: String,
    pub header: Header,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequestBuilder {
    pub(super) tmp: HttpRequest,
    pub(super) builder: HttpRequest,
}

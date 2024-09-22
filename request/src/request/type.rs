use crate::{methods::methods::Methods, protocol::protocol::Protocol};
use global_type::r#type::r#type::{Body, Header};

pub(crate) static HTTP_BR: &str = "\r\n";

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
    pub tmp: HttpRequest,
    pub builder: HttpRequest,
}

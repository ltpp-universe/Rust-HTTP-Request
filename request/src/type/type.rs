use std::sync::Arc;

use crate::{methods::methods::Methods, protocol::protocol::Protocol};
use global_type::r#type::r#type::{Body, Header};

pub(crate) static HTTP_BR: &str = "\r\n";
pub(crate) static HTTP_DOUBLE_BR: &str = "\r\n\r\n";

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    pub methods: Arc<Methods>,
    pub url: Arc<String>,
    pub protocol: Arc<Protocol>,
    pub header: Arc<Header>,
    pub body: Arc<Body>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequestBuilder {
    pub tmp: HttpRequest,
    pub builder: HttpRequest,
}

use super::r#type::HttpRequest;
use super::r#type::{HttpRequestBuilder, Methods, Protocol};

pub fn _request_builder() -> HttpRequest {
    use std::collections::HashMap;
    let http = HttpRequestBuilder::new()
        .set_methods(Methods::GET)
        .set_url("https://git.ltpp.vip/root/rust-http-request/-/tree/master/src?ref_type=heads")
        .set_protocol(Protocol::HTTP)
        .set_body(&HashMap::new())
        .set_header(&HashMap::new())
        .builder();
    http
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_builder() {
        let result: HttpRequest = _request_builder();
        if let Ok(response) = result.send() {
            assert_eq!(response, String::default());
        }
        assert_eq!(result, HttpRequest::default());
    }
}

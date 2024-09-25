use crate::methods::methods::Methods;
use crate::r#type::r#type::{HttpRequest, HttpRequestBuilder};

pub fn _request_builder() -> HttpRequest {
    use std::collections::HashMap;
    let http: HttpRequest = HttpRequestBuilder::new()
        .set_methods(Methods::GET)
        .set_url("https://git.ltpp.vip/root/rust-http-request/-/tree/master/src?ref_type=heads")
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
        let mut result: HttpRequest = _request_builder();
        if let Ok(response) = result.send() {
            assert_eq!(response, String::default());
        }
    }
}

use request::r#type::HttpRequest;

pub mod request;
pub mod response;
pub mod r#type;
mod url;
mod utils;

pub fn request_builder() -> HttpRequest {
    use request::r#type::HttpRequestBuilder;
    use std::collections::HashMap;
    let http = HttpRequestBuilder::new()
        .set_methods("".to_string())
        .set_url("".to_string())
        .set_protocol("".to_string())
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
        let result = request_builder();
        assert_eq!(result, HttpRequest::default());
    }
}

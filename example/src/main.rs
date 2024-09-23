use ::request::r#type::r#type::HttpRequest;
use request::{
    methods::methods::Methods, protocol::protocol::Protocol, r#type::r#type::HttpRequestBuilder,
};

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

fn main() {
    let req: HttpRequest = _request_builder();
    let res: Result<String, request::error::error::Error> = req.send();
    println!("{:?}", res);
}

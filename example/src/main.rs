use ::request::r#type::r#type::HttpRequest;
use request::{
    methods::methods::Methods, protocol::protocol::Protocol, r#type::r#type::HttpRequestBuilder,
};
use std::collections::HashMap;

pub fn _post_request_builder() -> HttpRequest {
    let mut body = HashMap::new();
    body.insert("code".to_owned(), "".to_owned());
    body.insert("language".to_owned(), "rust".to_owned());
    body.insert("testin".to_owned(), "".to_owned());
    let http = HttpRequestBuilder::new()
        .set_methods(Methods::POST)
        .set_url("https://code.ltpp.vip/")
        .set_protocol(Protocol::HTTP)
        .set_body(&body)
        .set_header(&HashMap::new())
        .builder();
    http
}

pub fn _get_request_builder() -> HttpRequest {
    let http = HttpRequestBuilder::new()
        .set_methods(Methods::GET)
        .set_url("https://git.ltpp.vip/root/rust-http-request/-/tree/master/src?ref_type=heads")
        .set_protocol(Protocol::HTTP)
        .builder();
    http
}

fn main() {
    let get_req: HttpRequest = _get_request_builder();
    let get_res: Result<String, request::error::error::Error> = get_req.send();
    println!("{:?}", get_res);
    let post_req: HttpRequest = _get_request_builder();
    let post_res: Result<String, request::error::error::Error> = post_req.send();
    println!("{:?}", post_res);
}

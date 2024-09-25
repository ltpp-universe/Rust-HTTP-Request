use ::request::r#type::r#type::HttpRequest;
use request::{methods::methods::Methods, r#type::r#type::HttpRequestBuilder};
use std::collections::HashMap;

pub fn _post_request_builder() -> HttpRequest {
    let mut body: HashMap<String, String> = HashMap::new();
    body.insert("code".to_owned(), "".to_owned());
    body.insert("language".to_owned(), "rust".to_owned());
    body.insert("testin".to_owned(), "".to_owned());
    let http: HttpRequest = HttpRequestBuilder::new()
        .set_methods(Methods::POST)
        .set_url("https://code.ltpp.vip/")
        .set_body(&body)
        .set_header(&HashMap::new())
        .builder();
    http
}

pub fn _get_request_builder() -> HttpRequest {
    let http: HttpRequest = HttpRequestBuilder::new()
        .set_methods(Methods::GET)
        .set_url("https://leetcode.cn/")
        .builder();
    http
}

fn main() {
    let mut get_req: HttpRequest = _get_request_builder();
    let get_res: Result<String, request::error::error::Error> = get_req.send();
    println!("{:?}", get_res);
    let mut post_req: HttpRequest = _post_request_builder();
    let post_res: Result<String, request::error::error::Error> = post_req.send();
    println!("{:?}", post_res);
}

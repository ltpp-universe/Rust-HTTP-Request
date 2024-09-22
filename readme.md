## Rust http request

> Rust http request

## Use

```rs
let http = HttpRequestBuilder::new()
    .set_methods(Methods::POST)
    .set_url("https://git.ltpp.vip/root/rust-http-request/-/tree/master/src?ref_type=heads")
    .set_protocol(Protocol::HTTP)
    .set_body(&HashMap::new())
    .set_header(&HashMap::new())
    .builder();
let result: HttpRequest = _request_builder();
if let Ok(response) = result.send() {
    prinln!("{:?}", response);
}
```

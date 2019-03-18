use http::{Request, Response, StatusCode, header};

 fn handler(request: Request<()>) -> http::Result<Response<String>> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html")
        .body("<!doctype html><html><head><title>A simple deployment with Now!</title></head><body><h1>Welcome to Rust on Now</h1></body></html>".to_string())
        .expect("failed to render response");

     Ok(response)
}

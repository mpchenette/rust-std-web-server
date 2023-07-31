pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub version: String,
}

pub fn is_valid_http_1_1(supposed_http_request: &str) -> bool {
    // https://datatracker.ietf.org/doc/html/rfc9112

    let mut request_items: std::str::SplitWhitespace = supposed_http_request.split_whitespace();
    

    while request_items.next() != None
    {

    }

    // Method - https://datatracker.ietf.org/doc/html/rfc9112#section-3.1
    // "The request method is case-sensitive."

    return false;
}

pub fn construct_http_request_from_vec_u8(buffer: Vec<u8>) -> HttpRequest {
    // Take the Vec<u8> and turn it into a &str
    let str_to_split: &str = match std::str::from_utf8(buffer.as_slice()) {
        Ok(str) => str,
        Err(e) => panic!("ERROR (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8): Invalid UTF-8: {}", e),
    };

    // Verify that the request &str contains valid HTTP. Panic if otherwise.
    // assert!(is_valid_http_1_1(str_to_split));

    // Split new &str on whitespace. Would prefer regex here for validation and ease of use purposes.
    let mut request_items: std::str::SplitWhitespace = str_to_split.split_whitespace();

    let request_method: String = match request_items.next() {
        Some(str) => str.to_string(),
        None => panic!("ERROR (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8): Unable to grab request method. Panicing!"),
    };
    let request_url: String = match request_items.next() {
        Some(str) => str.to_string(),
        None => panic!("ERROR (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8): Unable to grab request url. Panicing!"),
    };
    let request_version: String = match request_items.next() {
        Some(str) => str.to_string(),
        None => panic!("ERROR (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8): Unable to grab request version. Panicing!"),
    };


    // Construct HttpRequest
    let http_request: HttpRequest = HttpRequest {
        method: request_method,
        url: request_url,
        version: request_version,
    };
    println!(
        "LOG (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8):\n   HttpRequest Constructed:\n      method: {}\n      url: {}\n      version: {}",
        http_request.method, http_request.url, http_request.version
    );

    return http_request;
}
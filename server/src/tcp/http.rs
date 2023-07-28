pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub version: String,
}

// pub fn validate_http() {}

pub fn construct_http_request_from_vec_u8(buffer: Vec<u8>) -> HttpRequest {
    println!("\nconstruct_http_request_from_vec_u8\n-------------------------");

    // "Cast" typeless data into a &str
    let str_to_split: &str = match std::str::from_utf8(buffer.as_slice()) {
        Ok(str) => str,
        Err(e) => panic!("Invalid UTF-8: {}", e),
    };

    // Split new &str on whitespace. Would prefer regex here for validation and ease of use purposes.
    let mut request_items: std::str::SplitWhitespace = str_to_split.split_whitespace();

    // Construct HttpRequest
    let http_request: HttpRequest = HttpRequest {
        method: request_items.next().unwrap().to_string(),
        url: request_items.next().unwrap().to_string(),
        version: request_items.next().unwrap().to_string(),
    };
    println!(
        "returning:\n   method: {}\n   url: {}\n   version: {}\n",
        http_request.method, http_request.url, http_request.version
    );

    // Return HttpRequest
    http_request
}
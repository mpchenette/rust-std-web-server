// tcp/http/mod.rs

// TODO: "In practice, servers are implemented to only expect a request (a response is interpreted as an unknown or invalid request method)" - rfc9112#section-2.1

#[derive(Debug)]
pub enum HttpRequestError { BadRequest, UnsupportedMethod, UnsupportedVersion, InvalidHeader }

pub const HTTP_METHODS: [&[u8]; 9] = [ b"GET", b"HEAD", b"POST", b"PUT", b"DELETE", b"CONNECT", b"OPTIONS", b"TRACE", b"PATCH" ];
pub const SUPPORTED_HTTP_METHODS: [&[u8]; 5] = [
    HTTP_METHODS[0], // b"GET"
    HTTP_METHODS[1], // b"HEAD"
    HTTP_METHODS[2], // b"POST"
    HTTP_METHODS[3], // b"PUT"
    HTTP_METHODS[4], // b"DELETE"
];
pub const HTTP_VERSIONS: [[u8; 8]; 4] = [*b"HTTP/1.0", *b"HTTP/1.1", *b"HTTP/2.0", *b"HTTP/3.0"];
pub const SUPPORTED_HTTP_VERSIONS: [[u8; 8]; 1] = [*b"HTTP/1.1"];

// ----- START HttpMessage - rfc9112#section-2.1 -----

// Request - rfc9112#section-3
pub struct HttpRequestLine {
    pub method: Vec<u8>,
    pub request_target: Vec<u8>,
    pub http_version: Vec<u8>,
}

pub struct HttpRequest {
    pub start_line: HttpRequestLine,
    pub header_field_lines: std::collections::HashMap<Vec<u8>, Vec<u8>>, // "zero or more header field lines"
    pub body: Option<Vec<u8>>,                                           // "optional message body"
}

// Response - rfc9112#section-4
pub struct HttpStatusLine {
    pub http_version: Vec<u8>,
    pub status_code: Vec<u8>,
    pub reason_phrase: Vec<u8>,
}

pub struct HttpResponse {
    pub start_line: HttpStatusLine,
    pub header_field_lines: std::collections::HashMap<Vec<u8>, Vec<u8>>, // "zero or more header field lines"
    pub body: Option<Vec<u8>>,                                           // "optional message body"
}

// ----- END HttpMessage - rfc9112#section-2.1 -----

pub fn is_http_request_method(potential_http_request_method: &[u8]) -> bool {
    // "The method token is case-sensitive..." - rfc9110#section-9
    if HTTP_METHODS.contains(&potential_http_request_method) { return true }
    false
}

// Checks if this server supports the HTTP method passed to it.
pub fn is_supported_http_request_method(http_request_method: &[u8]) -> bool {
    if SUPPORTED_HTTP_METHODS.contains(&http_request_method) { return true }
    false
}

pub fn is_valid_http_request_uri(potential_http_uri: &[u8]) -> bool {
    // TODO: Improve the implementation here
    if potential_http_uri[0] == b'/' { return true }
    false
}

pub fn is_http_version(potential_http_version: &[u8]) -> bool {
    for http_version in HTTP_VERSIONS.iter() {
        if potential_http_version == http_version { return true }
    }
    false
}

pub fn is_supported_http_version(http_version: &[u8]) -> bool {
    for supported_http_version in SUPPORTED_HTTP_VERSIONS.iter() {
        if http_version == supported_http_version { return true }
    }
    false
}

// Construct a HttpRequest from a Vec<u8>. Will return an error if any parts of the passed Vec<u8> do not form a valid HTTP Request.
// TODO: Will also return an error if we are passed an HTTP response. (make sure this is true and add it to the test cases)

pub fn vec_u8_to_http_request(buffer: Vec<u8>) -> Result<HttpRequest, HttpRequestError> {

    // ----- REQUEST LINE -----
    let crlf_index: usize = match buffer.windows(2).position(|window| window == b"\r\n") {
        Some(crlf_index) => crlf_index,
        None => return Err(HttpRequestError::BadRequest) // No CRLF -> send 400
    };

    let request_line: &[u8] = &buffer[..crlf_index]; // we want this to be a & so we don't replicate the buffer into a new array
    let request_line_parts: Vec<&[u8]> = request_line.split(|&b| b == b' ').collect();
    if request_line_parts.len() != 3 { return Err(HttpRequestError::BadRequest) } // Malformed request line -> send 400

    // ----- method
    let method: Vec<u8> =
        if is_http_request_method(request_line_parts[0]) {
            if is_supported_http_request_method(request_line_parts[0]) { request_line_parts[0].to_vec() }
            else { return Err(HttpRequestError::UnsupportedMethod) } // Is an HTTP method, not a supported HTTP method -> send 501
        }
        else { return Err(HttpRequestError::BadRequest) }; // Not an HTTP method -> send 400

    // ----- request-target
    let request_target: Vec<u8> =
        if is_valid_http_request_uri(request_line_parts[1]) { request_line_parts[1].to_vec() }
        else { return Err(HttpRequestError::BadRequest) }; // Send 400 Bad Request

    // ----- HTTP-version
    let http_version: Vec<u8> =
        if is_http_version(request_line_parts[2]) {
            if is_supported_http_version(request_line_parts[2]) { request_line_parts[2].to_vec() }
            else { return Err(HttpRequestError::UnsupportedVersion) } // Is an HTTP version, not a supported HTTP version -> Send 505
        }
        else { return Err(HttpRequestError::BadRequest) }; // Not an HTTP version -> Send 400

    // Construct HttpRequestLine
    let http_request_line: HttpRequestLine = HttpRequestLine {
        method: method,
        request_target: request_target,
        http_version: http_version,
    };

    // ----- HEADER FIELDS -----
    // --- START FROM COPILOT
    let headers_start = crlf_index + 2; // Skip the CRLF after the request line
    let headers_end = match buffer[headers_start..].windows(4).position(|window| window == b"\r\n\r\n") {
        Some(pos) => headers_start + pos,
        None => return Err(HttpRequestError::BadRequest),
    };

    let headers_section = &buffer[headers_start..headers_end];
    let headers_lines: Vec<&[u8]> = headers_section.split(|&b| b == b'\r' || b == b'\n').filter(|line| !line.is_empty()).collect();

    let mut http_header_fields: std::collections::HashMap<Vec<u8>, Vec<u8>> = std::collections::HashMap::new();
    for line in headers_lines {
        if let Some(pos) = line.windows(2).position(|window| window == b": ") {
            let key = &line[..pos];
            let value = &line[pos + 2..];
            http_header_fields.insert(key.to_vec(), value.to_vec());
        }
        else { return Err(HttpRequestError::InvalidHeader) }
    }

    // Construct HttpRequest
    let http_request: HttpRequest = HttpRequest {
        start_line: http_request_line,
        header_field_lines: http_header_fields,
        body: Some(buffer[headers_end + 4..].to_vec()), // None,
    };
        // --- END FROM COPILOT


    //unused, just here to get rid of warnings
    let http_response: HttpResponse = construct_http_response(b"200".to_vec(), b"OK".to_vec());
    println!("LOG (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8):\n   HttpResponse Constructed:\n      version: {:?}\n      status_code: {:?}\n      reason_phrase: {:?}", http_response.start_line.http_version, http_response.start_line.status_code, http_response.start_line.reason_phrase);
    for (key, value) in http_response.header_field_lines.iter() {
        println!("      {:?}: {:?}", key, value);
    }
    println!("      body: {:?}", http_response.body);

    println!("LOG (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8):\n   HttpRequest Constructed:\n      method: {:?}\n      uri: {:?}\n      version: {:?}", http_request.start_line.method, http_request.start_line.request_target, http_request.start_line.http_version);
    for (key, value) in http_request.header_field_lines.iter() {
        println!("      {:?}: {:?}", key, value);
    }
    println!("      body: {:?}", http_request.body);
    Ok(http_request)
}

pub fn construct_http_response(status_code: Vec<u8>, reason_phrase: Vec<u8>) -> HttpResponse {
    let http_response: HttpResponse = HttpResponse {
        start_line: HttpStatusLine {
            http_version: b"HTTP/1.1".to_vec(),
            status_code: status_code,
            reason_phrase: reason_phrase,
        },
        header_field_lines: std::collections::HashMap::new(),
        body: None,
    };
    http_response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_methods() {
        //for each method in HTTP_METHODS, assert that it is a valid http method

        for method in HTTP_METHODS.iter() {
            assert!(is_http_request_method(method));
        }
        assert!(!is_http_request_method(b"NONE"));  // Not an HTTP method
        assert!(!is_http_request_method(b"get"));   // Lowercase
        assert!(!is_http_request_method(b"DELET")); // Incomplete
        assert!(!is_http_request_method(b""));      // Empty string
        assert!(!is_http_request_method(b" "));     // Single whitespace
        assert!(!is_http_request_method(b"   "));   // Multiple whitespaces
        assert!(!is_http_request_method(b"\r\n"));  // CRLF
        assert!(!is_http_request_method(b"\t"));    // Tab
        assert!(!is_http_request_method(b"GET "));  // Trailing whitespace
        assert!(!is_http_request_method(b" GET"));  // Leading whitespace
        assert!(!is_http_request_method(b"G@T"));   // Special character
        assert!(!is_http_request_method(b"123"));   // Numeric string
        assert!(!is_http_request_method(b"GeT"));   // Mixed case
        assert!(!is_http_request_method(b"GETPOST")); // Concatenated valid methods
        assert!(!is_http_request_method(&(b"A".repeat(100)))); // Excessively long string
    }

    #[test]
    fn test_vec_u8_to_http_message() {
        // TODO: test edge cases for vec_u8_to_http_message().
        // What if we are passed no headers?
        // What if we are passed no body?
        // What if we are passed no request line? etc.

        let http_request: HttpRequest = vec_u8_to_http_request(b"GET / HTTP/1.1\r\nHost: localhost:8000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n".to_vec()).unwrap();

        // Construct an HttpRequest to compare against the one returned from vec_u8_to_http_request
        let http_request_line: HttpRequestLine = HttpRequestLine {
            method: b"GET".to_vec(),
            request_target: b"/".to_vec(),
            http_version: b"HTTP/1.1".to_vec(),
        };

        let mut headers: std::collections::HashMap<Vec<u8>, Vec<u8>> = std::collections::HashMap::new();
        headers.insert(b"Host".to_vec(), b"localhost:8000".to_vec());
        headers.insert(b"User-Agent".to_vec(), b"curl/7.64.1".to_vec());
        headers.insert(b"Accept".to_vec(), b"*/*".to_vec());

        let http_request_to_compare: HttpRequest = HttpRequest {
            start_line: http_request_line,
            header_field_lines: headers,
            body: None,
        };

        // Compare the two HttpRequests
        assert_eq!(http_request.start_line.method,         http_request_to_compare.start_line.method);
        assert_eq!(http_request.start_line.request_target, http_request_to_compare.start_line.request_target);
        assert_eq!(http_request.start_line.http_version,   http_request_to_compare.start_line.http_version);

        for (key, value) in http_request.header_field_lines.iter() {
            assert_eq!(http_request_to_compare.header_field_lines.contains_key(key), true);
            assert_eq!(http_request_to_compare.header_field_lines.get(key), Some(value));
        }
        for (key, value) in http_request_to_compare.header_field_lines.iter() {
            assert_eq!(http_request.header_field_lines.contains_key(key), true);
            assert_eq!(http_request.header_field_lines.get(key), Some(value));
        }
    }
}

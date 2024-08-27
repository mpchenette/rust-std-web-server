// tcp/http/mod.rs

// TODO: "In practice, servers are implemented to only expect a request (a response is interpreted as an unknown or invalid request method)" - rfc9112#section-2.1

mod error;

pub const HTTP_METHODS: [&[u8]; 9] = [b"GET", b"HEAD", b"POST", b"PUT", b"DELETE", b"CONNECT", b"OPTIONS", b"TRACE", b"PATCH"];
pub const SUPPORTED_HTTP_METHODS: [&[u8]; 5] = [b"GET", b"HEAD", b"POST", b"PUT", b"DELETE"]; // "All general-purpose servers MUST support the methods GET and HEAD. All other methods are OPTIONAL." - rfc9110#section-9
pub const HTTP_VERSIONS: [&[u8]; 4] = [b"HTTP/1.0", b"HTTP/1.1", b"HTTP/2.0", b"HTTP/3.0"];
pub const SUPPORTED_HTTP_VERSIONS: [&[u8]; 1] = [b"HTTP/1.1"];

// ----- START HttpMessage - rfc9112#section-2.1 -----

pub enum StartLine {
    RequestLine(HttpRequestLine),
    StatusLine(HttpStatusLine),
}

// Request - rfc9112#section-3
pub struct HttpRequestLine {
    pub method: Vec<u8>,
    pub request_target: Vec<u8>,
    pub http_version: Vec<u8>,
}

// Response - rfc9112#section-4
pub struct HttpStatusLine {
    pub http_version: Vec<u8>,
    pub status_code: Vec<u8>,
    pub reason_phrase: Vec<u8>,
}

pub struct HttpMessage {
    pub start_line: StartLine,
    pub header_field_lines: std::collections::HashMap<Vec<u8>, Vec<u8>>, // "zero or more header field lines"
    pub body: Option<Vec<u8>>, // "optional message body"
}

// ----- END HttpMessage - rfc9112#section-2.1 -----

pub fn is_http_request_method(potential_http_request_method: &[u8]) -> bool {
    // "The method token is case-sensitive..." - rfc9110#section-9
    if HTTP_METHODS.contains(&potential_http_request_method) {
        return true;
    }
    false
}

// Checks if this server supports the HTTP method passed to it.
pub fn is_supported_http_request_method(http_request_method: &[u8]) -> bool {
    if SUPPORTED_HTTP_METHODS.contains(&http_request_method) {
        return true;
    }
    false
}

pub fn is_valid_http_request_uri(potential_http_uri: &[u8]) -> bool {
    // // TODO: Improve the implementation here
    // if potential_http_uri.chars().next() == Some('/') {
    //     return true;
    // }
    // false
    true
}

pub fn is_valid_http_version(unvalidated_http_version: &[u8]) -> bool {
    // "HTTP-version is case-sensitive." - rfc9112#section-2.3
    if SUPPORTED_HTTP_VERSIONS.contains(&unvalidated_http_version) {
        return true;
    }
    false
}

// Construct a HttpMessage from a Vec<u8>. Will return an error if any parts of the passed Vec<u8> do not form a valid HTTP Request.
// TODO: Will also return an error if we are passed an HTTP response. (make sure this is true and add it to the test cases)

pub fn vec_u8_to_http_message(buffer: Vec<u8>) -> Result<HttpMessage, error::HttpRequestError> {

    // ----- REQUEST LINE -----

    let crlf_index = match buffer.windows(2).position(|window| window == b"\r\n"){
        Some(crlf_index) => crlf_index,
        None => {
            // Send 400 Bad Request
            // As far as error messages to send to the client,
            // We would need to understand if the buffer was empty or if
            // There were contents in the buffer but there was no CRLF
            return Err(error::HttpRequestError::InvalidRequest);
        }
    };

    let request_line = &buffer[..crlf_index];
    let request_line_parts: Vec<&[u8]> = request_line.split(|&b| b == b' ').collect();
    if request_line_parts.len() != 3 {
        // Send 400 Bad Request
        return Err(error::HttpRequestError::InvalidRequestLine);
    }
    
    let method: Vec<u8> = if is_http_request_method(request_line_parts[0]) { request_line_parts[0].to_vec() } else {
        // Send 501 Not Implemented? In reality this is just not an HTTP method that we've received. What is the status code for that?
        return Err(error::HttpRequestError::InvalidMethod);//???? ^^^
    };
    let uri = request_line_parts[1].to_vec();
    let version = request_line_parts[2].to_vec();

    // Construct HttpRequestLine
    let http_request_line: HttpRequestLine = HttpRequestLine {
        method: method,
        request_target: uri,
        http_version: version,
    };


    // ----- HEADER FIELDS -----

    // // Construct HttpRequestHeaderFields
    // let mut http_header_fields: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    // // FROM COPILOT ---
    // for line in request_lines.by_ref() {
    //     if line.is_empty() {
    //         break;
    //     }
    //     if let Some((key, value)) = line.split_once(": ") {
    //         http_header_fields
    //             .insert(key.to_string(), value.to_string());
    //     }
    // }
    // // The remaining part is the body
    // let body: Vec<u8> = request_lines.collect::<Vec<&str>>().join("\n").into_bytes();

    // --- END FROM COPILOT

    // Construct HttpRequest
    let http_request: HttpMessage = HttpMessage {
        start_line: StartLine::RequestLine(http_request_line),
        header_field_lines: std::collections::HashMap::new(), //http_header_fields,
        body: None,
    };

    //unused, just here to get rid of warnings
    let http_response: HttpMessage = construct_http_response(Vec::new());
    // println!("LOG (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8):\n   HttpResponse Constructed:\n      version: {}\n      status_code: {}\n      reason_phrase: {}", http_response.status_line.version, http_response.status_line.status_code, http_response.status_line.reason_phrase);
    // for (key, value) in http_response.header_field_lines.iter() {
    //     println!("      {:?}: {:?}", key, value);
    // }
    // println!("      body: {:?}", http_response.body);

    // println!("LOG (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8):\n   HttpRequest Constructed:\n      method: {}\n      uri: {}\n      version: {}", http_request.http_request_line.http_method, http_request.http_request_line.uri, http_request.http_request_line.http_version);
    // for (key, value) in http_request.header_field_lines.iter() {
    //     println!("      {:?}: {:?}", key, value);
    // }
    println!("      body: {:?}", http_request.body);
    Ok(http_request)
}

pub fn construct_http_response(buffer: Vec<u8>) -> HttpMessage {
    let http_response: HttpMessage = HttpMessage {
        start_line: StartLine::StatusLine(HttpStatusLine {
            http_version: b"HTTP/1.1".to_vec(),
            status_code: b"200".to_vec(),
            reason_phrase: b"OK".to_vec(),
        }),
        header_field_lines: std::collections::HashMap::new(),
        body: None,
    };
    http_response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_http_request_methods() {
        //for each method in SUPPORTED_HTTP_METHODS, assert that it is a valid http method

        for method in SUPPORTED_HTTP_METHODS.iter() {
            // TODO: should I be putting prints in my unit tests?
            println!("method: {:?}", method);
            assert!(is_http_request_method(method));
        }
        assert!(!is_http_request_method(b"NONE")); // Not an HTTP method
        assert!(!is_http_request_method(b"get")); // lowercase
        assert!(!is_http_request_method(b"DELET")); // incomplete

        // Additional test cases (from Copilot)
        assert!(!is_http_request_method(b"")); // Empty string
        assert!(!is_http_request_method(b" ")); // Single whitespace
        assert!(!is_http_request_method(b"   ")); // Multiple whitespaces
        assert!(!is_http_request_method(b"GET ")); // Trailing whitespace
        assert!(!is_http_request_method(b" GET")); // Leading whitespace
        assert!(!is_http_request_method(b"G@T")); // Special character
        assert!(!is_http_request_method(b"123")); // Numeric string
        assert!(!is_http_request_method(b"GeT")); // Mixed case
        assert!(!is_http_request_method(b"GETPOST")); // Concatenated valid methods
        assert!(!is_http_request_method(&(b"A".repeat(100)))); // Excessively long string
    }

    #[test]
    fn test_vec_u8_to_http_message() {
        // TODO: test edge cases for vec_u8_to_http_message().
        // What if we are passed no headers?
        // What if we are passed no body?
        // What if we are passed no request line? etc.




        // generate a vec<u8> of a valid http request
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(b"GET / HTTP/1.1\r\n");
        buffer.extend_from_slice(b"Host: localhost:8000\r\n");
        buffer.extend_from_slice(b"User-Agent: curl/7.64.1\r\n");
        buffer.extend_from_slice(b"Accept: */*\r\n");
        buffer.extend_from_slice(b"\r\n");

        let http_request: HttpMessage = vec_u8_to_http_message(buffer).unwrap();

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

        let http_request_to_compare: HttpMessage = HttpMessage {
            start_line: StartLine::RequestLine(http_request_line),
            header_field_lines: headers,
            body: None,
        };

        // Compare the two HttpRequests

        //////// vvvvvv should be the same as the uncommented code below
        // if let StartLine::RequestLine(ref start_line) = http_request.start_line {
        //     if let StartLine::RequestLine(ref compare_line) = http_request_to_compare.start_line {
        //         assert_eq!(start_line.method, compare_line.method);
        //         assert_eq!(start_line.request_target, compare_line.request_target);
        //         assert_eq!(start_line.http_version, compare_line.http_version);
        //     }
        // }

        match (&http_request.start_line, &http_request_to_compare.start_line) {
            (StartLine::RequestLine(start_line), StartLine::RequestLine(compare_line)) => {
                assert_eq!(start_line.method, compare_line.method);
                assert_eq!(start_line.request_target, compare_line.request_target);
                assert_eq!(start_line.http_version, compare_line.http_version);
            },
            _ => {
                // Handle other cases if necessary
            }
        }
        // assert_eq!(http_request.http_request_line.method, http_request_to_compare.http_request_line.method);
        // assert_eq!(http_request.http_request_line.request_target, http_request_to_compare.http_request_line.request_target);
        // assert_eq!(http_request.http_request_line.http_version, http_request_to_compare.http_request_line.http_version);
        
        for (key, value) in http_request.header_field_lines.iter() {
            assert_eq!(http_request_to_compare.header_field_lines.contains_key(key), true);
            assert_eq!(http_request_to_compare.header_field_lines.get(key), Some(value));
        }
        for (key, value) in http_request_to_compare.header_field_lines.iter() {
            assert_eq!(http_request.header_field_lines.contains_key(key), true);
            assert_eq!(http_request.header_field_lines.get(key), Some(value));
        }

        // // Additional test cases (from Copilot)
        // // Test case: empty buffer
        // let buffer: Vec<u8> = Vec::new();
        // let result: Result<HttpRequest, error::HttpRequestError> = vec_u8_to_http_request(buffer);
        // assert!(result.is_err());
        // // Test case: missing request line
        // let mut buffer: Vec<u8> = Vec::new();
        // buffer.extend_from_slice(b"\r\n");
        // let result: Result<HttpRequest, error::HttpRequestError> = vec_u8_to_http_request(buffer);
        // assert!(result.is_err());
        // // Test case: missing request method
        // let mut buffer: Vec<u8> = Vec::new();
        // buffer.extend_from_slice(b"/ HTTP/1.1\r\n");
        // buffer.extend_from_slice(b"\r\n");
        // let result: Result<HttpRequest, error::HttpRequestError> = vec_u8_to_http_request(buffer);
        // assert!(result.is_err());
    }
}

// tcp/http/mod.rs

// TODO: "In practice, servers are implemented to only expect a request (a response is interpreted as an unknown or invalid request method)" - rfc9112#section-2.1

mod error;

pub const HTTP_METHODS: [&str; 9] = ["GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH"];
pub const SUPPORTED_HTTP_METHODS: [&str; 5] = ["GET", "HEAD", "POST", "PUT", "DELETE"]; // "All general-purpose servers MUST support the methods GET and HEAD. All other methods are OPTIONAL." - rfc9110#section-9
pub const HTTP_VERSIONS: [&str; 4] = ["HTTP/1.0", "HTTP/1.1", "HTTP/2.0", "HTTP/3.0"];
pub const SUPPORTED_HTTP_VERSIONS: [&str; 1] = ["HTTP/1.1"];

// ----- START HttpMessage - rfc9112#section-2.1 -----

enum StartLine {
    RequestLine(HttpRequestLine),
    StatusLine(HttpStatusLine),
}

// Request - rfc9112#section-3
pub struct HttpRequestLine {
    pub method: String,
    pub request_target: String,
    pub http_version: String,
}

// Response - rfc9112#section-4
pub struct HttpStatusLine {
    pub http_version: String,
    pub status_code: String,
    pub reason_phrase: String,
}

pub struct HttpMessage {
    pub start_line: StartLine,
    pub header_field_lines: std::collections::HashMap<String, String>, // "zero or more header field lines"
    pub body: Option<Vec<u8>>, // "optional message body"
}

// ----- END HttpMessage - rfc9112#section-2.1 -----

pub fn is_http_request_method(potential_http_request_method: &str) -> bool {
    // "The method token is case-sensitive..." - rfc9110#section-9
    if HTTP_METHODS.contains(&potential_http_request_method) {
        return true;
    }
    false
}

// Checks if this server supports the HTTP method passed to it.
pub fn is_supported_http_request_method(http_request_method: &str) -> bool {
    if SUPPORTED_HTTP_METHODS.contains(&http_request_method) {
        return true;
    }
    false
}

pub fn is_valid_http_request_uri(potential_http_uri: &str) -> bool {
    // TODO: Improve the implementation here
    if potential_http_uri.chars().next() == Some('/') {
        return true;
    }
    false
}

pub fn is_valid_http_version(unvalidated_http_version: &str) -> bool {
    // "HTTP-version is case-sensitive." - rfc9112#section-2.3
    if SUPPORTED_HTTP_VERSIONS.contains(&unvalidated_http_version) {
        return true;
    }
    false
}

// Construct a HttpMessage from a Vec<u8>. Will return an error if any parts of the passed Vec<u8> do not form a valid HTTP Request.
// TODO: Will also return an error if we are passed an HTTP response. (make sure this is true and add it to the test cases)

pub fn vec_u8_to_http_message(buffer: Vec<u8>) -> Result<HttpMessage, error::HttpRequestError> {
    
    // //OPTION 1
    // let mut request_method: String = String::new();
    // for byte in buffer{
    //     if byte == b' ' {
    //         break;
    //     }
    //     request_method.push(byte as char);
    // }
    // is_http_request_method(request_method.as_str());
    // is_supported_http_request_method(request_method.as_str());
    
    
    // OPTION 2
    // Take the Vec<u8> and turn it into a &str
    let str_to_split: &str = std::str::from_utf8(buffer.as_slice()).unwrap();

    // Split new &str on whitespace. Would prefer regex here for validation and ease of use purposes.
    let mut request_lines: std::str::Lines = str_to_split.lines();

    let request_line: &str = match request_lines.next() {
        Some(line) => line,
        None => {
            // TODO: What should we be doing here? Should we be returning an error? Should we be panicking? Should we be returning a Result?
            // TODO: also should this be None? Are there other options here? What does Some mean? What does None mean?
            eprintln!("Error: No request line found");
            panic!("no request line found");
            // return an HttpRequestError;
        }
    };

    let mut request_line_parts: std::str::SplitWhitespace = request_line.split_whitespace();

    // ----- REQUEST LINE -----
    let request_method: String = match request_line_parts.next() {
        Some(http_method) if is_valid_http_request_method(http_method) => http_method.to_string(),
        _ => {
            panic!("")
            // "An origin server that receives a request method that is unrecognized or not implemented SHOULD respond with the 501 (Not Implemented) status code." - rfc9110#section-9
            // TODO: don't panic, send 501 Not Implemented response and return Error
        }
    };

    let request_uri: String = match request_line_parts.next() {
        Some(uri) if is_valid_http_request_uri(uri) => uri.to_string(),
        _ => {
            panic!("")
            // TODO: don't panic, send ??? response and return Error
        }
    };

    let request_version: String = match request_line_parts.next() {
        Some(http_version) if is_valid_http_request_version(http_version) => {
            http_version.to_string()
        }
        _ => {
            panic!("")
            // TODO: don't panic, send ??? response and return Error
        }
    };

    // Construct HttpRequestLine
    let http_request_line: HttpRequestLine = HttpRequestLine {
        http_method: request_method,
        uri: request_uri,
        http_version: request_version,
    };

    // ----- HEADER FIELDS -----

    // Construct HttpRequestHeaderFields
    let mut http_header_fields: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    // FROM COPILOT ---
    for line in request_lines.by_ref() {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            http_header_fields
                .insert(key.to_string(), value.to_string());
        }
    }
    // The remaining part is the body
    let body: Vec<u8> = request_lines.collect::<Vec<&str>>().join("\n").into_bytes();

    // --- END FROM COPILOT

    // Construct HttpRequest
    let http_request: HttpRequest = HttpRequest {
        http_request_line: http_request_line,
        http_header_fields: http_header_fields,
        body: body,
    };

    //unused, just here to get rid of warnings
    let http_response: HttpResponse = construct_http_response(Vec::new());
    println!("LOG (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8):\n   HttpResponse Constructed:\n      version: {}\n      status_code: {}\n      reason_phrase: {}", http_response.status_line.version, http_response.status_line.status_code, http_response.status_line.reason_phrase);
    for (key, value) in http_response.header_fields.iter() {
        println!("      {}: {}", key, value);
    }
    println!("      body: {:?}", http_response.body);

    println!("LOG (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8):\n   HttpRequest Constructed:\n      method: {}\n      uri: {}\n      version: {}", http_request.http_request_line.http_method, http_request.http_request_line.uri, http_request.http_request_line.http_version);
    for (key, value) in http_request.http_header_fields.iter() {
        println!("      {}: {}", key, value);
    }
    println!("      body: {:?}", http_request.body);
    Ok(http_request)
}

pub fn construct_http_response(buffer: Vec<u8>) -> HttpResponse {
    let http_response: HttpResponse = HttpResponse {
        status_line: HttpResponseStatusLine {
            version: "HTTP/1.1".to_string(),
            status_code: "200".to_string(),
            reason_phrase: "OK".to_string(),
        },
        header_fields: std::collections::HashMap::new(),
        body: buffer,
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
            println!("method: {}", method);
            assert!(is_valid_http_request_method(method));
        }
        assert!(!is_valid_http_request_method("NONE")); // Not an HTTP method
        assert!(!is_valid_http_request_method("get")); // lowercase
        assert!(!is_valid_http_request_method("DELET")); // incomplete

        // Additional test cases (from Copilot)
        assert!(!is_valid_http_request_method("")); // Empty string
        assert!(!is_valid_http_request_method(" ")); // Single whitespace
        assert!(!is_valid_http_request_method("   ")); // Multiple whitespaces
        assert!(!is_valid_http_request_method("GET ")); // Trailing whitespace
        assert!(!is_valid_http_request_method(" GET")); // Leading whitespace
        assert!(!is_valid_http_request_method("G@T")); // Special character
        assert!(!is_valid_http_request_method("123")); // Numeric string
        assert!(!is_valid_http_request_method("GeT")); // Mixed case
        assert!(!is_valid_http_request_method("GETPOST")); // Concatenated valid methods
        assert!(!is_valid_http_request_method(&("A".repeat(100)))); // Excessively long string
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

        let http_request: HttpRequest = vec_u8_to_http_request(buffer).unwrap();

        // Construct an HttpRequest to compare against the one returned from vec_u8_to_http_request
        let http_request_line: HttpRequestLine = HttpRequestLine {
            http_method: "GET".to_string(),
            uri: "/".to_string(),
            http_version: "HTTP/1.1".to_string(),
        };

        let mut headers: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        headers.insert("Host".to_string(), "localhost:8000".to_string());
        headers.insert("User-Agent".to_string(), "curl/7.64.1".to_string());
        headers.insert("Accept".to_string(), "*/*".to_string());

        let http_request_to_compare: HttpRequest = HttpRequest {
            http_request_line: http_request_line,
            http_header_fields: headers,
            body: Vec::new(),
        };

        // Compare the two HttpRequests
        assert_eq!(http_request.http_request_line.http_method, http_request_to_compare.http_request_line.http_method);
        assert_eq!(http_request.http_request_line.uri, http_request_to_compare.http_request_line.uri);
        assert_eq!(http_request.http_request_line.http_version, http_request_to_compare.http_request_line.http_version);
        
        for (key, value) in http_request.http_header_fields.iter() {
            assert_eq!(http_request_to_compare.http_header_fields.contains_key(key), true);
            assert_eq!(http_request_to_compare.http_header_fields.get(key), Some(value));
        }
        for (key, value) in http_request_to_compare.http_header_fields.iter() {
            assert_eq!(http_request.http_header_fields.contains_key(key), true);
            assert_eq!(http_request.http_header_fields.get(key), Some(value));
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

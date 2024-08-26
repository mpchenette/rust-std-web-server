// tcp/http/mod.rs

// HttpRequest

mod error;

pub const SUPPORTED_HTTP_METHODS: [&str; 5] = ["GET", "HEAD", "POST", "PUT", "DELETE"]; // "All general-purpose servers MUST support the methods GET and HEAD. All other methods are OPTIONAL." - rfc9110#section-9
pub const SUPPORTED_HTTP_VERSIONS: [&str; 1] = ["HTTP/1.1"];

// https://datatracker.ietf.org/doc/html/rfc9112#section-3
pub struct HttpRequestLine {
    pub http_method: String,
    pub uri: String,
    pub http_version: String,
}

pub struct HttpRequestHeaderFields {
    pub headers: std::collections::HashMap<String, String>,
}
pub struct HttpRequest {
    pub http_request_line: HttpRequestLine,
    pub http_header_fields: HttpRequestHeaderFields,
    pub body: Vec<u8>, //TODO: Why is this Vec<u8>? What is the reasoning?
}

// HttpResponse
pub struct HttpResponseStatusLine {
    pub version: String,
    pub status_code: String,
    pub reason_phrase: String,
}

pub struct HttpResponseHeaderFields {
    pub headers: std::collections::HashMap<String, String>,
}

pub struct HttpResponse {
    pub status_line: HttpResponseStatusLine,
    pub header_fields: HttpResponseHeaderFields,
    pub body: Vec<u8>,
}

pub fn is_valid_http_request_method(potential_http_method: &str) -> bool {
    // "The method token is case-sensitive..." - rfc9110#section-9
    // TODO: CHECK IF THIS IS CASE SENSITIVE. IT NEEDS TO BE.
    if SUPPORTED_HTTP_METHODS.contains(&potential_http_method) {
        return true;
    }
    false
}

// TODO: Change return String to &str? Would this make sense?
pub fn is_valid_http_request_uri(potential_http_uri: &str) -> bool {
    println!("LOG (IS_VALID_HTTP_REQUEST_URI): potential_http_uri: {}", potential_http_uri);
    true
}

pub fn is_valid_http_request_version(potential_http_version: &str) -> bool {
    // "HTTP-version is case-sensitive." - rfc9112#section-2.3
    // TODO: CHECK IF THIS IS CASE SENSITIVE. IT NEEDS TO BE.
    if SUPPORTED_HTTP_VERSIONS.contains(&potential_http_version) {
        return true;
    }
    false
}

// Construct a HttpRequest from a Vec<u8>. Will return an error if any parts of the passed Vec<u8> request are invalid HTTP
pub fn vec_u8_to_http_request(buffer: Vec<u8>) -> Result<HttpRequest, error::HttpRequestError> {
    // Take the Vec<u8> and turn it into a &str
    let str_to_split: &str = std::str::from_utf8(buffer.as_slice()).unwrap();

    // Split new &str on whitespace. Would prefer regex here for validation and ease of use purposes.
    let mut request_lines: std::str::Lines = str_to_split.lines();

    let request_line: &str = request_lines.next().unwrap();
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
    let mut http_header_fields: HttpRequestHeaderFields = HttpRequestHeaderFields {
        headers: std::collections::HashMap::new(),
    };
    // FROM COPILOT ---
    for line in request_lines.by_ref() {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            http_header_fields
                .headers
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
    let http_response: HttpResponse = construct_http_response(buffer);
    println!("LOG (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8):\n   HttpResponse Constructed:\n      version: {}\n      status_code: {}\n      reason_phrase: {}", http_response.status_line.version, http_response.status_line.status_code, http_response.status_line.reason_phrase);
    for (key, value) in http_response.header_fields.headers.iter() {
        println!("      {}: {}", key, value);
    }
    println!("      body: {:?}", http_response.body);



    println!("LOG (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8):\n   HttpRequest Constructed:\n      method: {}\n      uri: {}\n      version: {}", http_request.http_request_line.http_method, http_request.http_request_line.uri, http_request.http_request_line.http_version);
    for (key, value) in http_request.http_header_fields.headers.iter() {
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
        header_fields: HttpResponseHeaderFields {
            headers: std::collections::HashMap::new(),
        },
        body: buffer,
    };
    http_response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
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
        assert!(!is_valid_http_request_method("GETPOSTDELETE")); // Multiple concatenated valid methods
        assert!(!is_valid_http_request_method("GETPOSTDELETEPUTHEAD")); // All valid methods concatenated
        assert!(!is_valid_http_request_method(&("A".repeat(100)))); // Excessively long string
        
    }
}

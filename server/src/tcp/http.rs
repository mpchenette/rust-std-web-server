// HttpRequest

// TODO: Should I be using const or static here? and why?
pub const SUPPORTED_HTTP_METHODS: [&str; 5] = ["GET", "HEAD", "POST", "PUT", "DELETE"];
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

// Rename to just 'request' since we are already in the http file? Whatever approach we use, we will need to standardize across all files.
// Answer: No. I prefer in the functions to have the vec_u8_to_http_request return an HttpRequest, for example
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
    pub version: String,
    pub status_code: String,
    pub reason_phrase: String,
}

pub struct HttpResponse {
    pub status_line: HttpResponseStatusLine,
    pub header_fields: HttpResponseHeaderFields,
    pub body: Vec<u8>,
}

// TODO: Change return String to &str? Would this make sense?
pub fn validate_http_request_method(potential_http_method: &str) -> String {
    // https://datatracker.ietf.org/doc/html/rfc9110#section-9
    // "The method token is case-sensitive..."
    // "All general-purpose servers MUST support the methods GET and HEAD. All other methods are OPTIONAL."
    // "An origin server that receives a request method that is unrecognized or not implemented SHOULD respond with the 501 (Not Implemented) status code."

    // TODO: CHECK IF THIS IS CASE SENSITIVE. IT NEEDS TO BE.
    if !SUPPORTED_HTTP_METHODS.contains(&potential_http_method) {
        // TODO: construct and return a 501 response
    }

    // Return the validated http method
    potential_http_method.to_string()
}

// TODO: Change return String to &str? Would this make sense?
pub fn validate_http_request_uri(potential_http_uri: &str) -> String {
    potential_http_uri.to_string()
}

// TODO: Change return String to &str? Would this make sense?
pub fn validate_http_request_version(potential_http_version: &str) -> String {
    // https://datatracker.ietf.org/doc/html/rfc9112#section-2.3
    // "HTTP-version is case-sensitive."

    // TODO: CHECK IF THIS IS CASE SENSITIVE. IT NEEDS TO BE.
    if !SUPPORTED_HTTP_VERSIONS.contains(&potential_http_version) {
        // TODO: construct and return a 501 response
    }

    // Return the validated http method
    potential_http_version.to_string()
}

pub fn vec_u8_to_http_request(buffer: Vec<u8>) -> HttpRequest {
    // Take the Vec<u8> and turn it into a &str
    let str_to_split: &str = std::str::from_utf8(buffer.as_slice()).unwrap();

    // Split new &str on whitespace. Would prefer regex here for validation and ease of use purposes.
    let mut request_lines: std::str::Lines = str_to_split.lines();

    let request_line: &str = request_lines.next().unwrap();
    let mut request_line_parts: std::str::SplitWhitespace = request_line.split_whitespace();

    let request_method: String = validate_http_request_method(request_line_parts.next().unwrap());
    let request_uri: String = validate_http_request_uri(request_line_parts.next().unwrap());
    let request_version: String = validate_http_request_version(request_line_parts.next().unwrap());

    // Construct HttpRequestLine
    let http_request_line: HttpRequestLine = HttpRequestLine {
        method: request_method.clone(), //TODO: Is clone needed here? It does not look like we use these values after this
        uri: request_uri.clone(),
        version: request_version.clone(),
    };

    // Construct HttpRequestHeaderFields
    let mut header_fields: HttpRequestHeaderFields = HttpRequestHeaderFields {
        headers: std::collections::HashMap::new(),
    };
    // FROM COPILOT ---
    for line in request_lines.by_ref() {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            header_fields.headers.insert(key.to_string(), value.to_string());
        }
    }
    // The remaining part is the body
    let body: Vec<u8> = request_lines.collect::<Vec<&str>>().join("\n").into_bytes();

    // --- END FROM COPILOT

    // Construct HttpRequest
    let http_request: HttpRequest = HttpRequest {
        request_line: http_request_line,
        header_fields: header_fields,
        body: body,
    };
    println!("LOG (CONSTRUCT_HTTP_REQUEST_FROM_VEC_U8):\n   HttpRequest Constructed:\n      method: {}\n      uri: {}\n      version: {}", http_request.request_line.method, http_request.request_line.uri, http_request.request_line.version);
    http_request
}

pub fn construct_http_response(buffer: Vec<u8>) -> HttpResponse {
    let http_response: HttpResponse = HttpResponse {
        status_line: HttpResponseStatusLine {
            version: "HTTP/1.1".to_string(),
            status_code: "200".to_string(),
            reason_phrase: "OK".to_string(),
        },
        header_fields: HttpResponseHeaderFields {
            version: "HTTP/1.1".to_string(),
            status_code: "200".to_string(),
            reason_phrase: "OK".to_string(),
        },
        body: buffer,
    };
    http_response
}

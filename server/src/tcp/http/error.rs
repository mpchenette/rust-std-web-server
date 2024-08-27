#[derive(Debug)] //
pub enum HttpRequestError {
    InvalidRequest,
    InvalidRequestLine,
    InvalidMethod,
    UnsupportedMethod,
    // InvalidUri,
    InvalidVersion,
    UnsupportedVersion,
    // Utf8Error,
    // EmptyRequest,
}
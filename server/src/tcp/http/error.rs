#[derive(Debug)] //
pub enum HttpRequestError {
    InvalidRequest,
    InvalidRequestLine,
    InvalidMethod,
    UnsupportedMethod,
    // InvalidUri,
    InvalidVersion,
    UnsupportedVersion,
    InvalidHeader
    // Utf8Error,
    // EmptyRequest,
}
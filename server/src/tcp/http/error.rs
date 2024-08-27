#[derive(Debug)] //
pub enum HttpRequestError {
    InvalidRequest,
    InvalidRequestLine,
    InvalidMethod,
    // InvalidUri,
    // InvalidVersion,
    // Utf8Error,
    // EmptyRequest,
}
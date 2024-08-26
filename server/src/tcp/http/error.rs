#[derive(Debug)] //
pub enum HttpRequestError {
    InvalidMethod,
    InvalidUri,
    InvalidVersion,
    Utf8Error,
    EmptyRequest,
}
use std::collections::HashMap;

use http::Request;

#[derive(Debug, Default)]
pub struct AppState {
    /// All recorded requests and their responses
    pub requests: HashMap<usize, RecordedRequest>,
    pub log_messages: Vec<String>,
}

#[derive(Debug)]
pub struct RecordedRequest {
    /// Contains all the information about the request
    pub request: Request<String>,
    /// Contains all the information abotu the response to this request. If the response hasn't been received yet it'll be None.
    pub response: Option<Request<String>>,
}

use anyhow::Result;
use http::{Request, Response};

pub async fn proxy_request(request: &Request<String>) -> Result<Response<String>> {
    anyhow::bail!("Not implemented");
}

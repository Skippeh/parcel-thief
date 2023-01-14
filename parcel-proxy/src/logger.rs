use anyhow::Result;
use http::{Request, Response};

pub async fn log_request_and_response(
    request: &Request<String>,
    response: &Response<String>,
) -> Result<()> {
    println!("{:#?}\n{:#?}", request, response);
    Ok(())
}

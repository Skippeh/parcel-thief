use std::net::ToSocketAddrs;

use anyhow::{Context, Result};
use http::{header::HeaderName, HeaderValue, Request, Response, StatusCode, Version};
use httparse::EMPTY_HEADER;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_native_tls::TlsConnector;

use crate::http_utility::{read_headers, ToHttp};

const REMOTE_SERVER: &str = "prod-pc-15.wws-gs2.com";

pub async fn proxy_request(request: &Request<String>) -> Result<Response<String>> {
    let addr = format!("{}:443", REMOTE_SERVER)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| anyhow::format_err!("failed to resolve {}", REMOTE_SERVER))?;

    let socket = TcpStream::connect(&addr).await?;
    let connector = tokio_native_tls::native_tls::TlsConnector::builder().build()?;
    let connector = TlsConnector::from(connector);
    let mut socket = connector.connect(REMOTE_SERVER, socket).await?;

    let http = request.to_raw_http(Some(REMOTE_SERVER));

    socket.write_all(http.as_bytes()).await?;

    let headers_buf = read_headers(&mut socket).await?;

    let mut headers = [EMPTY_HEADER; 64];
    let mut parsed_response = httparse::Response::new(&mut headers);

    match parsed_response.parse(&headers_buf) {
        Ok(status) => match status {
            httparse::Status::Complete(_) => {
                let content_len_header = parsed_response
                    .headers
                    .iter()
                    .find(|header| header.name.eq_ignore_ascii_case("content-length"));

                let content_len = match content_len_header {
                    Some(header) => String::from_utf8_lossy(header.value)
                        .parse::<usize>()
                        .unwrap_or_default(),
                    None => 0,
                };

                let mut body_buf = vec![0u8; content_len];
                socket.read_exact(&mut body_buf).await?;

                let body = String::from_utf8_lossy(&body_buf);
                let mut response = Response::new(body.into_owned());

                *response.version_mut() = Version::HTTP_11;

                if let Some(code) = parsed_response.code {
                    *response.status_mut() =
                        StatusCode::from_u16(code).context("invalid status code")?;
                }

                for header in parsed_response.headers {
                    response.headers_mut().append(
                        HeaderName::from_bytes(header.name.as_bytes())
                            .context("parsing header name")?,
                        HeaderValue::from_bytes(header.value).context("parsing header value")?,
                    );
                }

                Ok(response)
            }
            httparse::Status::Partial => anyhow::bail!("received partial response"),
        },
        Err(err) => Err(anyhow::anyhow!("could not parse response: {:#?}", err)),
    }
}

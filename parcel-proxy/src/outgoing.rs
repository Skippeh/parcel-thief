use std::{fmt::Display, net::SocketAddr, sync::Arc};

use anyhow::{Context, Result};
use http::{header::HeaderName, HeaderValue, Request, Response, StatusCode, Version};
use httparse::EMPTY_HEADER;
use lazy_static::lazy_static;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::RwLock,
};
use tokio_native_tls::TlsConnector;

use crate::http_utility::{read_headers, ToHttp};

#[derive(Debug, Clone)]
pub struct ForwardEndpoint {
    pub domain: String,
    pub addr: SocketAddr,
}

impl Display for ForwardEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{} (ip {})",
            self.domain,
            self.addr.port(),
            self.addr.ip()
        )
    }
}

lazy_static! {
    pub static ref REMOTE_SERVER: Arc<RwLock<Option<ForwardEndpoint>>> =
        Arc::new(RwLock::new(None));
}

pub async fn proxy_request(request: &Request<String>) -> Result<Response<String>> {
    let remote_server_guard = REMOTE_SERVER.read().await;
    let remote_server = remote_server_guard.as_ref().unwrap();

    let socket = TcpStream::connect(&remote_server.addr).await?;
    let connector = tokio_native_tls::native_tls::TlsConnector::builder().build()?;
    let connector = TlsConnector::from(connector);
    let mut socket = connector.connect(&remote_server.domain, socket).await?;

    let http = request.to_raw_http(Some(&remote_server.domain));

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

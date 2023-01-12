use std::str::FromStr;

use anyhow::{Context, Result};
use http::{header::HeaderName, HeaderValue, Method, Request};
use httparse::EMPTY_HEADER;
use tokio::{io::AsyncReadExt, net::TcpStream};
use tokio_rustls::server::TlsStream;

use crate::http_utility::read_headers;

pub async fn parse_request(stream: &mut TlsStream<TcpStream>) -> Result<Request<String>> {
    let mut headers = [EMPTY_HEADER; 64];
    let mut req = httparse::Request::new(&mut headers);

    let raw_headers = read_headers(stream).await?;
    let parse_result: httparse::Status<usize> = req.parse(&raw_headers)?;

    match parse_result {
        httparse::Status::Complete(_bytes_len) => {
            let mut request = http::Request::new("".to_owned());

            if let Some(path) = req.path {
                *request.uri_mut() = http::Uri::from_str(path)?;
            }

            if let Some(method) = req.method {
                let method = Method::from_str(method).context("unknown http method")?;
                *request.method_mut() = method;
            }

            let mut content_len = 0;

            for header in req.headers {
                if header == &EMPTY_HEADER {
                    break;
                }

                request.headers_mut().insert(
                    HeaderName::from_str(header.name).context("invalid header name")?,
                    HeaderValue::from_bytes(header.value).context("invalid header value")?,
                );

                if header.name.eq_ignore_ascii_case("content-length") {
                    let len_str = String::from_utf8(header.value.into())
                        .context("invalid content length encoding")?;

                    content_len = len_str.parse::<usize>().context("invalid content length")?;
                }
            }

            if content_len > 0 {
                let mut content_buf = vec![0; content_len];
                stream.read_exact(&mut content_buf).await?;
                let content = String::from_utf8(content_buf).context("invalid content encoding")?;

                *request.body_mut() = content;
            }

            Ok(request)
        }
        httparse::Status::Partial => anyhow::bail!("partial request received"),
    }
}

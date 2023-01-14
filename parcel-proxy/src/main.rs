pub mod aes;
mod http_utility;
mod incoming;
pub mod logger;
mod outgoing;
mod proxy_response_handler;

use std::{
    fs::File,
    io::BufReader,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result};
use clap::Parser;
use http::{Response, StatusCode};
use lazy_static::lazy_static;
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use tokio_rustls::{
    rustls::{Certificate, PrivateKey, ServerConfig},
    TlsAcceptor,
};

use crate::{http_utility::ToHttp, proxy_response_handler::handle_proxy_response};

lazy_static! {
    pub static ref PUBLIC_URL: Arc<Mutex<String>> = Arc::new(Mutex::new("".into()));
}

#[derive(Parser)]
struct Options {
    cert: Option<PathBuf>,
    key: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Options::parse();

    let mut certs: Option<Vec<Certificate>> = None;
    let mut secure_mode = true;

    if let Some(cert) = args.cert {
        certs = Some(load_certs(&cert)?);
    } else {
        secure_mode = false;
    }

    let mut keys: Option<Vec<PrivateKey>> = None;

    if let Some(key) = args.key {
        keys = Some(load_keys(&key)?);
    } else {
        secure_mode = false;
    }

    let acceptor: Option<TlsAcceptor> = match secure_mode {
        true => {
            let config = ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(
                    certs.unwrap(),
                    keys.unwrap()
                        .pop()
                        .context("no pkcs8 private keys found in keys file")?,
                )?;
            Some(TlsAcceptor::from(Arc::new(config)))
        }
        false => None,
    };
    let addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        if secure_mode { 443 } else { 80 },
    );
    let listener = TcpListener::bind(addr).await?;

    println!("bound on {}", addr);

    *PUBLIC_URL.lock().await = match secure_mode {
        true => "https://127.0.0.1/ds",
        false => "http://127.0.0.1/ds",
    }
    .into();

    loop {
        let (stream, _peer_addr) = listener.accept().await?;
        let acceptor = acceptor.as_ref().cloned();
        let result;

        if secure_mode {
            let stream = acceptor.unwrap().accept(stream).await?;
            result = handle_stream(stream).await;
        } else {
            result = handle_stream::<TcpStream>(stream).await;
        }

        if let Err(err) = result {
            eprintln!("unhandled critical error: {:?}", err);
        }
    }
}

async fn handle_stream<T>(mut stream: T) -> Result<()>
where
    T: AsyncReadExt + AsyncWriteExt + Unpin,
{
    match incoming::parse_request(&mut stream).await {
        Ok(request) => match outgoing::proxy_request(&request).await {
            Ok(mut response) => {
                handle_proxy_response(&request, &mut response).await?;
                stream
                    .write_all(response.to_raw_http(None).as_bytes())
                    .await?;
            }
            Err(err) => {
                eprintln!("error occured while proxying request: {}", err);
                stream
                    .write_all(
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body("".into())
                            .unwrap()
                            .to_raw_http(None)
                            .as_bytes(),
                    )
                    .await?;
            }
        },
        Err(err) => {
            eprintln!("invalid request received: {}", err);
            stream
                .write_all(
                    Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body("".into())
                        .unwrap()
                        .to_raw_http(None)
                        .as_bytes(),
                )
                .await?;
        }
    }

    stream.shutdown().await?;

    Ok(()) as Result<()>
}

fn load_certs(path: &Path) -> Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?))
        .map_err(|_| anyhow::anyhow!("Invalid certificates file"))
        .map(|mut certs| certs.drain(..).map(Certificate).collect())
}

fn load_keys(path: &Path) -> Result<Vec<PrivateKey>> {
    pkcs8_private_keys(&mut BufReader::new(File::open(path)?))
        .map_err(|_| anyhow::anyhow!("Invalid private keys file"))
        .map(|mut keys| keys.drain(..).map(PrivateKey).collect())
}

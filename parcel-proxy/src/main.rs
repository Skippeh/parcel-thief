mod http_utility;
mod incoming;
mod outgoing;

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
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio::{io::AsyncWriteExt, net::TcpListener};
use tokio_rustls::{
    rustls::{Certificate, PrivateKey, ServerConfig},
    TlsAcceptor,
};

use crate::http_utility::ToHttp;

#[derive(Parser)]
struct Options {
    cert: PathBuf,
    key: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Options::parse();

    let certs = load_certs(&args.cert)?;
    let mut keys = load_keys(&args.key)?;

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 443);

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(
            certs,
            keys.pop()
                .context("no pkcs8 private keys found in keys file")?,
        )?;
    let acceptor = TlsAcceptor::from(Arc::new(config));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _peer_addr) = listener.accept().await?;
        let acceptor = acceptor.clone();

        let handler = async move {
            let mut stream = acceptor.accept(stream).await?;

            match incoming::parse_request(&mut stream).await {
                Ok(request) => match outgoing::proxy_request(&request).await {
                    Ok(response) => {
                        stream
                            .write_all(response.to_raw_http(None).as_bytes())
                            .await?;

                        println!("request:\n{:#?}\n\nresponse:\n{:#?}", request, response);
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
        };

        tokio::spawn(async move {
            if let Err(err) = handler.await {
                eprintln!("unhandled critical error: {:?}", err);
            }
        });
    }
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

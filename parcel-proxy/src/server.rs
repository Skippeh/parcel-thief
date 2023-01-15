use std::{
    fs::File,
    io::BufReader,
    net::{IpAddr, SocketAddr},
    path::Path,
    sync::Arc,
};

use anyhow::{Context, Result};
use http::{Response, StatusCode};
use lazy_static::lazy_static;
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Mutex},
};
use tokio_rustls::{
    rustls::{Certificate, PrivateKey, ServerConfig},
    TlsAcceptor,
};

use crate::{
    http_utility::ToHttp, incoming, outgoing, proxy_response_handler::handle_proxy_response,
    AppState, Options, UiMessage,
};

lazy_static! {
    pub static ref PUBLIC_URL: Arc<Mutex<String>> = Arc::new(Mutex::new("".into()));
}

pub async fn start_http_server(
    args: Options,
    app_state: AppState,
    ui_tx: mpsc::UnboundedSender<UiMessage>,
) -> Result<()> {
    let certs_and_keys = match args.cert.is_some() {
        true => Some((args.cert.as_deref().unwrap(), args.key.as_deref().unwrap())),
        false => None,
    };
    let certs_and_keys = match certs_and_keys {
        Some((cert, key)) => Some((load_certs(cert)?, load_keys(key)?)),
        None => None,
    };

    let acceptor: Option<TlsAcceptor> = match &certs_and_keys {
        Some((certs, keys)) => {
            let config = ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(
                    certs.clone(),
                    keys.first()
                        .context("no pkcs8 private keys found in keys file")?
                        .clone(),
                )?;
            Some(TlsAcceptor::from(Arc::new(config)))
        }
        None => None,
    };
    let listen_port = match args.listen_port {
        Some(port) => port,
        None => {
            if certs_and_keys.is_some() {
                443
            } else {
                80
            }
        }
    };
    let addr = SocketAddr::new(args.bind_interface, listen_port);
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("Could not bind tcp listener to {addr}"))?;

    crate::cprintln!("Listening on {}", addr);

    {
        let mut public_url = PUBLIC_URL.lock().await;
        let gateway_domain = match args.gateway_domain {
            Some(gd) => gd,
            None => {
                if args.bind_interface.is_unspecified() {
                    "localhost".to_string()
                } else {
                    args.bind_interface.to_string()
                }
            }
        };
        *public_url = match certs_and_keys.is_some() {
            true => format!("https://{}:{}/ds", gateway_domain, listen_port),
            false => format!("http://{}:{}/ds", gateway_domain, listen_port),
        };

        crate::cprintln!("Gateway address set to {}", public_url);
    }

    loop {
        let (stream, _peer_addr) = listener.accept().await?;
        let acceptor = acceptor.as_ref().cloned();
        let result;

        if certs_and_keys.is_some() {
            let stream = acceptor.unwrap().accept(stream).await?;
            result = handle_stream(stream).await;
        } else {
            result = handle_stream::<TcpStream>(stream).await;
        }

        if let Err(err) = result {
            crate::cprintln!("unhandled critical error: {:?}", err);
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
                crate::cprintln!("error occured while proxying request: {}", err);
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
            crate::cprintln!("invalid request received: {}", err);
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

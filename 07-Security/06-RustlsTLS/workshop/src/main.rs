use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
use rustls::{ClientConfig, ServerConfig};
use std::error::Error;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{TlsAcceptor, TlsConnector};

use rustls_tls_workshop::{
    build_client_config, build_server_config, parse_server_name, tls_echo_roundtrip,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cert = CertificateDer::from(std::fs::read("certs/cert.der")?);
    let key = PrivateKeyDer::from(PrivatePkcs8KeyDer::from(std::fs::read("certs/key.der")?));

    let server_config = build_server_config(vec![cert], key)?;
    let listener = TcpListener::bind("127.0.0.1:8443").await?;
    println!("Listening on 127.0.0.1:8443 (TLS)");

    let server_config_clone = server_config.clone();
    tokio::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let config = server_config_clone.clone();
            tokio::spawn(async move {
                let acceptor = TlsAcceptor::from(config);
                match acceptor.accept(stream).await {
                    Ok(mut tls) => {
                        let mut buf = vec![0u8; 1024];
                        if let Ok(n) = tls.read(&mut buf).await {
                            let _ = tls.write_all(&buf[..n]).await;
                        }
                    }
                    Err(e) => eprintln!("TLS accept error: {}", e),
                }
            });
        }
    });

    let client_config = build_client_config();
    let _name = parse_server_name("localhost")?;
    let _client = TlsConnector::from(client_config);

    Ok(())
}

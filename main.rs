use anyhow::Result;
use rustls::server::{NoClientAuth, ServerConfig};
use std::fs::File;
use rustls::pemfile;
use std::io::BufReader;
use std::net::TcpListener;
use tokio::net::TcpStream;
use tokio::net::TcpListener;
use std::sync::Arc;
use tokio_rustls::TlsAcceptor;
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite};
use tokio::prelude::*;

async fn process_connection(mut stream: tokio_rustls::server::TlsStream<tokio::net::TcpStream>) -> Result<()> {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).await?;

        println!("Received: {}", String::from_utf8_lossy(&buffer[..]));

        Ok(())
    }

    #[tokio::main]
    async fn main() -> Result<()> {
        let mut config = ServerConfig::new(NoClientAuth::new());
        let cert_file = &mut BufReader::new(File::open("cert.pem")?);
        let key_file = &mut BufReader::new(File::open("key.pem")?);
        let cert_chain = rustls::internal::pemfile::certs(cert_file).unwrap();
        let key = rustls::internal::pemfile::rsa_private_keys(key_file).unwrap().remove(0);

        config.set_single_cert(cert_chain, key).unwrap();

        let addr = "127.0.0.1:7878";
        let listener = TcpListener::bind(addr)?;
        let acceptor = TlsAcceptor::from(Arc::new(config));

        println!("Server listening on {}", addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let acceptor = acceptor.clone();

            tokio::spawn(async move {
                if let Err(e) = process_connection(acceptor.accept(stream).await.unwrap()).await {
                    println!("Error: {}", e);
                }
            });
        }
    }

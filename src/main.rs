use example_website::core::database;
use example_website::{config, create_service};
use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::prelude::*;
use salvo::server::ServerHandle;
use tokio::fs;

#[tokio::main]
async fn main() {
    config::init().await;
    let config = crate::config::get();
    database::init(&config.db).await;

    let _guard = config.log.guard();
    tracing::info!("log level: {}", &config.log.filter_level);

    let service = create_service();
    println!("🔄 listen on {}", &config.listen_addr);

    //Acme support, automatically get TLS certificate from Let's Encrypt. For example, see https://github.com/salvo-rs/salvo/blob/main/examples/acme-http01-quinn/src/main.rs
    if config.tls.enabled {
        rustls::crypto::ring::default_provider()
            .install_default()
            .expect("Failed to install rustls crypto provider");

        let cert = fs::read_to_string(&config.tls.cert_path)
            .await
            .expect("unable to read tls cert file");
        let key = fs::read_to_string(&config.tls.key_path)
            .await
            .expect("unable to read tls key file");
        let listen_addr = &config.listen_addr;
        let rustls_config = RustlsConfig::new(Keycert::new().cert(cert.clone()).key(key.clone()));
        let listener = TcpListener::new(listen_addr).rustls(rustls_config.clone());
        let acceptor = QuinnListener::new(rustls_config.build_quinn_config().unwrap(), listen_addr)
            .join(listener)
            .bind()
            .await;
        let server = Server::new(acceptor);
        hook_stop(server.handle());
        server.serve(service).await;
    } else {
        let acceptor = TcpListener::new(&config.listen_addr).bind().await;
        let server = Server::new(acceptor);
        hook_stop(server.handle());
        server.serve(service).await;
    }
}

fn hook_stop(handle: ServerHandle) {
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        handle.stop_graceful(std::time::Duration::from_secs(60));
    });
}

#[cfg(test)]
mod tests {
    use example_website::create_service;
    use salvo::test::{ResponseExt, TestClient};

    use crate::config;

    #[tokio::test]
    async fn test_hello_world() {
        config::init().await;
        let service = create_service();

        let content = TestClient::get(format!(
            "http://{}",
            config::get().listen_addr.replace("0.0.0.0", "127.0.0.1")
        ))
        .send(&service)
        .await
        .take_string()
        .await
        .unwrap();
        assert_eq!(content, "Hello World from salvo");
    }
}

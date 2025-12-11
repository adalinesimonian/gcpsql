use rand::seq::IteratorRandom;
use std::net::{TcpStream, ToSocketAddrs};
use tokio::{
    net::TcpStream as TokioTcpStream,
    time::{Duration as AsyncDuration, Instant as AsyncInstant, sleep as async_sleep},
};

/// Check if a TCP port is available on localhost.
pub fn is_port_available(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);

    addr.to_socket_addrs()
        .ok()
        .and_then(|mut addrs| addrs.next())
        .and_then(|socket_addr| TcpStream::connect(socket_addr).err())
        .is_some()
}

/// Generates a random available TCP port on localhost.
pub fn find_free_port() -> Option<u16> {
    let mut rng = rand::rng();
    (1024..65535)
        .filter(|&port| is_port_available(port))
        .choose(&mut rng)
}

/// Async check if the given address and port respond within the specified timeout in seconds.
pub async fn is_address_responsive_within_timeout_async(
    address: &str,
    port: u16,
    timeout_secs: u64,
) -> bool {
    let addr = format!("{}:{}", address, port);
    let start = AsyncInstant::now();

    while start.elapsed().as_secs() < timeout_secs {
        if TokioTcpStream::connect(&addr).await.is_ok() {
            return true;
        }
        async_sleep(AsyncDuration::from_millis(100)).await;
    }

    false
}

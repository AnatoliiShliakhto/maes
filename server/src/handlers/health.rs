use crate::middleware::*;
use ::axum::http::StatusCode;
use ::std::net::SocketAddr;
use ::tokio::net::UdpSocket;

pub async fn liveness(connection: Connection) -> StatusCode {
    let client_ip = connection.ip;
    let target: SocketAddr = format!("{}:{}", client_ip, 54583)
        .parse()
        .unwrap_or_else(|_| SocketAddr::from(([127, 0, 0, 1], 54583)));

    if let Ok(sock) = UdpSocket::bind(("0.0.0.0", 0)).await {
        let _ = sock.send_to(b"pong", target).await;
    }

    StatusCode::OK
}

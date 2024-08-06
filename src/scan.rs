use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::time::Duration;

pub async fn check_port(host: &str, port: u16) -> bool {
    let address = format!("{}:{}", host, port);
    let addr: SocketAddr = address.parse().unwrap();
    match tokio::time::timeout(Duration::from_millis(300), TcpStream::connect(&addr)).await {
        Ok(Ok(_)) => true,
        _ => false,
    }
}

pub async fn perform_scan(hosts: Vec<std::net::Ipv4Addr>, start_port: u16, end_port: u16) {
    let mut handles = vec![];

    for host in hosts {
        for port in start_port..=end_port {
            let host = host.to_string();
            let handle = tokio::spawn(async move {
                if check_port(&host, port).await {
                    println!("Host {}: Port {} is open", host, port);
                }
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

use port_scanner::config::parse_args;
use port_scanner::scan::perform_scan;

#[tokio::main]
async fn main() {
    let (hosts, start_port, end_port) = parse_args();
    perform_scan(hosts, start_port, end_port).await;
}

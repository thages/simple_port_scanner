use crate::utils::parse_hosts;
use std::env;

pub fn parse_args() -> (Vec<std::net::Ipv4Addr>, u16, u16) {
    let args: Vec<String> = env::args().collect();
    let hosts_arg = args
        .iter()
        .find(|&arg| arg.starts_with("-host="))
        .expect("Missing -host argument");
    let hosts_str = hosts_arg.strip_prefix("-host=").unwrap();

    let hosts = parse_hosts(hosts_str);
    const START_PORT: u16 = 1; // Starting port
    const END_PORT: u16 = 65535; // Ending port

    (hosts, START_PORT, END_PORT)
}

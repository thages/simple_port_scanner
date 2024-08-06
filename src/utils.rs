use ipnetwork::Ipv4Network;
use regex::Regex;
use std::net::Ipv4Addr;
use std::str::FromStr;

pub fn expand_wildcard(ip_pattern: &str) -> Vec<Ipv4Addr> {
    let re = Regex::new(r"^(\d{1,3}\.\d{1,3}\.\d{1,3})\.\*$").unwrap();
    let mut ips = Vec::new();

    if let Some(caps) = re.captures(ip_pattern) {
        let base_ip = &caps[1];
        for i in 0..=255 {
            let ip_str = format!("{}.{}", base_ip, i);
            if let Ok(ip) = Ipv4Addr::from_str(&ip_str) {
                ips.push(ip);
            }
        }
    } else {
        if let Ok(ip) = Ipv4Addr::from_str(ip_pattern) {
            ips.push(ip);
        }
    }

    ips
}

pub fn expand_cidr(cidr: &str) -> Vec<Ipv4Addr> {
    let network = Ipv4Network::from_str(cidr).unwrap();
    let mut ips = Vec::new();

    let start_ip = network.network();
    let end_ip = network.broadcast();

    let mut current_ip = start_ip;
    while current_ip <= end_ip {
        ips.push(current_ip);
        let mut octets = current_ip.octets();
        for i in (0..4).rev() {
            if octets[i] < 255 {
                octets[i] += 1;
                current_ip = Ipv4Addr::from(octets);
                break;
            } else if i == 0 {
                return ips;
            }
            octets[i] = 0;
        }
    }

    ips
}

pub fn parse_hosts(hosts: &str) -> Vec<Ipv4Addr> {
    let mut ips = Vec::new();
    for host in hosts.split(',') {
        if host.contains('*') {
            ips.extend(expand_wildcard(host));
        } else if host.contains('/') {
            ips.extend(expand_cidr(host));
        } else {
            if let Ok(ip) = Ipv4Addr::from_str(host) {
                ips.push(ip);
            }
        }
    }
    ips
}

use crate::parse::{parse_ethernet_header, parse_ip_header};

pub fn handle_packet(buffer: &[u8]) {
    if let Some(eth_header) = parse_ethernet_header(buffer) {
        println!("Parsed Ethernet header: {:?}", eth_header);

        if eth_header.ethertype == 0x0800 {
            if let Some(ip_header) = parse_ip_header(&buffer[14..]) {
                println!("Parsed IP header: {:?}", ip_header);
            }
        }
    }
}

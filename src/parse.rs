#[derive(Debug)]
pub struct EthernetHeader {
    pub destination: Vec<u8>,
    pub source: Vec<u8>,
    pub ethertype: u16,
}

#[derive(Debug)]
pub struct IpHeader {
    pub version: u8,
    pub header_length: u8,
    pub total_length: u16,
    pub protocol: u8,
}

// Function to parse the Ethernet header from the buffer
pub fn parse_ethernet_header(buffer: &[u8]) -> Option<EthernetHeader> {
    if buffer.len() >= 14 {
        Some(EthernetHeader {
            destination: buffer[0..6].to_vec(),
            source: buffer[6..12].to_vec(),
            ethertype: u16::from_be_bytes([buffer[12], buffer[13]]),
        })
    } else {
        None
    }
}

// Function to parse the IP header from the buffer
pub fn parse_ip_header(buffer: &[u8]) -> Option<IpHeader> {
    if buffer.len() >= 20 {
        Some(IpHeader {
            version: (buffer[0] >> 4) & 0xF,
            header_length: buffer[0] & 0xF,
            total_length: u16::from_be_bytes([buffer[2], buffer[3]]),
            protocol: buffer[9],
        })
    } else {
        None
    }
}

// Function to parse a generic packet
pub fn parse_packet(buffer: &[u8]) -> Result<(), &'static str> {
    if let Some(eth_header) = parse_ethernet_header(buffer) {
        println!("Parsed Ethernet Header: {:?}", eth_header);

        if eth_header.ethertype == 0x0800 {
            if let Some(ip_header) = parse_ip_header(&buffer[14..]) {
                println!("Parsed IP Header: {:?}", ip_header);
            } else {
                return Err("Failed to parse IP header");
            }
        }
        Ok(())
    } else {
        Err("Failed to parse Ethernet header")
    }
}

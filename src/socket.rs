extern crate winapi;
use winapi::um::winsock2::{WSADATA, WSAStartup, WSACleanup, WSASocketW, INVALID_SOCKET, SOCKET, WSAGetLastError, bind, setsockopt, SOCKET_ERROR};
use winapi::shared::ws2def::{AF_INET, SOCK_RAW, SOCKADDR_IN, IPPROTO_IP, INADDR_ANY};
use winapi::shared::inaddr::IN_ADDR;
use winapi::shared::ws2ipdef::IP_HDRINCL;
use std::ptr::null_mut;
use std::mem;
use std::net::Ipv4Addr;

fn makeword(low: u8, high: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

pub fn initialize_socket() -> SOCKET {
    unsafe {
        let mut wsa_data: WSADATA = mem::zeroed();
        let wsa_startup = WSAStartup(makeword(2, 2), &mut wsa_data);
        if wsa_startup != 0 {
            panic!("Failed to initialize Winsock. Error: {}", wsa_startup);
        }

        println!("Winsock initialized successfully.");

        let socket = WSASocketW(AF_INET as i32, SOCK_RAW, 0, null_mut(), 0, 0);
        if socket == INVALID_SOCKET {
            let error_code = WSAGetLastError();
            panic!("Failed to create raw socket. Error code: {}", error_code);
        }

        println!("Raw socket created successfully. Socket descriptor: {}", socket);

        let mut address = SOCKADDR_IN {
            sin_family: AF_INET as u16,
            sin_port: 0,
            sin_addr: IN_ADDR { S_un: mem::zeroed() },
            sin_zero: [0; 8],
        };
        
        // Properly set the address
        *(address.sin_addr.S_un.S_addr_mut() as *mut u32) = INADDR_ANY as u32;

        if bind(socket, &address as *const _ as *const _, mem::size_of::<SOCKADDR_IN>() as i32) == SOCKET_ERROR {
            let error_code = WSAGetLastError();
            panic!("Failed to bind socket. Error code: {}", error_code);
        }

        let ipv4_addr = Ipv4Addr::from(u32::from_be(*(address.sin_addr.S_un.S_addr_mut())));
        println!("Socket bound successfully to address: {}", ipv4_addr);

        // Set the socket to promiscuous mode
        let opt_val: i32 = 1;
        if setsockopt(socket, IPPROTO_IP as i32, IP_HDRINCL as i32, &opt_val as *const _ as *const i8, std::mem::size_of::<i32>() as i32) == SOCKET_ERROR {
            let error_code = WSAGetLastError();
            eprintln!("Failed to set socket options. Error code: {}", error_code);
            WSACleanup();
            panic!("Failed to set socket options");
        }
        println!("Socket set to promiscuous mode successfully.");

        socket
    }
}

pub fn cleanup_socket() {
    unsafe {
        println!("Cleaning up Winsock...");
        WSACleanup();
        println!("Winsock cleanup completed.");
    }
}

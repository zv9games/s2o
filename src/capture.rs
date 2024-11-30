use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use winapi::um::winsock2::{recv, SOCKET, WSAGetLastError, WSAETIMEDOUT, WSAStartup, WSADATA, WSACleanup};
use std::time::Duration;

const MAKEWORD: fn(u8, u8) -> u16 = |low, high| ((high as u16) << 8) | (low as u16);

pub fn capture_packets<F>(socket: SOCKET, handle_packet: F, stop_signal: Arc<AtomicBool>)
where
    F: Fn(&[u8]) + Send + Sync + 'static,
{
    // Initialize Winsock
    let mut wsa_data: WSADATA = unsafe { std::mem::zeroed() };
    if unsafe { WSAStartup(MAKEWORD(2, 2), &mut wsa_data) } != 0 {
        eprintln!("WSAStartup failed with error: {}", unsafe { WSAGetLastError() });
        return;
    }

    println!("Winsock initialized for packet capture.");

    let mut buffer = [0u8; 65535];
    let extended_timeout = Duration::from_secs(1);  // Increased timeout to 1 second for better packet capture

    println!("Starting packet capture...");

    while !stop_signal.load(Ordering::Relaxed) {
        println!("Checking for packets...");
        let packet_size = unsafe { recv(socket, buffer.as_mut_ptr() as *mut _, buffer.len() as i32, 0) };
        if packet_size == -1 {
            let error_code = unsafe { WSAGetLastError() };
            if error_code == WSAETIMEDOUT {
                println!("No packet received, timed out.");
                std::thread::sleep(extended_timeout);
                continue;
            } else {
                eprintln!("Error: Failed to capture packet. Error code: {}", error_code);
                break;
            }
        } else if packet_size == 0 {
            println!("Received packet size is zero.");
        } else {
            println!("Captured packet of size: {}", packet_size);
            println!("Packet data: {:?}", &buffer[..packet_size as usize]);

            handle_packet(&buffer[..packet_size as usize]);
        }
    }

    // Cleanup Winsock
    unsafe { WSACleanup() };

    println!("Packet capture ended.");
}

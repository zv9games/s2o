pub mod socket;
pub mod capture;
pub mod parse;
pub mod handlers;
pub mod network_interfaces;

use socket::{initialize_socket, cleanup_socket};
use capture::capture_packets;
use handlers::handle_packet;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub fn run(stop_signal: Arc<AtomicBool>) {
    let socket = initialize_socket();
    capture_packets(socket, handle_packet, stop_signal);
    cleanup_socket();
}

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use s2o_net_lib::socket::initialize_socket;
use s2o_net_lib::capture::capture_packets;
use s2o_net_lib::handlers::handle_packet;
// use s2o_net_lib::network_interfaces::list_network_interfaces;

fn main() {
    println!("Starting s2o_net_lib...");

    // list_network_interfaces(); // Comment this out for now

    let stop_signal = Arc::new(AtomicBool::new(false));
    let socket = initialize_socket();

    capture_packets(socket, handle_packet, stop_signal);
}

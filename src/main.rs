extern crate s2o_net_lib;
use std::sync::atomic::{AtomicBool}; // Ensure Ordering is imported here
use std::sync::Arc;

fn main() {
    let stop_signal = Arc::new(AtomicBool::new(false));
    println!("Starting s2o_net_lib...");

    s2o_net_lib::run(stop_signal);

    println!("s2o_net_lib finished.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering}; // Ensure Ordering is imported here
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    // Test packet capture
    #[test]
    fn test_packet_capture() {
        let stop_signal = Arc::new(AtomicBool::new(false));

        // Start the packet capturing in a separate thread
        let handle = {
            let stop_signal = stop_signal.clone();
            thread::spawn(move || {
                s2o_net_lib::run(stop_signal);
            })
        };

        // Let the application run for a short while
        thread::sleep(Duration::from_secs(10));

        // Signal the capture process to stop
        stop_signal.store(true, Ordering::Relaxed);

        // Wait for the capture thread to finish
        handle.join().unwrap();

        // Verify the captured packets
        // (Add your verification logic here, e.g., checking logs or mock packet handling)
    }

    // Test packet parsing
    #[test]
    fn test_packet_parsing() {
        let sample_packet = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D];
        match s2o_net_lib::parse::parse_packet(&sample_packet) { // Make sure parse_packet is correctly referenced
            Ok(_) => println!("Packet parsing successful."),
            Err(err) => eprintln!("Packet parsing failed: {}", err),
        }
    }

    // Test packet handling
    #[test]
    fn test_packet_handling() {
        let sample_packet = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D];
        s2o_net_lib::handlers::handle_packet(&sample_packet);
        // Add verification logic for handling
    }
}

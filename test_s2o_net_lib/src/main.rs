#[cfg(test)]
mod tests {
    use std::os::windows::raw::SOCKET; // Import SOCKET
    use std::sync::atomic::{AtomicBool, Ordering}; // Ensure Ordering is imported here
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    // Mock function to handle packets
    fn mock_handle_packet(packet: &[u8]) {
        println!("Mock handling packet: {:?}", packet);
    }

    #[test]
    fn test_packet_capture() {
        let stop_signal = Arc::new(AtomicBool::new(false));

        // Mock socket for testing (replace with a valid SOCKET if available)
        let mock_socket: SOCKET = 0; // Placeholder value

        // Start packet capturing in a separate thread
        let handle = {
            let stop_signal = stop_signal.clone();
            thread::spawn(move || {
                s2o_net_lib::capture::capture_packets(mock_socket as usize, mock_handle_packet, stop_signal);
            })
        };

        // Let it run for a short while
        thread::sleep(Duration::from_secs(5)); // Adjust duration as needed

        // Signal to stop capturing
        stop_signal.store(true, Ordering::Relaxed);

        // Wait for the capture thread to finish
        handle.join().unwrap();

        // Verify captured packets (e.g., checking logs or handling output)
    }

    #[test]
    fn test_packet_parsing() {
        let sample_packet = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D];
        match s2o_net_lib::parse::parse_packet(&sample_packet) {
            Ok(_) => println!("Packet parsing successful."),
            Err(err) => eprintln!("Packet parsing failed: {}", err),
        }
    }

    #[test]
    fn test_packet_handling() {
        let sample_packet = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D];
        s2o_net_lib::handlers::handle_packet(&sample_packet);
        // Add verification logic for handling
    }

    #[test]
    fn test_integration_capture_parse_handle() {
        let stop_signal = Arc::new(AtomicBool::new(false));
        let mock_socket: SOCKET = 0; // Placeholder value

        let handle = {
            let stop_signal = stop_signal.clone();
            thread::spawn(move || {
                s2o_net_lib::capture::capture_packets(mock_socket as usize, mock_handle_packet, stop_signal);
            })
        };

        thread::sleep(Duration::from_secs(5)); // Simulate running time
        stop_signal.store(true, Ordering::Relaxed);
        handle.join().unwrap();

        // Add verification logic to check if packets were captured, parsed, and handled correctly
    }

    #[test]
    fn test_high_load_handling() {
        let stop_signal = Arc::new(AtomicBool::new(false));
        let mock_socket: SOCKET = 0; // Placeholder value

        let handle = {
            let stop_signal = stop_signal.clone();
            thread::spawn(move || {
                for _ in 0..10000 {
                    s2o_net_lib::capture::capture_packets(mock_socket as usize, mock_handle_packet, stop_signal.clone());
                }
            })
        };

        thread::sleep(Duration::from_secs(10)); // Run for a longer duration
        stop_signal.store(true, Ordering::Relaxed);
        handle.join().unwrap();

        // Verify the system's stability under high load
    }
}

mod block_all;
mod data_speeds;
mod packet_scan;
mod menu;
mod permissions;

use menu::{admin_menu, main_menu};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "admin" {
        admin_menu();
    } else {
        main_menu();
    }
}

use crate::permissions::{is_user_admin, elevate_process, graceful_shutdown};
use crate::block_all::{block_all_traffic, unblock_all_traffic};
use crate::data_speeds::data_speeds;
use crate::packet_scan::packet_scan;

pub fn main_menu() {
    loop {
        println!("Welcome to s2o_net_lib main menu.");
        println!("Press 1 to enter administrative mode.");
        println!("Press 9 to exit.");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => {
                if !is_user_admin() {
                    elevate_process();
                } else {
                    admin_menu();
                }
            },
            "9" => {
                println!("Exiting the program...");
                graceful_shutdown();
                break;
            },
            _ => println!("Invalid option, please try again."),
        }
    }
}

pub fn admin_menu() {
    println!("Welcome to s2o_net_lib administrative menu.");
    loop {
        println!("Press 1 for data speeds.");
        println!("Press 2 for packet scanning.");
        println!("Press 3 to block all traffic.");
        println!("Press 4 to unblock all traffic.");
        println!("Press 9 to exit.");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => data_speeds(),
            "2" => packet_scan(),
            "3" => block_all_traffic(),
            "4" => unblock_all_traffic(),
            "9" => {
                println!("Exiting administrative menu...");
                break;
            },
            _ => println!("Invalid option, please try again."),
        }
    }

    // Return to main menu after exiting admin menu
    main_menu();
}

mod admin;
mod packet;

use std::io::{self};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Threading::{OpenProcessToken, GetCurrentProcess};
use windows::Win32::Security::{TOKEN_ELEVATION, GetTokenInformation, TokenElevation, TOKEN_QUERY};

fn main() {
    // Check if we need elevated privileges
    if !is_elevated() {
        println!("This program requires elevated privileges. Restarting with elevation...");

        runas::Command::new(std::env::current_exe().unwrap())
            .arg("--elevated")
            .status()
            .expect("Failed to restart with elevated privileges");

        std::process::exit(0);
    }

    println!("Welcome to S2O's network traffic analyzer user mode.");
    println!("Press 3 for administrative mode.");
    println!("Press 9 to exit.");
    println!("Thank you, have a nice day!");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim() {
        "3" => {
            println!("Entering administrative mode...");
            admin::handle_admin();
        }
        "9" => {
            println!("Exiting the program. Goodbye!");
            std::process::exit(0);
        }
        _ => {
            println!("Invalid input. Please press 3 for administrative mode or 9 to exit.");
        }
    }
}

fn is_elevated() -> bool {
    let mut is_elevated = false;
    unsafe {
        let mut token: HANDLE = Default::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).as_bool() {
            let mut elevation = TOKEN_ELEVATION::default();
            let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
            if GetTokenInformation(token, TokenElevation, &mut elevation as *mut _ as _, size, &mut size).as_bool() {
                is_elevated = elevation.TokenIsElevated != 0;
            }
            CloseHandle(token);
        }
    }
    is_elevated
}

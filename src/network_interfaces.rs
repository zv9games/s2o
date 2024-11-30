extern crate winapi;

use std::ffi::CStr;
use winapi::shared::ws2def::AF_UNSPEC;
use winapi::shared::ntdef::NULL;
use winapi::um::iphlpapi::{GetAdaptersAddresses};
use winapi::um::iptypes::{IP_ADAPTER_ADDRESSES_LH, GAA_FLAG_INCLUDE_PREFIX}; // Removed IP_ADAPTER_ADDRESSES
use winapi::shared::winerror::{ERROR_BUFFER_OVERFLOW, NO_ERROR};
use std::os::raw::c_char;

pub struct NetworkInterface {
    pub name: String,
    pub description: String,
    pub operational_status: String,
}

pub fn list_network_interfaces() -> Vec<NetworkInterface> {
    let mut interfaces = Vec::new();

    unsafe {
        let mut adapters_addresses: Vec<u8> = Vec::with_capacity(15000);
        let mut out_buf_len: u32 = adapters_addresses.capacity() as u32;

        // Initial call to GetAdaptersAddresses to get size needed into out_buf_len
        let ret_val = GetAdaptersAddresses(
            AF_UNSPEC as u32,
            GAA_FLAG_INCLUDE_PREFIX,
            NULL,
            adapters_addresses.as_mut_ptr() as *mut _ as *mut IP_ADAPTER_ADDRESSES_LH,
            &mut out_buf_len,
        );

        if ret_val == ERROR_BUFFER_OVERFLOW {
            adapters_addresses.reserve(out_buf_len as usize);
        }

        let ret_val = GetAdaptersAddresses(
            AF_UNSPEC as u32,
            GAA_FLAG_INCLUDE_PREFIX,
            NULL,
            adapters_addresses.as_mut_ptr() as *mut _ as *mut IP_ADAPTER_ADDRESSES_LH,
            &mut out_buf_len,
        );

        if ret_val == NO_ERROR {
            let mut adapter = adapters_addresses.as_ptr() as *const IP_ADAPTER_ADDRESSES_LH;

            while !adapter.is_null() {
                let adapter_name = CStr::from_ptr((*adapter).AdapterName as *const c_char).to_string_lossy().into_owned();
                let description = CStr::from_ptr((*adapter).Description as *const c_char).to_string_lossy().into_owned();
                let operational_status = if (*adapter).OperStatus == 1 { // 1 corresponds to IF_OPER_STATUS_UP
                    "Up"
                } else {
                    "Down"
                }.to_string();

                interfaces.push(NetworkInterface {
                    name: adapter_name,
                    description,
                    operational_status,
                });

                adapter = (*adapter).Next;
            }
        } else {
            eprintln!("GetAdaptersAddresses failed with error: {}", ret_val);
        }
    }

    interfaces
}

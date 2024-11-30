// extern crate winapi;
// use winapi::um::iphlpapi::GetAdaptersInfo;
// use winapi::shared::ipifcons::MAX_ADAPTER_NAME;
// use winapi::shared::iptypes::{IP_ADDR_STRING, FIXED_INFO};
// use std::ptr::null_mut;

// pub fn list_network_interfaces() {
//     unsafe {
//         let mut adapters_info: [u8; 15000] = [0; 15000];
//         let mut size: u32 = adapters_info.len() as u32;

//         if GetAdaptersInfo(adapters_info.as_mut_ptr() as *mut _, &mut size) == 0 {
//             let mut p_adapter_info = adapters_info.as_ptr() as *const FIXED_INFO;
//             while !p_adapter_info.is_null() {
//                 let adapter_name = (*p_adapter_info).AdapterName;
//                 let ip_address = (*p_adapter_info).IpAddressList.IpAddress.String;

//                 println!("Adapter Name: {:?}", adapter_name);
//                 println!("IP Address: {:?}", ip_address);

//                 p_adapter_info = (*p_adapter_info).Next;
//             }
//         } else {
//             println!("Failed to retrieve network adapters.");
//         }
//     }
// }

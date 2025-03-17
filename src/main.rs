mod dpdk;
use dpdk::{eal, types::*};

fn main() {
    // Initialize DPDK EAL
    let args = vec!["my_program".to_string(), "-l".to_string(), "0-3".to_string()];
    eal::init(&args).expect("Failed to initialize EAL");

    // Example: Use the manually defined structs
    let eth_addr = rte_ether_addr {
        addr_bytes: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
    };

    let arp_ipv4 = rte_arp_ipv4 {
        arp_sha: eth_addr,  // No error because rte_ether_addr is Copy
        arp_tha: eth_addr,  // No error because rte_ether_addr is Copy
        arp_sip: 0x0a000001, // 10.0.0.1
        arp_tip: 0x0a000002, // 10.0.0.2
    };

    let arp_hdr = rte_arp_hdr {
        arp_hardware: 1, // Ethernet
        arp_protocol: 0x0800, // IPv4
        arp_hlen: 6, // Ethernet address length
        arp_plen: 4, // IPv4 address length
        arp_opcode: 1, // ARP request
        arp_data: arp_ipv4,
    };

    println!("ARP header created successfully!");

    // Clean up DPDK EAL
    eal::cleanup();
}
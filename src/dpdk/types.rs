#[repr(C)]
#[derive(Copy, Clone)]  // Derive Copy and Clone
pub struct rte_ether_addr {
    pub addr_bytes: [u8; 6],
}

#[repr(C)]
pub struct rte_arp_ipv4 {
    pub arp_sha: rte_ether_addr,
    pub arp_tha: rte_ether_addr,
    pub arp_sip: u32,
    pub arp_tip: u32,
}

#[repr(C)]
pub struct rte_arp_hdr {
    pub arp_hardware: u16,
    pub arp_protocol: u16,
    pub arp_hlen: u8,
    pub arp_plen: u8,
    pub arp_opcode: u16,
    pub arp_data: rte_arp_ipv4,
}
pub type Family = u16;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SocketAddress {
    pub family: Family,
    pub data: [u8; 14],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SocketAddressUnix {
    pub family: Family,
    pub path: [u8; 108],
}

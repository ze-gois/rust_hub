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

/// Abstraction over every concrete `sockaddr`-like representation
/// (`SocketAddress`, `SocketAddressUnix`, and whatever else may come, e.g.
/// `SocketAddressInet`), so syscalls such as `connect`/`bind` can be generic
/// over "either `SocketAddress` or `SocketAddressUnix` may be given",
/// mirroring how POSIX casts every concrete `sockaddr_*` down to a generic
/// `struct sockaddr *` before handing it to the kernel.
pub trait SocketAddressLike {
    fn family(&self) -> Family;

    /// Size, in bytes, of this concrete address representation.
    /// Defaults to the full size of the struct; override when only a
    /// prefix of it is meaningful (e.g. an abstract/short Unix path).
    fn size(&self) -> i32
    where
        Self: Sized,
    {
        core::mem::size_of::<Self>() as i32
    }

    /// Reinterprets this concrete address as a generic `SocketAddress`
    /// pointer, the shape the underlying syscall actually expects.
    fn as_sockaddr(&self) -> *const SocketAddress
    where
        Self: Sized,
    {
        self as *const Self as *const SocketAddress
    }
}

impl SocketAddressLike for SocketAddress {
    fn family(&self) -> Family {
        self.family
    }
}

impl SocketAddressLike for SocketAddressUnix {
    fn family(&self) -> Family {
        self.family
    }
}

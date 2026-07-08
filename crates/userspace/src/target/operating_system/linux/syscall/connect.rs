use crate::target::arch::{Arch, traits::Callable};

pub use super::socket::types::SocketAddress;

hooking!(CONNECT);

#[inline(always)]
pub fn connect(fd: isize, sa: *const SocketAddress, address_length: i32) -> crate::Result {
    let arch_result = Arch::syscall3(NUMBER, fd as usize, sa as usize, address_length as usize);
    handle_result(arch_result)
}

pub mod ok {

    ample::result!( Ok; "Connect Ok"; usize; [
        [0; OK; Default; usize; "Ok"; "All good"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    ample::result!(Error; "Connect error"; usize; [
        [1; ERROR; Default; usize; "Error"; "Something wicked this way comes"],
    ]);

    impl Error {
        pub fn from_no(no: usize) -> Self {
            Error::Default(no)
        }
    }
}

pub use error::Error;
pub use ok::Ok;

pub type Result = core::result::Result<Ok, Error>;

#[rustfmt::skip]
pub fn handle_result(result: crate::Result) -> crate::Result {
    match result {
        crate::Result::Ok(
            crate::Ok::Target(
            crate::target::Ok::Arch(
            crate::target::arch::Ok::X86_64Syscall(
            crate::target::arch::syscall::Ok::X86_64Syscall2(
            crate::target::arch::syscall::syscall2::Ok::Default(m),
        ))))) =>
            core::result::Result::Ok(
                crate::Ok::Target(
                crate::target::Ok::Os(
                crate::target::os::Ok::Syscall(
                crate::target::os::syscall::Ok::Connect(
                crate::target::os::syscall::connect::Ok::Default(m),
            ))))),
        _ =>
            core::result::Result::Err(
                crate::Error::Target(
                crate::target::Error::Os(
                crate::target::os::Error::Syscall(
                crate::target::os::syscall::Error::Connect(
                Error::Default(3),
            ))))),
    }
}

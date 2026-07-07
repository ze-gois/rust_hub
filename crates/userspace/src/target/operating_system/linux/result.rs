pub mod ok {
    use crate::target::os;
    ample::result!(
        Ok;
        "Human Ok";
        usize;
        [
            [0; LINUX_DEFAULT_OK; Default;           usize; "ZE"; "Entry to ze"],
            [1; LINUX_SYSCALL_OK; Syscall; os::syscall::Ok; "ZE"; "Entry to ze"],
        ]
    );

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    use crate::target::os;

    ample::result!(
        Error;
        "Human error";
        usize;
        [
            [0; LINUX_DEFAULT_ERROR; Default;              usize; "ZE"; "Entry to ze"],
            [1; LINUX_SYSCALL_ERROR; Syscall; os::syscall::Error; "ZE"; "Entry to ze"],
        ]
    );

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
pub fn handle_result(result: usize) -> crate::Result {
    if (result as isize) < 0 {
        core::result::Result::Err(
            crate::Error::Target(
            crate::target::Error::Os(
            Error::from_no(result),
        )))
    } else {
        core::result::Result::Ok(
            crate::Ok::Target(
            crate::target::Ok::Os(
            Ok::from_no(result)
        )))
    }
}

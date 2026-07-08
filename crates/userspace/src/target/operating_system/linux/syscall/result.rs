#[rustfmt::skip]
pub mod ok {
    use crate::target::os::syscall;
    ample::result!( Ok; "MMap Ok"; usize; [
        [0; OK;           Default; usize; "Ok"; "All good"],
        [2; ERR_CLOSE;    Close;    syscall::close::Ok;        "close";     "E_CLOSE" ],
        [3; ERR_LSEEK;    LSeek;    syscall::lseek::Ok;        "lseek";     "E_LSEEK"],
        [4; ERR_MMAP;     MMap;     syscall::mmap::Ok;          "mmap";      "E_MMAP"],
        [5; ERR_MPROTECT; MProtect; syscall::mprotect::Ok;  "mprotect";  "E_MPROTECT"],
        [6; ERR_MUNMAP;   MUnmap;   syscall::munmap::Ok;      "munmap";    "E_MUNMAP"],
        [7; ERR_OPEN;     Open;     syscall::open::Ok;          "open";      "E_OPEN"],
        [8; ERR_READ;     Read;     syscall::read::Ok;          "read";      "E_READ"],
        [9; ERR_WRITE;    Write;    syscall::write::Ok;        "write";     "E_WRITE"],
        [10; ERR_FSTAT;   FStat;    syscall::fstat::Ok;        "fstat";     "E_FSTAT"],
        [11; ERR_CONNECT; Connect;    syscall::connect::Ok;        "connect";     "E_CONNECT"],
        [12; ERR_SOCKET;    Socket;   syscall::socket::Ok;        "socket"; "E_SOCKET"],
    ]);

    impl Ok {
        pub fn from_no(no: usize) -> Self {
            Ok::Default(no)
        }
    }
}

pub mod error {
    use crate::target::os::syscall;
    ample::result!(Error; "MMap error"; usize; [
        [1; ERROR;          Default;                  usize;        "Error"; "Something wicked this way comes"],
        [2; ERR_CLOSE;      Close;    syscall::close::Error;        "close"; "E_CLOSE" ],
        [3; ERR_LSEEK;      LSeek;    syscall::lseek::Error;        "lseek"; "E_LSEEK"],
        [4; ERR_MMAP;       MMap;     syscall::mmap::Error;          "mmap"; "E_MMAP"],
        [5; ERR_MPROTECT;   MProtect; syscall::mprotect::Error;  "mprotect"; "E_MPROTECT"],
        [6; ERR_MUNMAP;     MUnmap;   syscall::munmap::Error;      "munmap"; "E_MUNMAP"],
        [7; ERR_OPEN;       Open;     syscall::open::Error;          "open"; "E_OPEN"],
        [8; ERR_READ;       Read;     syscall::read::Error;          "read"; "E_READ"],
        [9; ERR_WRITE;      Write;    syscall::write::Error;        "write"; "E_WRITE"],
        [10; ERR_FSTAT;     FStat;    syscall::fstat::Error;        "fstat"; "E_FSTAT"],
        [11; ERR_CONNECT;   Connect;  syscall::connect::Error;        "connect"; "E_CONNECT"],
        [12; ERR_SOCKET;    Socket;   syscall::socket::Error;        "socket"; "E_SOCKET"],
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

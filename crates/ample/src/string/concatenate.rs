use crate::traits::{Allocatable, AllocatableResult, Bytes};

pub fn concatenate<Origin, Destination, A: Allocatable<Origin>>(
    head: &str,
    tail: &str,
) -> core::result::Result<A::Ok, A::Error>
where
    u8: Bytes<Origin, Destination>,
{
    let tailed = A::allocate_zeroed(
        head.len() * core::mem::size_of::<u8>() + tail.len() * core::mem::size_of::<u8>() + 1,
    )?;
    let tailed = tailed.as_ptr() as *mut u8;
    if tailed.is_null() {
        panic!("allocation failed");
    }

    unsafe {
        core::ptr::copy_nonoverlapping(head.as_ptr(), tailed, head.len());
        core::ptr::copy_nonoverlapping(tail.as_ptr(), tailed.add(head.len()), tail.len());
        *tailed.add(head.len() + tail.len()) = 0;
    };

    core::result::Result::Ok(A::Ok::from_raw(tailed as *mut u8))
}

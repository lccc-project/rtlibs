use core::{cmp::Ordering, mem::MaybeUninit};

use crate::freeze;

pub unsafe fn compare_bytes<T>(left: *const T, right: *const T) -> Ordering
where
    [u8; core::mem::size_of::<T>()]:,
{
    let left = freeze::read_freeze(left as *const [u8; core::mem::size_of::<T>()]).assume_init();
    let right = freeze::read_freeze(right as *const [u8; core::mem::size_of::<T>()]).assume_init();

    return Ord::cmp(&left, &right);
}

use core::mem::ManuallyDrop;

union Transmuter<T, U> {
    input: ManuallyDrop<T>,
    output: ManuallyDrop<U>,
}

pub unsafe fn transmute_unchecked<T, U>(x: T) -> U {
    ManuallyDrop::into_inner(
        Transmuter::<T, U> {
            input: ManuallyDrop::new(x),
        }
        .output,
    )
}

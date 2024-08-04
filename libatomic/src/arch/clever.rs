use super::{ArchAtomic, ArchAtomicFlag};

unsafe impl ArchAtomicFlag for bool {
    type Underlying = bool;

    unsafe fn clear(p: *mut Self, _: crate::Ordering) {
        core::arch::asm!("fence", "mov byte [{}], short 0", in(reg) p);
    }

    unsafe fn test_and_set(p: *mut Self, _: crate::Ordering) -> Self {
        let val: u8;
        core::arch::asm!("fence", "xchg byte [{ptr}], {reg}", "fence", ptr = in(reg) p, reg = inlateout(reg) 1u8=>val);

        val != 0
    }
}

macro_rules! clever_arch_atomic{
    {
        $($inner_ty:ident => $size_spec:literal),* $(,)?
    } => {
        $(unsafe impl ArchAtomic for [u8;::core::mem::size_of::<$inner_ty>()]{
            unsafe fn load(p: *mut Self, _: crate::Ordering) -> Self{
                let val: $inner_ty;
                core::arch::asm!(concat!("mov {reg}, ", $size_spec, "[{ptr}]}"), "fence", ptr = in(reg) p, reg = lateout(reg) val, options(readonly, pure));
                val.to_le_bytes()
            }

            unsafe fn store(p: *mut Self, val: Self, _: crate::Ordering){
                let val: $inner_ty = $inner_ty::from_le_bytes(val);
                core::arch::asm!("fence",concat!("mov ", $size_spec, "[{ptr}], {reg}"), ptr = in(reg) p, reg = in(reg) val, options()); // This wants to be `noglobals, pure`
            }

            unsafe fn compare_exchange(p: *mut Self, expected_val: Self, new_val: Self, _: crate::Ordering, _: crate::Ordering) -> (Self, bool){
                let mut expected_val = $inner_ty::from_le_bytes(expected_val);
                let new_val = $inner_ty::from_le_bytes(new_val);
                let success: u8;

                core::arch::asm!("fence", concat!("cmpxchg ", $size_spec, "[{ptr}], {expected}, {new}"), "fence", "mov.f {out}, 0", "cmovz {out}, 1", ptr = in(reg) p, expected = inout(reg) expected_val, new = in(reg) new_val, out = lateout(reg) success);

                (expected_val.to_le_bytes(), core::mem::transmute(success))
            }
        })*
    }
}

clever_arch_atomic! {
    u8 => "byte",
    u16 => "half",
    u32 => "single",
    u64 => "double"
}

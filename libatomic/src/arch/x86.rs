use core::arch::x86_64::__m128;

use crate::Ordering;

use super::{ArchAtomic, ArchAtomicFlag};

unsafe impl ArchAtomicFlag for bool {
    type Underlying = bool;

    unsafe fn test_and_set(p: *mut Self, _: Ordering) -> Self {
        let mut val: u8 = 1;
        core::arch::asm!("xchg byte ptr [{}], {}", in(reg) p, inout(reg_byte) val);

        unsafe { core::mem::transmute(val) }
    }

    unsafe fn clear(p: *mut Self, _: Ordering) {
        let val: u8 = 0;
        core::arch::asm!("xchg byte ptr [{}], {}", in(reg) p, inout(reg_byte) val => _);
    }
}

unsafe impl ArchAtomic for [u8; 1] {
    unsafe fn load(p: *const Self, _: Ordering) -> Self {
        let val: u8;
        core::arch::asm!("mov {}, byte ptr [{}]",out(reg_byte) val, in(reg) p);
        val.to_le_bytes()
    }

    unsafe fn store(p: *mut Self, val: Self, _: Ordering) {
        let val: u8 = u8::from_le_bytes(val);
        core::arch::asm!("xchg byte ptr [{}], {}", in(reg) p, inout(reg_byte) val => _);
    }

    unsafe fn compare_exchange_weak(
        p: *mut Self,
        expected: Self,
        new: Self,
        success_order: Ordering,
        fail_order: Ordering,
    ) -> (Self, bool) {
        Self::compare_exchange(p, expected, new, success_order, fail_order)
    }

    unsafe fn compare_exchange(
        p: *mut Self,
        expected: Self,
        new: Self,
        _: Ordering,
        _: Ordering,
    ) -> (Self, bool) {
        let mut val_expected = u8::from_le_bytes(expected);
        let val_new = u8::from_le_bytes(new);

        let mut is_val: u8;

        core::arch::asm!("xor ecx, ecx; lock cmpxchg byte ptr [{}], {}; setnz cl", in(reg) p, in(reg_byte) val_new, inout("al") val_expected, out("cl") is_val);

        (val_expected.to_le_bytes(), is_val != 0)
    }

    unsafe fn swap(p: *mut Self, val: Self, ord: Ordering) -> Self {
        let mut val = u8::from_le_bytes(val);
        core::arch::asm!("xchg byte ptr [{}], {}", in(reg) p, inout(reg_byte) val);
        val.to_le_bytes()
    }
}

unsafe impl ArchAtomic for [u8; 2] {
    unsafe fn load(p: *const Self, _: Ordering) -> Self {
        let val: u16;
        core::arch::asm!("mov {:x}, word ptr [{}]",out(reg) val, in(reg) p);
        val.to_le_bytes()
    }

    unsafe fn store(p: *mut Self, val: Self, _: Ordering) {
        let val = u16::from_le_bytes(val);
        core::arch::asm!("xchg word ptr [{}], {:x}", in(reg) p, inout(reg) val => _);
    }

    unsafe fn compare_exchange_weak(
        p: *mut Self,
        expected: Self,
        new: Self,
        success_order: Ordering,
        fail_order: Ordering,
    ) -> (Self, bool) {
        Self::compare_exchange(p, expected, new, success_order, fail_order)
    }

    unsafe fn compare_exchange(
        p: *mut Self,
        expected: Self,
        new: Self,
        _: Ordering,
        _: Ordering,
    ) -> (Self, bool) {
        let mut val_expected = u16::from_le_bytes(expected);
        let val_new = u16::from_le_bytes(new);

        let mut is_val: u8;

        core::arch::asm!("xor ecx, ecx; lock cmpxchg word ptr [{}], {:x}; setnz cl", in(reg) p, in(reg) val_new, inout("ax") val_expected, out("cl") is_val);

        (val_expected.to_le_bytes(), is_val != 0)
    }
    unsafe fn swap(p: *mut Self, val: Self, ord: Ordering) -> Self {
        let mut val = u16::from_le_bytes(val);
        core::arch::asm!("xchg word ptr [{}], {:x}", in(reg) p, inout(reg) val);
        val.to_le_bytes()
    }
}

unsafe impl ArchAtomic for [u8; 4] {
    unsafe fn load(p: *const Self, _: Ordering) -> Self {
        let val: u32;
        core::arch::asm!("mov {:e}, dword ptr [{}]",out(reg) val, in(reg) p);
        val.to_le_bytes()
    }

    unsafe fn store(p: *mut Self, val: Self, _: Ordering) {
        let val = u32::from_le_bytes(val);
        core::arch::asm!("xchg dword ptr [{}], {:e}", in(reg) p, inout(reg) val => _);
    }

    unsafe fn compare_exchange_weak(
        p: *mut Self,
        expected: Self,
        new: Self,
        success_order: Ordering,
        fail_order: Ordering,
    ) -> (Self, bool) {
        Self::compare_exchange(p, expected, new, success_order, fail_order)
    }

    unsafe fn compare_exchange(
        p: *mut Self,
        expected: Self,
        new: Self,
        _: Ordering,
        _: Ordering,
    ) -> (Self, bool) {
        let mut val_expected = u32::from_le_bytes(expected);
        let val_new = u32::from_le_bytes(new);

        let mut is_val: u8;

        core::arch::asm!("xor ecx, ecx; lock cmpxchg dword ptr [{}], {:e}; setnz cl", in(reg) p, in(reg) val_new, inout("ax") val_expected, out("cl") is_val);

        (val_expected.to_le_bytes(), is_val != 0)
    }

    unsafe fn swap(p: *mut Self, val: Self, ord: Ordering) -> Self {
        let mut val = u32::from_le_bytes(val);
        core::arch::asm!("xchg dword ptr [{}], {:e}", in(reg) p, inout(reg) val);
        val.to_le_bytes()
    }
}

#[cfg(target_arch = "x86_64")]
unsafe impl ArchAtomic for [u8; 8] {
    unsafe fn load(p: *const Self, _: Ordering) -> Self {
        let val: u64;
        core::arch::asm!("mov {}, qword ptr [{}]",out(reg) val, in(reg) p);
        val.to_le_bytes()
    }

    unsafe fn store(p: *mut Self, val: Self, _: Ordering) {
        let val = u64::from_le_bytes(val);
        core::arch::asm!("xchg qword ptr [{}], {}", in(reg) p, inout(reg) val => _);
    }

    unsafe fn compare_exchange_weak(
        p: *mut Self,
        expected: Self,
        new: Self,
        success_order: Ordering,
        fail_order: Ordering,
    ) -> (Self, bool) {
        Self::compare_exchange(p, expected, new, success_order, fail_order)
    }

    unsafe fn compare_exchange(
        p: *mut Self,
        expected: Self,
        new: Self,
        _: Ordering,
        _: Ordering,
    ) -> (Self, bool) {
        let mut val_expected = u64::from_le_bytes(expected);
        let val_new = u64::from_le_bytes(new);

        let mut is_val: u8;

        core::arch::asm!("xor ecx, ecx; lock cmpxchg word ptr [{}], {:x}; setnz cl", in(reg) p, in(reg) val_new, inout("ax") val_expected, out("cl") is_val);

        (val_expected.to_le_bytes(), is_val != 0)
    }

    unsafe fn swap(p: *mut Self, val: Self, ord: Ordering) -> Self {
        let mut val = u64::from_le_bytes(val);
        core::arch::asm!("xchg qword ptr [{}], {}", in(reg) p, inout(reg) val);
        val.to_le_bytes()
    }
}

#[cfg(all(not(target_arch = "x86_64"), target_feature = "cmpxchg8b"))]
unsafe impl ArchAtomic for [u8; 8] {
    unsafe fn load(p: *const Self, _: Ordering) -> Self {
        let mut val @ [ref mut l, ref mut h] = [0, 0];
        #[cfg(target_feature = "sse2")]
        {
            core::arch::asm!("movq xmm0, qword ptr [{}]", "movq qword ptr [{}], xmm0", in(reg) p, in(reg) core::ptr::addr_of_mut!(val), out("xmm0") _);
        }
        #[cfg(not(target_feature = "sse2"))]
        {
            core::arch::asm!("mov ebx, eax; mov ecx, edx; cmpxchg8b qword ptr [{}]", in(reg) p, lateout("eax") *l, lateout("edx") *r, out("ecx") _, out("ebx") _);
        }

        core::mem::transmute(val)
    }

    unsafe fn store(p: *const Self, val: Self, _: Ordering) -> Self {
        #[cfg(target_feature = "sse2")]
        {
            core::arch::asm!("mfence","movq xmm0, qword ptr [{}]", "movq qword ptr [{}], xmm0", in(reg) core::ptr::addr_of_mut!(val), in(reg) p, out("xmm0") _);
        }

        #[cfg(not(target_feature = "sse2"))]
        {
            let [l, r]: [u32; 2] = core::mem::transmute(val);
            core::arch::asm!("2: cmpxchg8b [{}]; jnz 2b", in(reg) p, inout("ecx") h=>_, inout("ebx") l=>_);
        }
    }

    unsafe fn compare_exchange(
        p: *const Self,
        expected: Self,
        val: Self,
        _: Ordering,
        _: Ordering,
    ) -> (Self, bool) {
        let [val_l, val_h]: [u32; 2] = core::mem::transmute(val);
        let [mut expected_l, mut expected_h] = core::mem::transmute(expected);
        let res: u32;
        core::arch::asm!("cmpxchg8b [{}]; xor ecx, ecx; setz cl", in(reg) p, inout("eax") expected_l, inout("edx") expected_h, inout("ecx") val_h=> res, in("ebx") expected_l);
        (core::mem::transmute([val_l, val_h]), res != 0)
    }

    unsafe fn compare_exchange_weak(
        p: *const Self,
        expected: Self,
        val: Self,
        succ_order: Ordering,
        fail_order: Ordering,
    ) -> (Self, bool) {
        Self::compare_exchange(p, expected, new, succ_order, fail_order)
    }

    unsafe fn swap(p: *const Self, val: Self, _: Ordering) -> Self {
        let mut flag = false;
        let mut expected_val = Self::load(p, Ordering::Relaxed);
        while flag {
            (expected_val, flag) =
                Self::compare_exchange_weak(p, expected_val, val, ord, Ordering::Relaxed);
        }
        expected_val
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "cmpxchg16b"))]
unsafe impl ArchAtomic for [u8; 16] {
    unsafe fn load(p: *const Self, _: Ordering) -> Self {
        #[cfg(not(target_feature = "avx"))]
        {
            let [l, r]: [u64; 2];

            core::arch::asm!("push rbx; mov rcx, rdx; mov rbx, rax; lock cmpxchg16b [{}]; pop rbx", in(reg) p, out("rax") l, out("rdx") r, out("rcx") _);

            core::mem::transmute([l, r])
        }

        #[cfg(target_feature = "avx")]
        {
            let x: __m128;

            core::arch::asm!("vmovaps {}, [{}]", out(xmm_reg) x, in(reg) p);

            core::mem::transmute(x)
        }
    }

    unsafe fn store(p: *mut Self, val: Self, _: Ordering) {
        #[cfg(not(target_feature = "avx"))]
        {
            let [l, r]: [u64; 2] = core::mem::transmute(val);

            core::arch::asm!("xchg rbx, {rbx}",
                    "2: lock cmpxchg16b [{ptr}]",
                    "jz 2b",
                    "xchg {rbx}, rbx", rbx = inout(reg) r => _, in("rcx") l, out("rax") _, out("rdx") _)
        }
        #[cfg(target_feature = "avx")]
        {
            let x: __m128 = core::mem::transmute(val);

            core::arch::asm!("mfence; vmovaps [{}], {}", in(reg) p, in(xmm_reg) x);
        }
    }

    unsafe fn compare_exchange(
        p: *mut Self,
        expected: Self,
        new: Self,
        _: Ordering,
        _: Ordering,
    ) -> (Self, bool)
    where
        [u8; core::mem::size_of::<Self>()]:,
    {
        let [expected_l, expected_r]: [u64; 2] = core::mem::transmute(expected);
        let [new_l, new_r]: [u64; 2] = core::mem::transmute(new);
        let [l, r]: [u64; 2];
        let cmp: u64;

        core::arch::asm!("xchg rbx, {rbx}; xor {cmp:e},{cmp:e}; lock cmpxchg16b [{ptr}]; setnz {cmp:l}; xchg {rbx}, rbx", rbx= in(reg) new_r ,  cmp = out(reg) cmp, ptr = in(reg) p, inout("rax") expected_l => l, inout("rdx") expected_r => r, in("rcx") new_l);

        (core::mem::transmute([l, r]), cmp != 0)
    }

    unsafe fn compare_exchange_weak(
        p: *mut Self,
        expected: Self,
        new: Self,
        success_order: Ordering,
        fail_order: Ordering,
    ) -> (Self, bool) {
        Self::compare_exchange(p, expected, new, success_order, fail_order)
    }

    unsafe fn swap(p: *mut Self, val: Self, ord: Ordering) -> Self {
        let mut flag = false;
        let mut expected_val = Self::load(p, Ordering::Relaxed);
        while flag {
            (expected_val, flag) =
                Self::compare_exchange_weak(p, expected_val, val, ord, Ordering::Relaxed);
        }
        expected_val
    }
}

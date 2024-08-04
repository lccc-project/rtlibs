// Hack in this file for stable concrete specialization is courtesy of Yandros on the Rust Programming Language Community Server

use crate::{
    align::NaturalAlignment,
    arch::{self, ArchAtomic, AtomicArchImpl},
    locking::AtomicLockingImpl,
};

mod helper {
    use super::*;
    pub trait AtomicArchSwitch<const HAS_ARCH: bool>
    where
        Self: NaturalAlignment,
        [u8; core::mem::size_of::<Self>()]:,
    {
        type T;
    }

    impl<T> AtomicArchSwitch<true> for T
    where
        Self: NaturalAlignment,
        [u8; core::mem::size_of::<Self>()]:,
        [u8; core::mem::size_of::<T>()]: ArchAtomic,
    {
        type T = AtomicArchImpl<T>;
    }

    impl<T> AtomicArchSwitch<false> for T
    where
        Self: NaturalAlignment,
        [u8; core::mem::size_of::<Self>()]:,
    {
        type T = AtomicLockingImpl<T>;
    }

    pub struct HasAtomicArchSel<T>(*mut Self);

    impl<T> HasAtomicArchSel<T>
    where
        T: NaturalAlignment,
        [u8; core::mem::size_of::<T>()]: ArchAtomic,
    {
        pub const HAS_ARCH: bool = true;
    }
    pub trait Fallback {
        const HAS_ARCH: bool;
    }
    impl<T> Fallback for HasAtomicArchSel<T> {
        const HAS_ARCH: bool = false;
    }
}

use helper::*;

use crate::Ordering;
use core::mem::MaybeUninit;

use portable_compiler_intrinsics::freeze;

#[track_caller]
fn assert_store_ordering(ord: Ordering) {
    if let Ordering::AcqRel | Ordering::Acquire = ord {
        panic!("Ordering {:?} is not allowed for loads", ord)
    }
}

#[track_caller]
fn assert_load_ordering(ord: Ordering) {
    if let Ordering::AcqRel | Ordering::Release = ord {
        panic!("Ordering {:?} is not allowed for stores", ord)
    }
}

#[track_caller]
fn assert_cmpxchg_ordering(succ_ord: Ordering, fail_ord: Ordering) {
    match (succ_ord, fail_ord) {
        (Ordering::Relaxed | Ordering::Release, Ordering::Relaxed) => {}
        (Ordering::Acquire | Ordering::AcqRel, Ordering::Relaxed | Ordering::Acquire) => {}
        (_, Ordering::Release | Ordering::AcqRel) => {
            panic!("Failure Ordering {:?} is not allowed", fail_ord)
        }
        (Ordering::Relaxed, _) => panic!(
            "Failure Ordering {:?} (stronger than {:?}) is not allowed",
            fail_ord, succ_ord
        ),
        (Ordering::Acquire | Ordering::AcqRel | Ordering::Release, _) => panic!(
            "Failure Ordering {:?} (stronger than {:?}) is not allowed",
            fail_ord, succ_ord
        ),
        (Ordering::SeqCst, _) => {}
    }
}

macro_rules! export_atomics{
    {
        $($vis:vis type $atomic_ty:ident = Atomic($base_ty:ident);)*
    } => {
        $(
            mod $base_ty{
                use super::*;
                pub type AtomicImpl = <MaybeUninit<$base_ty> as AtomicArchSwitch<{
                    HasAtomicArchSel::<MaybeUninit<$base_ty>>::HAS_ARCH
                }>>::T;
            }

            #[repr(C)]
            $vis struct $atomic_ty($base_ty::AtomicImpl);

            impl $atomic_ty{
                $vis const fn new(x: $base_ty) -> Self{
                    Self($base_ty::AtomicImpl::new(MaybeUninit::new(x)))
                }

                $vis fn into_inner(self) -> $base_ty{
                    unsafe{self.0.into_inner().assume_init()}
                }

                $vis fn get_mut(&mut self) -> &mut $base_ty{
                    unsafe{&mut *(self.0.get_mut() as *mut _ as *mut $base_ty)}
                }

                #[track_caller]
                $vis fn load(&self, order: Ordering) -> $base_ty{
                    assert_load_ordering(order);
                    unsafe{freeze(self.0.load(order)).assume_init()}
                }

                #[track_caller]
                $vis fn store(&self, val: $base_ty, order: Ordering){
                    assert_store_ordering(order);
                    self.0.store(MaybeUninit::new(val),order)
                }

                #[track_caller]
                $vis fn store_uninit(&self, val: MaybeUninit<$base_ty>, order: Ordering){
                    assert_store_ordering(order);
                    self.0.store(val,order)
                }

                #[track_caller]
                $vis fn compare_exchange(&self, expected: $base_ty, new: $base_ty, success_order: Ordering, fail_order: Ordering) -> Result<$base_ty,$base_ty>{
                    assert_cmpxchg_ordering(success_order,fail_order);
                    match self.0.compare_exchange(MaybeUninit::new(expected), MaybeUninit::new(new), success_order, fail_order){
                        Ok(val) => Ok(unsafe{freeze(val).assume_init()}),
                        Err(val) => Err(unsafe{freeze(val).assume_init()})
                    }
                }

                #[track_caller]
                $vis fn compare_exchange_weak(&self, expected: $base_ty, new: $base_ty, success_order: Ordering, fail_order: Ordering) -> Result<$base_ty,$base_ty>{
                    assert_cmpxchg_ordering(success_order,fail_order);
                    match self.0.compare_exchange_weak(MaybeUninit::new(expected), MaybeUninit::new(new), success_order, fail_order){
                        Ok(val) => Ok(unsafe{freeze(val).assume_init()}),
                        Err(val) => Err(unsafe{freeze(val).assume_init()})
                    }
                }

                #[track_caller]
                $vis fn compare_exchange_uninit(&self, expected: MaybeUninit<$base_ty>, new: MaybeUninit<$base_ty>, success_order: Ordering, fail_order: Ordering) -> Result<$base_ty,$base_ty>{
                    assert_cmpxchg_ordering(success_order,fail_order);
                    match self.0.compare_exchange(expected, new, success_order, fail_order){
                        Ok(val) => Ok(unsafe{freeze(val).assume_init()}),
                        Err(val) => Err(unsafe{freeze(val).assume_init()})
                    }
                }

                #[track_caller]
                $vis fn compare_exchange_weak_uninit(&self, expected: MaybeUninit<$base_ty>, new: MaybeUninit<$base_ty>, success_order: Ordering, fail_order: Ordering) -> Result<$base_ty,$base_ty>{
                    assert_cmpxchg_ordering(success_order,fail_order);
                    match self.0.compare_exchange_weak(expected, new, success_order, fail_order){
                        Ok(val) => Ok(unsafe{freeze(val).assume_init()}),
                        Err(val) => Err(unsafe{freeze(val).assume_init()})
                    }
                }
            }
        )*
    }
}

pub type AtomicFlag = arch::AtomicFlag;

export_atomics! {
    pub type AtomicU8Ty = Atomic(u8);
    pub type AtomicU16Ty = Atomic(u16);
    pub type AtomicU32Ty = Atomic(u32);
    pub type AtomicU64Ty = Atomic(u64);
    pub type AtomicU128Ty = Atomic(u128);
    pub type AtomicUsizeTy = Atomic(usize);
    pub type AtomicI8Ty = Atomic(i8);
    pub type AtomicI16Ty = Atomic(i16);
    pub type AtomicI32Ty = Atomic(i32);
    pub type AtomicI64Ty = Atomic(i64);
    pub type AtomicI128Ty = Atomic(i128);
    pub type AtomicIsizeTy = Atomic(isize);
    pub type AtomicBoolTy = Atomic(bool);
}

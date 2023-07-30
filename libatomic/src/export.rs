use crate::{
    align::NaturalAlignment,
    arch::{ArchAtomic, AtomicArchImpl},
    locking::AtomicLockingImpl,
};

mod helper {
    use super::*;
    pub trait Wrap<const HAS_ARCH: bool>
    where
        Self: NaturalAlignment,
        [u8; core::mem::size_of::<Self>()]:,
    {
        type T;
    }

    impl<T> Wrap<true> for T
    where
        Self: NaturalAlignment,
        [u8; core::mem::size_of::<Self>()]:,
        [u8; core::mem::size_of::<T>()]: ArchAtomic,
    {
        type T = AtomicArchImpl<T>;
    }

    impl<T> Wrap<false> for T
    where
        Self: NaturalAlignment,
        [u8; core::mem::size_of::<Self>()]:,
    {
        type T = AtomicLockingImpl<T>;
    }

    pub struct Helper<T>(*mut Self);

    impl<T> Helper<T>
    where
        T: NaturalAlignment,
        [u8; core::mem::size_of::<T>()]: ArchAtomic,
    {
        pub const HAS_ARCH: bool = true;
    }
    pub trait Fallback {
        const HAS_ARCH: bool;
    }
    impl<T> Fallback for Helper<T> {
        const HAS_ARCH: bool = false;
    }
}

use helper::*;

use crate::Ordering;
use core::mem::MaybeUninit;

use portable_compiler_intrinsics::freeze;

macro_rules! export_atomics{
    {
        $($vis:vis type $atomic_ty:ident = Atomic($base_ty:ident);)*
    } => {
        $(
            mod $base_ty{
                use super::*;
                pub type AtomicImpl = <MaybeUninit<$base_ty> as Wrap<{
                    Helper::<MaybeUninit<$base_ty>>::HAS_ARCH
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

                $vis fn load(&self, order: Ordering) -> $base_ty{
                    unsafe{freeze(self.0.load(order)).assume_init()}
                }

                $vis fn store(&self, val: $base_ty, order: Ordering){
                    self.0.store(MaybeUninit::new(val),order)
                }

                $vis fn store_uninit(&self, val: MaybeUninit<$base_ty>, order: Ordering){
                    self.0.store(val,order)
                }

                $vis fn compare_exchange(&self, expected: $base_ty, new: $base_ty, success_order: Ordering, fail_order: Ordering) -> Result<$base_ty,$base_ty>{
                    match self.0.compare_exchange(MaybeUninit::new(expected), MaybeUninit::new(new), success_order, fail_order){
                        Ok(val) => Ok(unsafe{freeze(val).assume_init()}),
                        Err(val) => Err(unsafe{freeze(val).assume_init()})
                    }
                }

                $vis fn compare_exchange_weak(&self, expected: $base_ty, new: $base_ty, success_order: Ordering, fail_order: Ordering) -> Result<$base_ty,$base_ty>{
                    match self.0.compare_exchange_weak(MaybeUninit::new(expected), MaybeUninit::new(new), success_order, fail_order){
                        Ok(val) => Ok(unsafe{freeze(val).assume_init()}),
                        Err(val) => Err(unsafe{freeze(val).assume_init()})
                    }
                }

                $vis fn compare_exchange_uninit(&self, expected: MaybeUninit<$base_ty>, new: MaybeUninit<$base_ty>, success_order: Ordering, fail_order: Ordering) -> Result<$base_ty,$base_ty>{
                    match self.0.compare_exchange(expected, new, success_order, fail_order){
                        Ok(val) => Ok(unsafe{freeze(val).assume_init()}),
                        Err(val) => Err(unsafe{freeze(val).assume_init()})
                    }
                }

                $vis fn compare_exchange_weak_uninit(&self, expected: MaybeUninit<$base_ty>, new: MaybeUninit<$base_ty>, success_order: Ordering, fail_order: Ordering) -> Result<$base_ty,$base_ty>{
                    match self.0.compare_exchange_weak(expected, new, success_order, fail_order){
                        Ok(val) => Ok(unsafe{freeze(val).assume_init()}),
                        Err(val) => Err(unsafe{freeze(val).assume_init()})
                    }
                }
            }
        )*
    }
}

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

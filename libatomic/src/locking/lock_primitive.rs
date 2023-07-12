pub mod spin;

pub type LockImpl = spin::SpinLock;
pub type GuardImpl<'a> = spin::Guard<'a>;

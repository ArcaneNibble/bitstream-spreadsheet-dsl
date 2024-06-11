//! Holds awful hacks

use core::mem::MaybeUninit;

/// This trait is used to work around limitations of min_const_generics.
///
/// It is only implemented on `[bool; N]` and is deliberately hidden.
pub trait MustBeABoolArrayConstGenericsWorkaround: AsRef<[bool]> {
    type MaybeUninitTy: AsMut<[MaybeUninit<bool>]>;
    const NBITS: usize;
}
impl<const N: usize> MustBeABoolArrayConstGenericsWorkaround for [bool; N] {
    type MaybeUninitTy = [MaybeUninit<bool>; N];
    const NBITS: usize = N;
}

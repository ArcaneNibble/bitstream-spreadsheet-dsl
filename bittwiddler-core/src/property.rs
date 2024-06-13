//! Contains traits related to "properties"
//!
//! A "property" is some field in a bitstream that can be set to some value.
//! This includes things such as integers and enums, but it can also be something
//! entirely custom.

#[cfg(feature = "alloc")]
use core::mem::{self, MaybeUninit};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::string::String;

use crate::accessor::PropertyAccessor;
use crate::workarounds::MustBeABoolArrayConstGenericsWorkaround;

/// Trait for converting between property and array of bool
#[allow(private_bounds)]
pub trait PropertyLeaf<T: MustBeABoolArrayConstGenericsWorkaround> {
    /// Convert from a bool array to a property value
    fn from_bits(bits: &T) -> Self;
    /// Convert a property value to a bool array
    fn to_bits(&self) -> T;
}

/// Trait for checking whether or not this field is at its default value
///
/// This is used for creating human-readable files.
///
/// By default this is auto-implemented for anything implementing [Default] (and [PartialEq])
#[allow(private_bounds)]
pub trait PropertyLeafWithDefault<
    T: MustBeABoolArrayConstGenericsWorkaround,
    A: PropertyAccessor + ?Sized,
>: PropertyLeaf<T>
{
    fn is_default(&self, accessor: &A) -> bool;
}
impl<
        B: MustBeABoolArrayConstGenericsWorkaround,
        T: PropertyLeaf<B> + Default + PartialEq,
        A: PropertyAccessor + ?Sized,
    > PropertyLeafWithDefault<B, A> for T
{
    fn is_default(&self, _accessor: &A) -> bool {
        self == &T::default()
    }
}

/// Trait for converting between property and a string
///
/// This is used for creating human-readable files.
///
/// The default implementation converts to a string of '0' and '1' characters
/// corresponding to the raw bits.
#[cfg(feature = "alloc")]
#[allow(private_bounds)]
pub trait PropertyLeafWithStringConv<
    T: MustBeABoolArrayConstGenericsWorkaround,
    A: PropertyAccessor + ?Sized,
>: PropertyLeaf<T>
{
    fn to_string(&self, _accessor: &A) -> Cow<'static, str> {
        let bits = self.to_bits();
        let mut s = String::with_capacity(T::NBITS);
        for i in 0..T::NBITS {
            if bits.as_ref()[i] {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        s.into()
    }
    fn from_string(s: &str, _accessor: &A) -> Result<Self, ()>
    where
        Self: Sized,
    {
        if s.len() != T::NBITS {
            return Err(());
        }

        // safety: T::MaybeUninitTy is an array of MaybeUninit which doesn't require init
        let mut bits: T::MaybeUninitTy = unsafe { MaybeUninit::uninit().assume_init() };
        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                bits.as_mut()[i].write(true);
            } else if c == '0' {
                bits.as_mut()[i].write(false);
            } else {
                return Err(());
            }
        }
        // safety: converting between the same memory representation, guaranteed by MaybeUninit
        let bits = unsafe { mem::transmute_copy::<_, T>(&bits) };
        Ok(Self::from_bits(&bits))
    }
}

// impl PropertyLeaf for bool and integers below

impl PropertyLeaf<[bool; 1]> for bool {
    fn from_bits(bits: &[bool; 1]) -> Self {
        bits[0]
    }

    fn to_bits(&self) -> [bool; 1] {
        [*self]
    }
}
#[cfg(feature = "alloc")]
impl<A: PropertyAccessor> PropertyLeafWithStringConv<[bool; 1], A> for bool {
    fn to_string(&self, _accessor: &A) -> Cow<'static, str> {
        if *self {
            "true".into()
        } else {
            "false".into()
        }
    }

    fn from_string(s: &str, _accessor: &A) -> Result<Self, ()>
    where
        Self: Sized,
    {
        match s {
            "true" | "1" => Ok(true),
            "false" | "0" => Ok(false),
            _ => Err(()),
        }
    }
}

macro_rules! impl_bit_prop_for_int {
    ($nbits:expr, $int_ty:ty) => {
        impl PropertyLeaf<[bool; $nbits]> for $int_ty {
            fn from_bits(bits: &[bool; $nbits]) -> Self {
                let mut ret = 0;
                for i in 0..$nbits {
                    if bits[i] {
                        ret |= 1 << i;
                    }
                }
                ret
            }

            fn to_bits(&self) -> [bool; $nbits] {
                let mut ret = [false; $nbits];
                for i in 0..$nbits {
                    if (self & (1 << i)) != 0 {
                        ret[i] = true;
                    }
                }
                ret
            }
        }
        #[cfg(feature = "alloc")]
        impl<A: PropertyAccessor> PropertyLeafWithStringConv<[bool; $nbits], A> for $int_ty {
            fn to_string(&self, _accessor: &A) -> Cow<'static, str> {
                alloc::format!("0x{self:X}").into()
            }
            fn from_string(s: &str, _accessor: &A) -> Result<Self, ()> {
                if let Some(s) = s.strip_prefix("0x") {
                    Self::from_str_radix(s, 16)
                } else {
                    Self::from_str_radix(s, 10)
                }
                .map_err(|_| ())
            }
        }
    };
}

impl_bit_prop_for_int!(1, u8);
impl_bit_prop_for_int!(2, u8);
impl_bit_prop_for_int!(3, u8);
impl_bit_prop_for_int!(4, u8);
impl_bit_prop_for_int!(5, u8);
impl_bit_prop_for_int!(6, u8);
impl_bit_prop_for_int!(7, u8);
impl_bit_prop_for_int!(8, u8);

impl_bit_prop_for_int!(1, u16);
impl_bit_prop_for_int!(2, u16);
impl_bit_prop_for_int!(3, u16);
impl_bit_prop_for_int!(4, u16);
impl_bit_prop_for_int!(5, u16);
impl_bit_prop_for_int!(6, u16);
impl_bit_prop_for_int!(7, u16);
impl_bit_prop_for_int!(8, u16);
impl_bit_prop_for_int!(9, u16);
impl_bit_prop_for_int!(10, u16);
impl_bit_prop_for_int!(11, u16);
impl_bit_prop_for_int!(12, u16);
impl_bit_prop_for_int!(13, u16);
impl_bit_prop_for_int!(14, u16);
impl_bit_prop_for_int!(15, u16);
impl_bit_prop_for_int!(16, u16);

impl_bit_prop_for_int!(1, u32);
impl_bit_prop_for_int!(2, u32);
impl_bit_prop_for_int!(3, u32);
impl_bit_prop_for_int!(4, u32);
impl_bit_prop_for_int!(5, u32);
impl_bit_prop_for_int!(6, u32);
impl_bit_prop_for_int!(7, u32);
impl_bit_prop_for_int!(8, u32);
impl_bit_prop_for_int!(9, u32);
impl_bit_prop_for_int!(10, u32);
impl_bit_prop_for_int!(11, u32);
impl_bit_prop_for_int!(12, u32);
impl_bit_prop_for_int!(13, u32);
impl_bit_prop_for_int!(14, u32);
impl_bit_prop_for_int!(15, u32);
impl_bit_prop_for_int!(16, u32);
impl_bit_prop_for_int!(17, u32);
impl_bit_prop_for_int!(18, u32);
impl_bit_prop_for_int!(19, u32);
impl_bit_prop_for_int!(20, u32);
impl_bit_prop_for_int!(21, u32);
impl_bit_prop_for_int!(22, u32);
impl_bit_prop_for_int!(23, u32);
impl_bit_prop_for_int!(24, u32);
impl_bit_prop_for_int!(25, u32);
impl_bit_prop_for_int!(26, u32);
impl_bit_prop_for_int!(27, u32);
impl_bit_prop_for_int!(28, u32);
impl_bit_prop_for_int!(29, u32);
impl_bit_prop_for_int!(30, u32);
impl_bit_prop_for_int!(31, u32);
impl_bit_prop_for_int!(32, u32);

impl_bit_prop_for_int!(1, u64);
impl_bit_prop_for_int!(2, u64);
impl_bit_prop_for_int!(3, u64);
impl_bit_prop_for_int!(4, u64);
impl_bit_prop_for_int!(5, u64);
impl_bit_prop_for_int!(6, u64);
impl_bit_prop_for_int!(7, u64);
impl_bit_prop_for_int!(8, u64);
impl_bit_prop_for_int!(9, u64);
impl_bit_prop_for_int!(10, u64);
impl_bit_prop_for_int!(11, u64);
impl_bit_prop_for_int!(12, u64);
impl_bit_prop_for_int!(13, u64);
impl_bit_prop_for_int!(14, u64);
impl_bit_prop_for_int!(15, u64);
impl_bit_prop_for_int!(16, u64);
impl_bit_prop_for_int!(17, u64);
impl_bit_prop_for_int!(18, u64);
impl_bit_prop_for_int!(19, u64);
impl_bit_prop_for_int!(20, u64);
impl_bit_prop_for_int!(21, u64);
impl_bit_prop_for_int!(22, u64);
impl_bit_prop_for_int!(23, u64);
impl_bit_prop_for_int!(24, u64);
impl_bit_prop_for_int!(25, u64);
impl_bit_prop_for_int!(26, u64);
impl_bit_prop_for_int!(27, u64);
impl_bit_prop_for_int!(28, u64);
impl_bit_prop_for_int!(29, u64);
impl_bit_prop_for_int!(30, u64);
impl_bit_prop_for_int!(31, u64);
impl_bit_prop_for_int!(32, u64);
impl_bit_prop_for_int!(33, u64);
impl_bit_prop_for_int!(34, u64);
impl_bit_prop_for_int!(35, u64);
impl_bit_prop_for_int!(36, u64);
impl_bit_prop_for_int!(37, u64);
impl_bit_prop_for_int!(38, u64);
impl_bit_prop_for_int!(39, u64);
impl_bit_prop_for_int!(40, u64);
impl_bit_prop_for_int!(41, u64);
impl_bit_prop_for_int!(42, u64);
impl_bit_prop_for_int!(43, u64);
impl_bit_prop_for_int!(44, u64);
impl_bit_prop_for_int!(45, u64);
impl_bit_prop_for_int!(46, u64);
impl_bit_prop_for_int!(47, u64);
impl_bit_prop_for_int!(48, u64);
impl_bit_prop_for_int!(49, u64);
impl_bit_prop_for_int!(50, u64);
impl_bit_prop_for_int!(51, u64);
impl_bit_prop_for_int!(52, u64);
impl_bit_prop_for_int!(53, u64);
impl_bit_prop_for_int!(54, u64);
impl_bit_prop_for_int!(55, u64);
impl_bit_prop_for_int!(56, u64);
impl_bit_prop_for_int!(57, u64);
impl_bit_prop_for_int!(58, u64);
impl_bit_prop_for_int!(59, u64);
impl_bit_prop_for_int!(60, u64);
impl_bit_prop_for_int!(61, u64);
impl_bit_prop_for_int!(62, u64);
impl_bit_prop_for_int!(63, u64);
impl_bit_prop_for_int!(64, u64);

impl_bit_prop_for_int!(1, u128);
impl_bit_prop_for_int!(2, u128);
impl_bit_prop_for_int!(3, u128);
impl_bit_prop_for_int!(4, u128);
impl_bit_prop_for_int!(5, u128);
impl_bit_prop_for_int!(6, u128);
impl_bit_prop_for_int!(7, u128);
impl_bit_prop_for_int!(8, u128);
impl_bit_prop_for_int!(9, u128);
impl_bit_prop_for_int!(10, u128);
impl_bit_prop_for_int!(11, u128);
impl_bit_prop_for_int!(12, u128);
impl_bit_prop_for_int!(13, u128);
impl_bit_prop_for_int!(14, u128);
impl_bit_prop_for_int!(15, u128);
impl_bit_prop_for_int!(16, u128);
impl_bit_prop_for_int!(17, u128);
impl_bit_prop_for_int!(18, u128);
impl_bit_prop_for_int!(19, u128);
impl_bit_prop_for_int!(20, u128);
impl_bit_prop_for_int!(21, u128);
impl_bit_prop_for_int!(22, u128);
impl_bit_prop_for_int!(23, u128);
impl_bit_prop_for_int!(24, u128);
impl_bit_prop_for_int!(25, u128);
impl_bit_prop_for_int!(26, u128);
impl_bit_prop_for_int!(27, u128);
impl_bit_prop_for_int!(28, u128);
impl_bit_prop_for_int!(29, u128);
impl_bit_prop_for_int!(30, u128);
impl_bit_prop_for_int!(31, u128);
impl_bit_prop_for_int!(32, u128);
impl_bit_prop_for_int!(33, u128);
impl_bit_prop_for_int!(34, u128);
impl_bit_prop_for_int!(35, u128);
impl_bit_prop_for_int!(36, u128);
impl_bit_prop_for_int!(37, u128);
impl_bit_prop_for_int!(38, u128);
impl_bit_prop_for_int!(39, u128);
impl_bit_prop_for_int!(40, u128);
impl_bit_prop_for_int!(41, u128);
impl_bit_prop_for_int!(42, u128);
impl_bit_prop_for_int!(43, u128);
impl_bit_prop_for_int!(44, u128);
impl_bit_prop_for_int!(45, u128);
impl_bit_prop_for_int!(46, u128);
impl_bit_prop_for_int!(47, u128);
impl_bit_prop_for_int!(48, u128);
impl_bit_prop_for_int!(49, u128);
impl_bit_prop_for_int!(50, u128);
impl_bit_prop_for_int!(51, u128);
impl_bit_prop_for_int!(52, u128);
impl_bit_prop_for_int!(53, u128);
impl_bit_prop_for_int!(54, u128);
impl_bit_prop_for_int!(55, u128);
impl_bit_prop_for_int!(56, u128);
impl_bit_prop_for_int!(57, u128);
impl_bit_prop_for_int!(58, u128);
impl_bit_prop_for_int!(59, u128);
impl_bit_prop_for_int!(60, u128);
impl_bit_prop_for_int!(61, u128);
impl_bit_prop_for_int!(62, u128);
impl_bit_prop_for_int!(63, u128);
impl_bit_prop_for_int!(64, u128);
impl_bit_prop_for_int!(65, u128);
impl_bit_prop_for_int!(66, u128);
impl_bit_prop_for_int!(67, u128);
impl_bit_prop_for_int!(68, u128);
impl_bit_prop_for_int!(69, u128);
impl_bit_prop_for_int!(70, u128);
impl_bit_prop_for_int!(71, u128);
impl_bit_prop_for_int!(72, u128);
impl_bit_prop_for_int!(73, u128);
impl_bit_prop_for_int!(74, u128);
impl_bit_prop_for_int!(75, u128);
impl_bit_prop_for_int!(76, u128);
impl_bit_prop_for_int!(77, u128);
impl_bit_prop_for_int!(78, u128);
impl_bit_prop_for_int!(79, u128);
impl_bit_prop_for_int!(80, u128);
impl_bit_prop_for_int!(81, u128);
impl_bit_prop_for_int!(82, u128);
impl_bit_prop_for_int!(83, u128);
impl_bit_prop_for_int!(84, u128);
impl_bit_prop_for_int!(85, u128);
impl_bit_prop_for_int!(86, u128);
impl_bit_prop_for_int!(87, u128);
impl_bit_prop_for_int!(88, u128);
impl_bit_prop_for_int!(89, u128);
impl_bit_prop_for_int!(90, u128);
impl_bit_prop_for_int!(91, u128);
impl_bit_prop_for_int!(92, u128);
impl_bit_prop_for_int!(93, u128);
impl_bit_prop_for_int!(94, u128);
impl_bit_prop_for_int!(95, u128);
impl_bit_prop_for_int!(96, u128);
impl_bit_prop_for_int!(97, u128);
impl_bit_prop_for_int!(98, u128);
impl_bit_prop_for_int!(99, u128);
impl_bit_prop_for_int!(100, u128);
impl_bit_prop_for_int!(101, u128);
impl_bit_prop_for_int!(102, u128);
impl_bit_prop_for_int!(103, u128);
impl_bit_prop_for_int!(104, u128);
impl_bit_prop_for_int!(105, u128);
impl_bit_prop_for_int!(106, u128);
impl_bit_prop_for_int!(107, u128);
impl_bit_prop_for_int!(108, u128);
impl_bit_prop_for_int!(109, u128);
impl_bit_prop_for_int!(110, u128);
impl_bit_prop_for_int!(111, u128);
impl_bit_prop_for_int!(112, u128);
impl_bit_prop_for_int!(113, u128);
impl_bit_prop_for_int!(114, u128);
impl_bit_prop_for_int!(115, u128);
impl_bit_prop_for_int!(116, u128);
impl_bit_prop_for_int!(117, u128);
impl_bit_prop_for_int!(118, u128);
impl_bit_prop_for_int!(119, u128);
impl_bit_prop_for_int!(120, u128);
impl_bit_prop_for_int!(121, u128);
impl_bit_prop_for_int!(122, u128);
impl_bit_prop_for_int!(123, u128);
impl_bit_prop_for_int!(124, u128);
impl_bit_prop_for_int!(125, u128);
impl_bit_prop_for_int!(126, u128);
impl_bit_prop_for_int!(127, u128);
impl_bit_prop_for_int!(128, u128);

//! Contains traits related to "accessors"
//!
//! An "accessor" is the entire collection data needed to select *one* "property"
//! in a bitstream. This might include things like "tile coordinate" or "LUT index".

use core::mem::{self, MaybeUninit};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::borrow::Cow;

use crate::bit_access::{BitArray, Coordinate};
#[cfg(feature = "alloc")]
use crate::property::PropertyLeafWithStringConv;
use crate::property::{PropertyLeaf, PropertyLeafWithDefault};
use crate::workarounds::MustBeABoolArrayConstGenericsWorkaround;

/// This trait needs to be implemented on a type holding a complete package of state
pub trait PropertyAccessor {
    /// Must be a bool array containing the exact number of bits this property takes
    #[allow(private_bounds)]
    type BoolArray: MustBeABoolArrayConstGenericsWorkaround;
    /// Must be the friendly type of this property
    type Output: PropertyLeaf<Self::BoolArray>;

    /// Must implement this. Return a coordinate and invert Y/N for each bit of this property.
    fn get_bit_pos(&self, biti: usize) -> (Coordinate, bool);

    /// Automatically read all the bits, create a bool array, and convert it to a friendly type
    fn get(&self, bitstream: &(impl BitArray + ?Sized)) -> Self::Output {
        let mut bits: <Self::BoolArray as MustBeABoolArrayConstGenericsWorkaround>::MaybeUninitTy =
            unsafe { MaybeUninit::uninit().assume_init() };
        for biti in 0..Self::BoolArray::NBITS {
            let (c, inv) = self.get_bit_pos(biti);
            bits.as_mut()[biti].write(bitstream.get(c) ^ inv);
        }
        let bits = unsafe { mem::transmute_copy::<_, Self::BoolArray>(&bits) };
        Self::Output::from_bits(&bits)
    }
    /// Automatically convert from a friendly type value to a bool array and writes it to the correct coordinates
    fn set(&self, bitstream: &mut (impl BitArray + ?Sized), val: Self::Output) {
        let bits = val.to_bits();
        for biti in 0..Self::BoolArray::NBITS {
            let (c, inv) = self.get_bit_pos(biti);
            bitstream.set(c, bits.as_ref()[biti] ^ inv);
        }
    }
}

/// Allows interacting with this property using strings instead of typed objects
///
/// This is used for creating human-readable files.
#[cfg(feature = "alloc")]
pub trait PropertyAccessorWithStringConv: PropertyAccessor
where
    Self::Output: PropertyLeafWithStringConv<Self::BoolArray, Self>,
{
    fn is_at_default(&self, bitstream: &(impl BitArray + ?Sized)) -> bool {
        let val = self.get(bitstream);
        val.is_default(self)
    }
    fn get_as_string(&self, bitstream: &(impl BitArray + ?Sized)) -> Cow<'static, str> {
        let val = self.get(bitstream);
        val.to_string(self)
    }
    fn set_from_string(
        &self,
        bitstream: &mut (impl BitArray + ?Sized),
        val: &str,
    ) -> Result<(), ()> {
        let val = Self::Output::from_string(val, self)?;
        self.set(bitstream, val);
        Ok(())
    }
}
#[cfg(feature = "alloc")]
impl<A: PropertyAccessor> PropertyAccessorWithStringConv for A where
    Self::Output: PropertyLeafWithStringConv<Self::BoolArray, Self>
{
}

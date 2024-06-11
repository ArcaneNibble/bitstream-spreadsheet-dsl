//! Contains traits for accessing fields dynamically using human-friendly strings
//!
//! Most of this is intended to be implemented automatically with macros

use core::str::FromStr;

extern crate alloc;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::ToString;

use crate::accessor::PropertyAccessorWithStringConv;
use crate::bit_access::BitArray;
use crate::property::PropertyLeafWithStringConv;

/// Trait to be implemented by human text *writer* software to accept
/// "pieces of state" at _this_ hierarchy sublevel only
pub trait HumanSinkForStatePieces {
    fn add_state_piece(&mut self, arg: &str, val: &str);
}

/// Trait for one single bit of state, which can be converted to/from strings
pub trait StatePiece {
    fn _should_add_piece(&self) -> bool {
        true
    }
    fn to_human_string(&self) -> Cow<'static, str>;
    fn from_human_string(s: &str) -> Result<Self, ()>
    where
        Self: Sized;
}
impl<T: ToString + FromStr> StatePiece for T {
    fn to_human_string(&self) -> Cow<'static, str> {
        self.to_string().into()
    }

    fn from_human_string(s: &str) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Self::from_str(s).map_err(|_| ())
    }
}

/// Trait common to sublevels as well as property leaf nodes
/// for dumping all of the "pieces of state"
pub trait HumanLevelThatHasState {
    fn _human_dump_my_state(&self, dump: &mut dyn HumanSinkForStatePieces);
}

/// Object-safe level of wrapping/indirection to poke a property leaf node accessor
///
/// You shouldn't need to implement this, the default impl for [Box] should be sufficient.
pub trait PropertyAccessorDyn: HumanLevelThatHasState {
    fn _human_string_get(&self, bitstream: &dyn BitArray) -> Cow<'static, str>;
    fn _human_string_set(&self, bitstream: &mut dyn BitArray, val: &str) -> Result<(), ()>;
}
impl<A: PropertyAccessorWithStringConv + HumanLevelThatHasState> PropertyAccessorDyn for Box<A>
where
    A::Output: PropertyLeafWithStringConv<A::BoolArray, A>,
{
    fn _human_string_get(&self, bitstream: &dyn BitArray) -> Cow<'static, str> {
        self.get_as_string(bitstream)
    }

    fn _human_string_set(&self, bitstream: &mut dyn BitArray, val: &str) -> Result<(), ()> {
        self.set_from_string(bitstream, val)
    }
}
impl<A: HumanLevelThatHasState> HumanLevelThatHasState for Box<A> {
    fn _human_dump_my_state(&self, dump: &mut dyn HumanSinkForStatePieces) {
        A::_human_dump_my_state(self, dump);
    }
}

/// Trait intended to be implemented automagically for a hierarchy sublevel
pub trait HumanLevelDynamicAccessor: HumanLevelThatHasState {
    fn _human_fields(&self) -> &'static [&'static str];
    fn _human_sublevels(&self) -> &'static [&'static str];

    fn _human_construct_field(
        &self,
        idx: usize,
        params: &[&str],
    ) -> Result<Box<dyn PropertyAccessorDyn>, ()>;
    fn _human_construct_all_fields<'s>(
        &'s self,
        idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn PropertyAccessorDyn>> + 's>;

    fn _human_descend_sublevel(
        &self,
        idx: usize,
        params: &[&str],
    ) -> Result<Box<dyn HumanLevelDynamicAccessor>, ()>;
    fn _human_construct_all_sublevels<'s>(
        &'s self,
        idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn HumanLevelDynamicAccessor>> + 's>;
}

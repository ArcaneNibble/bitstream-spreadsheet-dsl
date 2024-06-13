//! Public exports

#[cfg(feature = "alloc")]
pub use crate::accessor::PropertyAccessorWithStringConv;
pub use crate::accessor::{PropertyAccessor, PropertyAccessorWithDefault};
pub use crate::bit_access::{BitArray, Coordinate};
#[cfg(feature = "alloc")]
pub use crate::human_text::{
    HumanLevelDynamicAccessor, HumanLevelThatHasState, HumanSinkForStatePieces,
    PropertyAccessorDyn, StatePiece,
};
#[cfg(feature = "alloc")]
pub use crate::property::PropertyLeafWithStringConv;
pub use crate::property::{PropertyLeaf, PropertyLeafWithDefault};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
pub use alloc::borrow::Cow as CowReexport;
#[cfg(feature = "alloc")]
pub use alloc::boxed::Box as BoxReexport;

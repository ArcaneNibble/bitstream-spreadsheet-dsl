#![no_std]

//! Core traits that make up the bittwiddler package.
//!
//! Both autogenerated and human-written code will depend on the contents of this crate.

pub mod prelude;

mod accessor;
mod bit_access;
#[cfg(feature = "alloc")]
mod human_text;
mod property;
mod workarounds;


//! Implementations of [`Arbitrary`] for [`alloc`] types,
//!   excluding those in [`core`].
//!
//! [`Arbitrary`]: crate::Arbitrary
//! 

extern crate alloc;

mod borrow;
mod boxed;
mod collections;
mod ffi;
mod rc;
mod string;
mod sync;
mod vec;

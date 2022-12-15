//! A module for utility traits and implementations.
mod lock_type;
mod prim_int;

pub use lock_type::LockType;
pub use prim_int::{FromByteSlice, PrimInt};

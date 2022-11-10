use std::mem::size_of;

/// A trait for arrays that can be instantiated from a u8 slice.
pub trait FromByteSlice {
    fn from_slice(bytes: &[u8]) -> Self;
}

macro_rules! from_byte_slice_impl {
    ($T:ty) => {
        impl FromByteSlice for $T {
            #[inline]
            fn from_slice(bytes: &[u8]) -> $T {
                bytes.try_into().unwrap()
            }
        }
    };
}

from_byte_slice_impl!([u8; 1]);
from_byte_slice_impl!([u8; 2]);
from_byte_slice_impl!([u8; 4]);
from_byte_slice_impl!([u8; 8]);

/// A trait for primitive integers which can be instantiated
/// from a u8 slice.
pub trait PrimInt {
    type Array: FromByteSlice;

    fn from_be_bytes(bytes: Self::Array) -> Self;
    fn from_le_bytes(bytes: Self::Array) -> Self;
    fn from_ne_bytes(bytes: Self::Array) -> Self;
}

macro_rules! prim_int_impl {
    ($T:ty) => {
        impl PrimInt for $T {
            type Array = [u8; size_of::<$T>()];

            #[inline]
            fn from_le_bytes(bytes: Self::Array) -> $T {
                <$T>::from_le_bytes(bytes)
            }

            #[inline]
            fn from_be_bytes(bytes: Self::Array) -> $T {
                <$T>::from_be_bytes(bytes)
            }

            #[inline]
            fn from_ne_bytes(bytes: Self::Array) -> $T {
                <$T>::from_ne_bytes(bytes)
            }
        }
    };
}

prim_int_impl!(u8);
prim_int_impl!(u16);
prim_int_impl!(u32);
prim_int_impl!(u64);
prim_int_impl!(i8);
prim_int_impl!(i16);
prim_int_impl!(i32);
prim_int_impl!(i64);

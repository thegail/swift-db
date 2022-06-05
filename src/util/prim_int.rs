use std::mem::size_of;

pub trait PrimInt {
    type Array;

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

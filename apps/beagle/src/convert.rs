//! This module provides the `Source` and `Destination` traits for converting the primitive types to
//! and from `u64`. The primary reason this exists is to ease the writing of generic code for the
//! virtual machine.

pub trait Source {
    fn from_bits_64(value: u64) -> Self;
}

macro_rules! source_integer {
    ($($t:ty),*) => {
        $(
            impl Source for $t {
                #[inline(always)]
                fn from_bits_64(value: u64) -> Self {
                    value as Self
                }
            }
        )*
    };
}

macro_rules! source_float {
    ($($t:ty),*) => {
        $(
            impl Source for $t {
                #[inline(always)]
                fn from_bits_64(value: u64) -> Self {
                    f64::from_bits(value) as $t
                }
            }
        )*
    };
}

source_integer!(u8, u16, u32, u64, i8, i16, i32, i64);
source_float!(f32, f64);

pub trait Destination: Copy {
    fn to_bits_64(self) -> u64;
}

macro_rules! destination_integer {
    ($($t:ty),*) => {
        $(
            impl Destination for $t {
                #[inline(always)]
                fn to_bits_64(self) -> u64 {
                    self as u64
                }
            }
        )*
    };
}

macro_rules! destination_float {
    ($($t:ty),*) => {
        $(
            impl Destination for $t {
                #[inline(always)]
                fn to_bits_64(self) -> u64 {
                    (self as f64).to_bits()
                }
            }
        )*
    };
}

destination_integer!(u8, u16, u32, u64, i8, i16, i32, i64, bool);
destination_float!(f32, f64);

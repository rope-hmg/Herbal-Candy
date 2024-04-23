use std::mem::transmute;

pub trait Source<T: Copy> {
    fn from_u64(value: u64) -> Self;
}

pub trait Destination: Copy {
    fn into_u64(self) -> u64;
}

// Source
// ------

impl Source<u64> for u8 {
    #[inline(always)]
    fn from_u64(value: u64) -> Self {
        value as u8
    }
}

impl Source<u64> for u16 {
    #[inline(always)]
    fn from_u64(value: u64) -> Self {
        value as u16
    }
}

impl Source<u64> for u32 {
    #[inline(always)]
    fn from_u64(value: u64) -> Self {
        value as u32
    }
}

impl Source<u64> for u64 {
    #[inline(always)]
    fn from_u64(value: u64) -> Self {
        value
    }
}

impl Source<u64> for i8 {
    #[inline(always)]
    fn from_u64(value: u64) -> Self {
        unsafe { transmute::<u64, i64>(value) as i8 }
    }
}

impl Source<u64> for i16 {
    #[inline(always)]
    fn from_u64(value: u64) -> Self {
        unsafe { transmute::<u64, i64>(value) as i16 }
    }
}

impl Source<u64> for i32 {
    #[inline(always)]
    fn from_u64(value: u64) -> Self {
        unsafe { transmute::<u64, i64>(value) as i32 }
    }
}

impl Source<u64> for i64 {
    #[inline(always)]
    fn from_u64(value: u64) -> Self {
        unsafe { transmute::<u64, i64>(value) as i64 }
    }
}

impl Source<u64> for f32 {
    #[inline(always)]
    fn from_u64(value: u64) -> Self {
        f64::from_bits(value) as f32
    }
}

impl Source<u64> for f64 {
    #[inline(always)]
    fn from_u64(value: u64) -> Self {
        f64::from_bits(value)
    }
}

// Destination
// -----------

impl Destination for u8 {
    #[inline(always)]
    fn into_u64(self) -> u64 {
        self as u64
    }
}

impl Destination for u16 {
    #[inline(always)]
    fn into_u64(self) -> u64 {
        self as u64
    }
}

impl Destination for u32 {
    #[inline(always)]
    fn into_u64(self) -> u64 {
        self as u64
    }
}

impl Destination for u64 {
    #[inline(always)]
    fn into_u64(self) -> u64 {
        self
    }
}

impl Destination for i8 {
    #[inline(always)]
    fn into_u64(self) -> u64 {
        unsafe { transmute(self as i64) }
    }
}

impl Destination for i16 {
    #[inline(always)]
    fn into_u64(self) -> u64 {
        unsafe { transmute(self as i64) }
    }
}

impl Destination for i32 {
    #[inline(always)]
    fn into_u64(self) -> u64 {
        unsafe { transmute(self as i64) }
    }
}

impl Destination for i64 {
    #[inline(always)]
    fn into_u64(self) -> u64 {
        unsafe { transmute(self) }
    }
}

impl Destination for f32 {
    #[inline(always)]
    fn into_u64(self) -> u64 {
        (self as f64).to_bits()
    }
}

impl Destination for f64 {
    #[inline(always)]
    fn into_u64(self) -> u64 {
        self.to_bits()
    }
}

impl Destination for bool {
    #[inline(always)]
    fn into_u64(self) -> u64 {
        self as u64
    }
}

use std::hint::unreachable_unchecked;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit_Width {
    Eight,
    Sixteen,
    Thirty_Two,
    Sixty_Four,
}

impl Bit_Width {
    pub fn from_mask(mask: u8) -> Self {
        match mask & 0x3 {
            0b00 => Self::Eight,
            0b01 => Self::Sixteen,
            0b10 => Self::Thirty_Two,
            0b11 => Self::Sixty_Four,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn byte_count(&self) -> usize {
        match self {
            Self::Eight => 1,
            Self::Sixteen => 2,
            Self::Thirty_Two => 4,
            Self::Sixty_Four => 8,
        }
    }
}

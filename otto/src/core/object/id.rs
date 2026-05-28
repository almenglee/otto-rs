/// Packed u64 generational ID (index:generation).
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ID(pub u64);

const INVALID_ID: ID = ID(0);

impl ID {
    #[inline]
    pub const fn new(index: u32, generation: u32) -> Self {
        Self(((generation as u64) << 32) | index as u64)
    }

    #[inline]
    pub fn index(self) -> u32 {
        (self.0 & 0xFFFFFFFF) as u32
    }

    #[inline]
    pub fn generation(self) -> u32 {
        (self.0 >> 32) as u32
    }

    #[inline]
    pub fn is_valid(self) -> bool {
        self == INVALID_ID
    }
}

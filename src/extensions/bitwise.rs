pub trait Bitwise {
    fn is_bit_active(&self, bit_position: u32) -> bool;
    fn active_bit(&mut self, bit_position: u32) -> bool;
    fn deactive_bit(&mut self, bit_position: u32) -> bool;
}
macro_rules! impl_bitwise {
    ($($t:ty),*) => {
        $(
            impl Bitwise for $t {
                fn is_bit_active(&self, bit_position: u32) -> bool {
                    if bit_position >= 8 * std::mem::size_of::<$t>() as u32 {
                        panic!("bit_position out of size of type {}/{}", bit_position, 8 * std::mem::size_of::<$t>());
                    }
                    (self & (1 as $t).wrapping_shl(bit_position)) != 0
                }

                fn active_bit(&mut self, bit_position: u32) -> bool {
                    if bit_position >= 8 * std::mem::size_of::<$t>() as u32 {
                        panic!("bit_position out of size of type {}/{}", bit_position, 8 * std::mem::size_of::<$t>());
                    }
                    if self.is_bit_active(bit_position)
                    {
                        false
                    } else {
                        *self |= (1 as $t).wrapping_shl(bit_position);
                        true
                    }
                }
                fn deactive_bit(&mut self, bit_position: u32) -> bool {
                    if bit_position >= 8 * std::mem::size_of::<$t>() as u32 {
                        panic!("bit_position out of size of type {}/{}", bit_position, 8 * std::mem::size_of::<$t>());
                    }
                    if !self.is_bit_active(bit_position)
                    {
                        false
                    } else {
                        *self &= !(1 as $t).wrapping_shl(bit_position);
                        true
                    }
                }
            }
        )*
    };
}

impl_bitwise!(i8, u8, i16, u16, i32, u32, i64, u64);
impl Bitwise for [u8] {
    fn is_bit_active(&self, bit_position: u32) -> bool {
        if bit_position >= 8 * self.len() as u32 {
            panic!("bit_position must be between 0 and {}", 8 * self.len() - 1);
        }
        (self[bit_position as usize / 8] & (1 << (bit_position % 8))) != 0
    }
    fn active_bit(&mut self, bit_position: u32) -> bool {
        if bit_position >= 8 * self.len() as u32 {
            panic!("bit_position must be between 0 and {}", 8 * self.len() - 1);
        }
        if self.is_bit_active(bit_position) {
            false
        } else {
            self[bit_position as usize / 8] |= 1 << (bit_position % 8);
            true
        }
    }
    fn deactive_bit(&mut self, bit_position: u32) -> bool {
        if bit_position >= 8 * self.len() as u32 {
            panic!("bit_position must be between 0 and {}", 8 * self.len() - 1);
        }
        if !self.is_bit_active(bit_position) {
            false
        } else {
            self[bit_position as usize / 8] &= !(1 << (bit_position % 8));
            true
        }
    }
}
impl Bitwise for Vec<u8> {
    fn is_bit_active(&self, bit_position: u32) -> bool {
        if bit_position >= 8 * self.len() as u32 {
            panic!("bit_position must be between 0 and {}", 8 * self.len() - 1);
        }
        (self[bit_position as usize / 8] & (1 << (bit_position % 8))) != 0
    }
    fn active_bit(&mut self, bit_position: u32) -> bool {
        if bit_position >= 8 * self.len() as u32 {
            panic!("bit_position must be between 0 and {}", 8 * self.len() - 1);
        }
        if self.is_bit_active(bit_position) {
            false
        } else {
            self[bit_position as usize / 8] |= 1 << (bit_position % 8);
            true
        }
    }
    fn deactive_bit(&mut self, bit_position: u32) -> bool {
        if bit_position >= 8 * self.len() as u32 {
            panic!("bit_position must be between 0 and {}", 8 * self.len() - 1);
        }
        if !self.is_bit_active(bit_position) {
            false
        } else {
            self[bit_position as usize / 8] &= !(1 << (bit_position % 8));
            true
        }
    }
}

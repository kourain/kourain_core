pub struct BinaryBuilder {
    pub buffer: Vec<u8>,
    pub is_little_endian: bool,
}

impl BinaryBuilder {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            is_little_endian: true,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            is_little_endian: true,
        }
    }

    #[inline]
    /// return position of the first byte of the data
    pub fn push(&mut self, data: &[u8]) -> usize {
        let pos = self.buffer.len();
        self.buffer.extend_from_slice(data);
        pos
    }

    #[inline]
    pub fn push_u8(&mut self, value: u8) -> usize {
        let pos = self.buffer.len();
        self.buffer.push(value);
        pos
    }

    #[inline]
    pub fn push_u16(&mut self, value: u16) -> usize {
        let bytes = if self.is_little_endian {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.push(&bytes)
    }

    #[inline]
    pub fn push_u32(&mut self, value: u32) -> usize {
        let bytes = if self.is_little_endian {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.push(&bytes)
    }

    #[inline]
    pub fn push_u64(&mut self, value: u64) -> usize {
        let bytes = if self.is_little_endian {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.push(&bytes)
    }

    #[inline]
    pub fn push_i8(&mut self, value: i8) -> usize {
        self.push_u8(value as u8)
    }

    #[inline]
    pub fn push_i16(&mut self, value: i16) -> usize {
        let bytes = if self.is_little_endian {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.push(&bytes)
    }

    #[inline]
    pub fn push_i32(&mut self, value: i32) -> usize {
        let bytes = if self.is_little_endian {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.push(&bytes)
    }

    #[inline]
    pub fn push_i64(&mut self, value: i64) -> usize {
        let bytes = if self.is_little_endian {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.push(&bytes)
    }

    #[inline]
    pub fn push_f32(&mut self, value: f32) -> usize {
        let bytes = if self.is_little_endian {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.push(&bytes)
    }

    #[inline]
    pub fn push_f64(&mut self, value: f64) -> usize {
        let bytes = if self.is_little_endian {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.push(&bytes)
    }

    #[inline]
    pub fn push_string(&mut self, value: &str) -> usize {
        let pos = self.push_u32(value.len() as u32);
        self.buffer.extend_from_slice(value.as_bytes());
        pos
    }

    /// Ghi đè tại vị trí byte đã biết (thay thế drop_at)
    pub fn patch_u32(&mut self, pos: usize, value: u32) {
        let bytes = if self.is_little_endian {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        };
        self.buffer[pos..pos + 4].copy_from_slice(&bytes);
    }

    pub fn to_array(self) -> Vec<u8> {
        self.buffer
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }
}
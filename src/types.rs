pub struct VecU8_16 {
    pub data: [u8; 16]
}

impl VecU8_16 {
    pub fn zero() -> Self {
        Self { data: [0; 16] }
    }
}

pub struct VecU8_8 {
    pub data: [u8; 8]
}

impl VecU8_8 {
    pub fn zero() -> Self {
        Self { data: [0; 8] }
    }
}

pub struct VecU16_8 {
    pub data: [u16; 8]
}

pub struct VecU16_4 {
    pub data: [u16; 4]
}

pub struct VecU32_4 {
    pub data: [u32; 4]
}

pub struct VecU32_2 {
    pub data: [u32; 2]
}

pub struct VecU64_2 {
    pub data: [u64; 2]
}

pub struct VecI8_16 {
    pub data: [i8; 16]
}

pub struct VecI8_8 {
    pub data: [i8; 8]
}

pub struct VecI16_8 {
    pub data: [i16; 8]
}

pub struct VecI16_4 {
    pub data: [i16; 4]
}

pub struct VecI32_4 {
    pub data: [i32; 4]
}

pub struct VecI32_2 {
    pub data: [i32; 2]
}

pub struct VecI64_2 {
    pub data: [i64; 2]
}

pub struct VecF32_4 {
    pub data: [f32; 4]
}

pub struct VecF32_2 {
    pub data: [f32; 2]
}

pub struct VecF64_2 {
    pub data: [f64; 2]
}
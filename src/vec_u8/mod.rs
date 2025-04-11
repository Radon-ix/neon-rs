#[cfg(test)]
mod test {
    use crate::types::{VecU8_16, VecU8_8};

    #[test]
    fn add_u8() {
        let v0 = VecU8_16 {
            data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
        };

        let v1 = VecU8_16 {
            data: [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
        };

        assert_eq!([17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17], (v0 + v1).data);

        let v0 = VecU8_8 {
            data: [1, 2, 3, 4, 5, 6, 7, 8]
        };

        let v1 = VecU8_8 {
            data: [8, 7, 6, 5, 4, 3, 2, 1]
        };

        assert_eq!([9, 9, 9, 9, 9, 9, 9, 9], (v0 + v1).data);
    }
}

use std::ops::Add;

use crate::{ffi::{_add_vecu8_16, _add_vecu8_8}, types::{VecU8_16, VecU8_8}};

impl Add for VecU8_16 {
    type Output = VecU8_16;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self {
            data: [0; 16]
        };

        unsafe {
            _add_vecu8_16(&self.data[0], &rhs.data[0], &mut result.data[0]);
        }

        result
    }
}

impl Add for VecU8_8 {
    type Output = VecU8_8;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self {
            data: [0; 8]
        };

        unsafe {
            _add_vecu8_8(&self.data[0], &rhs.data[0], &mut result.data[0]);
        }

        result
    }
}
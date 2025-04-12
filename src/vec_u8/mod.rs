#[cfg(test)]
mod test;

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
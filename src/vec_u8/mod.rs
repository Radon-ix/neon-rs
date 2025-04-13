#[cfg(test)]
mod test;

pub mod ops;

use std::ops::{Add, Sub};

use crate::{ffi::{_w_add_vecu8_16, _w_add_vecu8_8, _w_sub_vecu8_16, _w_sub_vecu8_8}, types::{VecU8_16, VecU8_8}};

impl Add for VecU8_16 {
    type Output = VecU8_16;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = VecU8_16::zero();

        unsafe {
            _w_add_vecu8_16(&self.data[0], &rhs.data[0], &mut result.data[0]);
        }

        result
    }
}

impl Add for VecU8_8 {
    type Output = VecU8_8;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = VecU8_8::zero();

        unsafe {
            _w_add_vecu8_8(&self.data[0], &rhs.data[0], &mut result.data[0]);
        }

        result
    }
}

impl Sub for VecU8_16 {
    type Output = VecU8_16;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = VecU8_16::zero();

        unsafe {
            _w_sub_vecu8_16(&self.data[0], &rhs.data[0], &mut result.data[0]);
        }

        result
    }
}

impl Sub for VecU8_8 {
    type Output = VecU8_8;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = VecU8_8::zero();

        unsafe {
            _w_sub_vecu8_8(&self.data[0], &rhs.data[0], &mut result.data[0]);
        }

        result
    }
}
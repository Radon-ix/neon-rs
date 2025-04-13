use crate::{ffi::{_w_add_vecu8_16, _w_add_vecu8_8, _w_sub_vecu8_16, _w_sub_vecu8_8}, types::{VecU8_16, VecU8_8}};

pub fn vecu8_16_add(v0: &VecU8_16, v1: &VecU8_16) -> VecU8_16 {
    let mut result = VecU8_16::zero();

    unsafe {
        _w_add_vecu8_16(&v0.data[0], &v1.data[0], &mut result.data[0]);
    }

    result
}

pub fn vecu8_8_add(v0: &VecU8_8, v1: &VecU8_8) -> VecU8_8 {
    let mut result = VecU8_8::zero();

    unsafe {
        _w_add_vecu8_8(&v0.data[0], &v1.data[0], &mut result.data[0]);
    }

    result
}

pub fn vecu8_16_sub(v0: &VecU8_16, v1: &VecU8_16) -> VecU8_16 {
    let mut result = VecU8_16::zero();

    unsafe {
        _w_sub_vecu8_16(&v0.data[0], &v1.data[0], &mut result.data[0]);
    }

    result
}

pub fn vecu8_8_sub(v0: &VecU8_8, v1: &VecU8_8) -> VecU8_8 {
    let mut result = VecU8_8::zero();

    unsafe {
        _w_sub_vecu8_8(&v0.data[0], &v1.data[0], &mut result.data[0]);
    }

    result
}
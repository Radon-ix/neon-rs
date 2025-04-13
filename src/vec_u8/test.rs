use crate::{types::{VecU8_16, VecU8_8}, vec_u8::ops::{vecu8_16_add, vecu8_16_sub, vecu8_8_add, vecu8_8_sub}};

#[test]
fn add_u8_16() {
    let v0 = VecU8_16 {
        data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    };

    let v1 = VecU8_16 {
        data: [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
    };

    assert_eq!([17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17], (v0 + v1).data);

    assert_eq!([17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17], vecu8_16_add(&v0, &v1).data);
}

#[test]
fn add_u8_8() {
    let v0 = VecU8_8 {
        data: [1, 2, 3, 4, 5, 6, 7, 8]
    };

    let v1 = VecU8_8 {
        data: [8, 7, 6, 5, 4, 3, 2, 1]
    };

    assert_eq!([9, 9, 9, 9, 9, 9, 9, 9], (v0 + v1).data);

    assert_eq!([9, 9, 9, 9, 9, 9, 9, 9], vecu8_8_add(&v0, &v1).data);
}

#[test]
fn sub_u8_16() {
    let v0 = VecU8_16 {
        data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    };

    let v1 = VecU8_16 {
        data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    };

    assert_eq!(VecU8_16::zero().data, (v0 - v1).data);

    assert_eq!(VecU8_16::zero().data, vecu8_16_sub(&v0, &v1).data);
}

#[test]
fn sub_u8_8() {
    let v0 = VecU8_8 {
        data: [1, 2, 3, 4, 5, 6, 7, 8]
    };

    let v1 = VecU8_8 {
        data: [1, 2, 3, 4, 5, 6, 7, 8]
    };

    assert_eq!(VecU8_8::zero().data, (v0 - v1).data);

    assert_eq!(VecU8_8::zero().data, vecu8_8_sub(&v0, &v1).data);
}
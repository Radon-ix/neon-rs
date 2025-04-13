#[link(name = "neon", kind = "static")]
unsafe extern "C" {
    pub(crate) fn _w_add_vecu8_16(v0: *const u8, v1: *const u8, result: *mut u8);

    pub(crate) fn _w_add_vecu8_8(v0: *const u8, v1: *const u8, result: *mut u8);

    pub(crate) fn _w_sub_vecu8_16(v0: *const u8, v1: *const u8, result: *mut u8);

    pub(crate) fn _w_sub_vecu8_8(v0: *const u8, v1: *const u8, result: *mut u8);
}
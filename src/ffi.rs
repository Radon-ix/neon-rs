#[link(name = "neon")]
unsafe extern "C" {
    pub(crate) fn _load_vecu8_16(v: *const u8);

    pub(crate) fn _load_vecu8_8(v: *const u8);
}
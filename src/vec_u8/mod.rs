use crate::{ffi::{_load_vecu8_16, _load_vecu8_8}, types::{Loadable, VecU8_16, VecU8_8}};

impl Loadable for VecU8_16 {
    fn load(&self) {
        unsafe {
            _load_vecu8_16(&self.data[0]);
        }
    }
}

impl Loadable for VecU8_8 {
    fn load(&self) {
        unsafe {
            _load_vecu8_8(&self.data[0]);
        }
    }
}
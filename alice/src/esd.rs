use std::fmt;
use std::ffi::CString;

use alice_sys as ffi;

pub struct ESD {
    pub raw: *mut ffi::ESD_t
}

unsafe impl Send for ESD {}
unsafe impl Sync for ESD {}

impl ESD {
    pub fn new(path: &str) -> ESD {
        let local_path = CString::new(path).unwrap();
        let raw = unsafe {ffi::esd_new(local_path.as_ptr())};
        ESD {raw: raw}
    }

    pub fn load_event(&mut self, ievent: i64) -> Option<()> {
        match unsafe {ffi::esd_load_next(self.raw, ievent)} {
            // A return value <= 0 means failure; welcome to AliRoot
            a if a <= 0 => None,
            _ => Some(())
        }
    }
}

impl Drop for ESD {
    fn drop(&mut self) {
        unsafe { ffi::esd_destroy(self.raw); }
    }
}

impl fmt::Debug for ESD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ESD: {:p}", self)
    }
}
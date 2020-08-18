use std::ffi::CString;

pub trait Entity {
    fn entity_name(&self) -> CString;
}
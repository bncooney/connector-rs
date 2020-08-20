use std::ffi::CString;

use super::{Entity as EntityType};

pub trait Entity {
    fn entity_name(&self) -> CString;
    fn entity_type() -> EntityType;
}
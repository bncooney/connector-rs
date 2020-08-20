use std::ffi::CString;

use super::{connector::Connector, entity::Entity, ConnextLibrary, Entity as EntityType, Result};

#[derive(Debug)]
pub struct Writer<'lib> {
	pub(crate) entity_name: CString,
	library: &'lib ConnextLibrary<'lib>,
	handle: isize,
}

impl Entity for Writer<'_> {
	fn entity_name(&self) -> CString {
		self.entity_name.to_owned()
	}

	fn entity_type() -> EntityType {
		EntityType::Writer
	}
}

impl<'lib> Writer<'lib> {
	pub fn new(library: &'lib ConnextLibrary, connector: &Connector, entity_name: &str) -> Result<Self> {
		let entity_name = CString::new(entity_name)?;
		let func = &library.writer_new_symbol;
		let handle: isize;

		unsafe {
			handle = func(connector.connector_handle, entity_name.as_ptr());
		}
		if handle == 0 {
			// Safe to unwrap, &str -> CString -> &str conversion
			return Err(format!("Couldnt create writer, {}", entity_name.to_str().unwrap()).into());
		}

		Ok(Self { entity_name, library, handle })
	}
}

impl Writer<'_> {}

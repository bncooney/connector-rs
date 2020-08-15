use std::{convert::TryInto, ffi::CString, time::Duration};

use super::{connector::Connector, ConnextLibrary, Entity, Handle, Result, ReturnCode, Timeout, NULL_HANDLE};

use num_traits::FromPrimitive;

#[derive(Debug)]
pub struct Reader<'library> {
	library: &'library ConnextLibrary<'library>,
	reader_handle: Handle,
}

impl<'library> Reader<'library> {
	pub fn new(library: &'library ConnextLibrary, connector: &Connector, entity_name: &str) -> Result<Self> {
		let entity_name = CString::new(entity_name)?;
		let reader_new = &library.reader_new_symbol;

		let reader_handle: Handle;
		unsafe {
			reader_handle = reader_new(connector.connector_handle, entity_name.as_ptr());
		}
		if reader_handle == NULL_HANDLE {
			// Safe to unwrap, &str -> CString -> &str conversion
			return Err(format!("Couldnt create reader, {}", entity_name.to_str().unwrap()).into());
		}

		Ok(Self { library, reader_handle })
	}

	pub fn wait(&self, timeout: Option<Duration>) -> Result<()> {
		let timeout_millis: i32;
		match timeout {
			Some(x) => timeout_millis = x.as_millis().try_into().unwrap_or(std::i32::MAX),
			None => timeout_millis = -1,
		}

		let return_code: i32;
		let reader_wait = &self.library.reader_wait_symbol;

		unsafe {
			return_code = reader_wait(self.reader_handle, timeout_millis);
		}

		match ReturnCode::from_i32(return_code) {
			Some(ReturnCode::Ok) => return Ok(()),
			Some(ReturnCode::Timeout) => return Err(Box::new(Timeout { entity: Entity::Reader })),
			_ => return Err("Unexpected error occured in Connector::wait".into()),
		}
	}
}

impl<'library> PartialEq for Reader<'library> {
	fn eq(&self, other: &Self) -> bool {
		self.reader_handle == other.reader_handle
	}
}

impl<'library> Eq for Reader<'library> {}

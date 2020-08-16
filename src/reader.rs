use std::{convert::TryInto, ffi::CString, time::Duration};

use super::{connector::Connector, ConnextLibrary, Entity, Ptr, Result, ReturnCode, Timeout, NULL_PTR};

use num_traits::FromPrimitive;

#[derive(Debug)]
pub struct Reader<'library, 'connector> {
	connector: &'connector Connector<'connector>,
	entity_name: CString,
	library: &'library ConnextLibrary<'library>,
	reader_handle: Ptr,
}

impl<'library, 'connector> Reader<'library, 'connector> {
	pub fn new(library: &'library ConnextLibrary, connector: &'connector Connector, entity_name: &str) -> Result<Self> {
		let c_entity_name = CString::new(entity_name)?;
		let reader_new = &library.reader_new_symbol;

		let reader_handle: Ptr;
		unsafe {
			reader_handle = reader_new(connector.connector_handle, c_entity_name.as_ptr());
		}
		if reader_handle == NULL_PTR {
			return Err(format!("Couldnt create reader, {}", entity_name).into());
		}

		Ok(Self {
			connector,
			entity_name: CString::new(entity_name)?,
			library,
			reader_handle,
		})
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
			_ => return Err(format!("{}:{}","Unexpected error occured in Reader::wait", return_code).into()),
		}
	}

	pub fn take(&self) -> Result<()> {
		let return_code: i32;
		let take = &self.library.take_symbol;
		unsafe {
			return_code = take(self.connector.connector_handle, self.entity_name.as_ptr());
		}
		match ReturnCode::from_i32(return_code) {
			Some(ReturnCode::Ok) => return Ok(()),
			Some(ReturnCode::NoData) => panic!(), //TODO: Manage this as a logical error in calling site?
			_ => return Err(format!("{}:{}","Unexpected error occured in Reader::take", return_code).into()),
		}
	}
}

impl<'library, 'connector> PartialEq for Reader<'library, 'connector> {
	fn eq(&self, other: &Self) -> bool {
		self.reader_handle == other.reader_handle
	}
}

impl<'library, 'connector> Eq for Reader<'library, 'connector> {}

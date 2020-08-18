use std::ffi::CString;
use num_traits::FromPrimitive;

use super::{reader::{Reader, ReturnCode}, ConnextLibrary, Result};

#[derive(Debug)]
pub struct Connector<'lib> {
	library: &'lib ConnextLibrary<'lib>,
	pub(crate) connector_handle: isize,
}

impl<'lib> Connector<'lib> {
	pub fn new(library: &'lib ConnextLibrary, config_name: &str, config_file: &str) -> Result<Self> {
		let config_name = CString::new(config_name)?;
		let config_file = CString::new(config_file)?;

		let connector_handle: isize;
		let connector_new = &library.connector_new_symbol;

		unsafe {
			connector_handle = connector_new(config_name.as_ptr(), config_file.as_ptr(), 0);
		}

		if connector_handle == 0 {
			// Safe to unwrap, &str -> CString -> &str conversion
			return Err(format!("Couldnt create connector, {}", config_name.to_str().unwrap()).into());
		}

		Ok(Self { library, connector_handle })
	}
}

impl Connector<'_> {
	pub fn take(&self, reader: Reader) -> Result<()> {
		let func = &self.library.take_symbol;
		let return_code: i32;

		unsafe {
			return_code = func(self.connector_handle, reader.entity_name.as_ptr());
		}

		match ReturnCode::from_i32(return_code) {
			Some(ReturnCode::Ok) => return Ok(()),
			Some(ReturnCode::NoData) => return Ok(()), //TODO: Log this as a logical error
			_ => return Err(format!("{}:{}", "Unexpected error occured in Reader::next", return_code).into()),
		}
	}
}

impl Drop for Connector<'_> {
	fn drop(&mut self) {
		let connector_delete = &self.library.connector_delete_symbol;
		unsafe {
			connector_delete(self.connector_handle);
		}
	}
}

impl PartialEq for Connector<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.connector_handle == other.connector_handle
	}
}

impl Eq for Connector<'_> {}

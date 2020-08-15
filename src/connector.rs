use std::ffi::CString;

use super::{ConnextLibrary, Result};

#[derive(Debug)]
pub struct Connector<'library> {
	library: &'library ConnextLibrary<'library>,
	pub(crate) connector_handle: isize,
}

impl<'library> Connector<'library> {
	pub fn new(library: &'library ConnextLibrary, config_name: &str, config_file: &str) -> Result<Self> {
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

impl<'library> Drop for Connector<'library> {
	fn drop(&mut self) {
		let connector_delete = &self.library.connector_delete_symbol;
		unsafe {
			connector_delete(self.connector_handle);
		}
	}
}

impl<'library> PartialEq for Connector<'library> {
	fn eq(&self, other: &Self) -> bool {
		self.connector_handle == other.connector_handle
	}
}

impl<'library> Eq for Connector<'library> {}

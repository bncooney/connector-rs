use std::{
	// convert::TryInto,
	ffi::{CStr, CString},
	// os::raw::c_char,
	// time::Duration,
};

use super::{ConnextLibrary, Result, ReturnCode, TimeoutError};

// use libloading::{Library, Symbol};
// use num_traits::FromPrimitive;

#[derive(Debug)]
pub struct Connector<'library> {
	library: &'library ConnextLibrary<'library>,
	connector_handle: isize,
}

impl<'library> Connector<'library> {
	pub fn new(library: &'library ConnextLibrary, config_name: &str, config_file: &str) -> Result<Self> {
		let connector_handle: isize;
		let fn_connector_new = &library.connector_new_symbol;

		unsafe {
			connector_handle = fn_connector_new(CString::new(config_name)?.as_ptr(), CString::new(config_file)?.as_ptr(), 0);
		}

		if connector_handle == 0 {
			return Err("Couldn't create RTIDDSconnector, see stderr".into());
		}

		Ok(Self { library, connector_handle })
	}
}

impl<'library> Drop for Connector<'library> {
	fn drop(&mut self) {
		let fn_connector_delete = &self.library.connector_delete_symbol;
		unsafe {
			fn_connector_delete(self.connector_handle);
		}
	}
}

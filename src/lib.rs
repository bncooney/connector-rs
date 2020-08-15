use std::{
	// convert::TryInto,
	// ffi::{CStr, CString},
	fmt::Display,
	os::raw::c_char,
	// time::Duration,
};

pub mod connector;
pub mod reader;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;

#[macro_use]
extern crate num_derive;
use num_derive::FromPrimitive;
// use num_traits::FromPrimitive;

use libloading::{Library, Symbol};

#[derive(Debug)]
pub struct ConnextLibrary<'library> {
	connector_new_symbol: Symbol<'library, unsafe extern "C" fn(config_name: *const c_char, config_file: *const c_char, config: isize) -> isize>,
	connector_delete_symbol: Symbol<'library, unsafe extern "C" fn(connector_handle: isize)>,
	reader_new_symbol: Symbol<'library, unsafe extern "C" fn(connector_handle: isize, entity_name: *const c_char) -> isize>,
}

impl<'library> ConnextLibrary<'library> {

	pub fn new(library: &'library Library) -> Result<Self> {
		Ok(ConnextLibrary {
			connector_new_symbol: ConnextLibrary::load_connector_new_symbol(library)?,
			connector_delete_symbol: ConnextLibrary::load_connector_delete_symbol(library)?,
			reader_new_symbol: ConnextLibrary::load_reader_new_symbol(library)?,
		})
	}

	fn load_connector_new_symbol(library: &'library Library) -> Result<Symbol<'library, unsafe extern "C" fn(*const c_char, *const c_char, isize) -> isize>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_new")?;
		}
		return Ok(func);
	}

	fn load_connector_delete_symbol(library: &'library Library) -> Result<Symbol<'library, unsafe extern "C" fn(connector_handle: isize)>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_delete")?;
		}
		return Ok(func);
	}

	fn load_reader_new_symbol(library: &'library Library) -> Result<Symbol<'library, unsafe extern "C" fn(connector_handle: isize, entity_name: *const c_char) -> isize>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_getReader")?;
		}
		return Ok(func);
	}

}

#[derive(Debug)]
pub struct TimeoutError;

impl Display for TimeoutError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		// TODO: Add subject enum (Connector / Participant, Reader)?
		write!(f, "{}", "Connector wait timed out.")
	}
}

impl std::error::Error for TimeoutError {}

#[derive(FromPrimitive, ToPrimitive)]
enum ReturnCode {
	Ok = 0,
	Timeout = 10,
	NoData = 11,
}

// #[derive(Debug)]
// pub struct Writer {
// 	writer_handle: isize,
// }

// impl PartialEq for Writer {
// 	fn eq(&self, other: &Self) -> bool {
// 		self.writer_handle == other.writer_handle
// 	}
// }

// impl Eq for Writer {}

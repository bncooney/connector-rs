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

type Handle = isize;
const NULL_HANDLE: Handle = 0;

#[derive(Debug)]
pub struct ConnextLibrary<'library> {
	connector_new_symbol: Symbol<'library, unsafe extern "C" fn(config_name: *const c_char, config_file: *const c_char, config: i32) -> Handle>,
	connector_delete_symbol: Symbol<'library, unsafe extern "C" fn(connector_handle: Handle)>,
	reader_new_symbol: Symbol<'library, unsafe extern "C" fn(connector_handle: Handle, entity_name: *const c_char) -> Handle>,
	reader_wait_symbol: Symbol<'library, unsafe extern "C" fn(reader_handle: Handle, timeout: i32) -> i32>,
}

impl<'library> ConnextLibrary<'library> {

	pub fn new(library: &'library Library) -> Result<Self> {
		Ok(ConnextLibrary {
			connector_new_symbol: ConnextLibrary::load_connector_new_symbol(library)?,
			connector_delete_symbol: ConnextLibrary::load_connector_delete_symbol(library)?,
			reader_new_symbol: ConnextLibrary::load_reader_new_symbol(library)?,
			reader_wait_symbol: ConnextLibrary::load_reader_wait_symbol(library)?,
		})
	}

	fn load_connector_new_symbol(library: &'library Library) -> Result<Symbol<'library, unsafe extern "C" fn(*const c_char, *const c_char, i32) -> Handle>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_new")?;
		}
		return Ok(func);
	}

	fn load_connector_delete_symbol(library: &'library Library) -> Result<Symbol<'library, unsafe extern "C" fn(Handle)>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_delete")?;
		}
		return Ok(func);
	}

	fn load_reader_new_symbol(library: &'library Library) -> Result<Symbol<'library, unsafe extern "C" fn(Handle, *const c_char) -> Handle>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_getReader")?;
		}
		return Ok(func);
	}

	fn load_reader_wait_symbol(library: &'library Library) -> Result<Symbol<'library, unsafe extern "C" fn(Handle, i32) -> i32>> {
		let func;
		unsafe {
			func = library.get(b"RTI_Connector_wait_for_data_on_reader")?;
		}
		return Ok(func);
	}

}

#[derive(Debug)]
pub(crate) enum Entity {
	Connector,
	Reader,
	Writer,
}

#[derive(Debug)]
pub(crate) struct Timeout {
	entity: Entity,
}

impl Display for Timeout {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?} wait timed out", &self.entity)
	}
}

impl std::error::Error for Timeout {}

#[derive(FromPrimitive, ToPrimitive)]
pub enum ReturnCode {
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

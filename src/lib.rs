use std::os::raw::c_char;

pub mod connector;
pub mod entity;
pub mod reader;
pub mod writer;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;

use libloading::{Library, Symbol};

type CString = *const c_char;
type SamplesLength = f64; // TODO: Look into why "samples length" is a double

#[derive(Debug)]
pub struct ConnextLibrary<'lib> {
	connector_new_symbol: Symbol<'lib, unsafe extern "C" fn(config_name: CString, config_file: CString, config: i32) -> isize>,
	connector_delete_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: isize)>,
	reader_new_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: isize, entity_name: CString) -> isize>,
	writer_new_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: isize, entity_name: CString) -> isize>,
	reader_wait_symbol: Symbol<'lib, unsafe extern "C" fn(reader_handle: isize, timeout: i32) -> i32>,
	writer_clear_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: isize, entity_name: CString)>,
	take_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: isize, entity_name: CString) -> i32>,
	read_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: isize, entity_name: CString) -> i32>,
	writer_write_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: isize, entity_name: CString, params_json: CString)>,
	get_samples_length_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: isize, entity_name: CString) -> SamplesLength>,
	get_json_sample_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: isize, entity_name: CString, index: i32) -> CString>,
	set_json_instance_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: isize, entity_name: CString, json: CString)>,
	free_string_symbol: Symbol<'lib, unsafe extern "C" fn(CString)>,
}

impl<'lib> ConnextLibrary<'lib> {
	pub fn new(library: &'lib Library) -> Result<Self> {
		Ok(ConnextLibrary {
			connector_new_symbol: ConnextLibrary::load_connector_new_symbol(library)?,
			connector_delete_symbol: ConnextLibrary::load_connector_delete_symbol(library)?,
			reader_new_symbol: ConnextLibrary::load_reader_new_symbol(library)?,
			writer_new_symbol: ConnextLibrary::load_writer_new_symbol(library)?,
			reader_wait_symbol: ConnextLibrary::load_reader_wait_symbol(library)?,
			writer_clear_symbol: ConnextLibrary::load_writer_clear_symbol(library)?,
			take_symbol: ConnextLibrary::load_take_symbol(library)?,
			read_symbol: ConnextLibrary::load_read_symbol(library)?,
			writer_write_symbol: ConnextLibrary::load_writer_write_symbol(library)?,
			get_samples_length_symbol: ConnextLibrary::load_get_samples_length_symbol(library)?,
			get_json_sample_symbol: ConnextLibrary::load_get_json_sample_symbol(library)?,
			set_json_instance_symbol: ConnextLibrary::load_set_json_instance_symbol(library)?,
			free_string_symbol: ConnextLibrary::load_free_string_symbol(library)?,
		})
	}
}

impl ConnextLibrary<'_> {
	fn load_connector_new_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(CString, CString, i32) -> isize>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_new")?;
		}
		Ok(func)
	}

	fn load_connector_delete_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(isize)>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_delete")?;
		}
		Ok(func)
	}

	fn load_reader_new_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(isize, CString) -> isize>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_getReader")?;
		}
		Ok(func)
	}

	fn load_writer_new_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(isize, CString) -> isize>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_getWriter")?;
		}
		Ok(func)
	}

	fn load_reader_wait_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(isize, i32) -> i32>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnectorReaders_waitForData")?;
		}
		Ok(func)
	}

	fn load_writer_clear_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(isize, CString)>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_clear")?;
		}
		Ok(func)
	}

	fn load_take_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(isize, CString) -> i32>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_take")?;
		}
		Ok(func)
	}

	fn load_read_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(isize, CString) -> i32>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_read")?;
		}
		Ok(func)
	}

	fn load_writer_write_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(isize, CString, CString)>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_write")?;
		}
		Ok(func)
	}

	fn load_get_samples_length_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(isize, CString) -> SamplesLength>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_getSamplesLength")?;
		}
		Ok(func)
	}

	fn load_get_json_sample_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(isize, CString, i32) -> CString>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_getJSONSample")?;
		}
		Ok(func)
	}

	fn load_set_json_instance_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(isize, CString, CString)>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_setJSONInstance")?;
		}
		Ok(func)
	}

	fn load_free_string_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(CString)>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_freeString")?;
		}
		Ok(func)
	} 
}

#[derive(Debug)]
pub enum Entity {
	Participant,
	Reader,
	Writer,
}

#[derive(Debug)]
pub(crate) enum Operation {
	Take,
	Read,
}

mod error {
	use super::{Entity, Operation};
	use std::fmt::Display;

	#[derive(Debug)]
	pub(crate) struct Timeout {
		pub(crate) entity: Entity,
	}

	impl Display for Timeout {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			write!(f, "{:?} wait timed out.", &self.entity)
		}
	}

	impl std::error::Error for Timeout {}

	#[derive(Debug)]
	pub(crate) struct NoData {
		pub(crate) entity: Entity,
		pub(crate) operation: Operation,
	}

	impl Display for NoData {
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			write!(f, "{:?} called on {:?} returned no data.", &self.operation, &self.entity)
		}
	}

	impl std::error::Error for NoData {}
}

#[cfg(test)]
mod tests {
	use super::*;
	use libloading::Library;
	use std::path::Path;

	#[test]
	// Creates a new instance of the ConnextLibrary, loading all symbols from the rtiddsconnector assembly
	fn test_load_symbols() -> Result<()> {
		// FIXME
		let library = Library::new(Path::new("rticonnextdds-connector/lib/x64Darwin16clang8.0/librtiddsconnector.dylib"))?;
		ConnextLibrary::new(&library)?;
		Ok(())
	}
}

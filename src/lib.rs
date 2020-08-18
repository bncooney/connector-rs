use std::{error::Error, fmt::Display, os::raw::c_char};

pub mod connector;
pub mod entity;
pub mod reader;
pub mod writer;

type Result<T> = std::result::Result<T, Box<dyn Error + Sync + Send>>;

use libloading::{Library, Symbol};

type Ptr = isize;
type CString = *const c_char;
type ReturnCode = i32;
type Duration = i32;
type SamplesLength = f64; // TODO: Look into why "samples length" is a double

#[derive(Debug)]
pub struct ConnextLibrary<'lib> {
	connector_new_symbol: Symbol<'lib, unsafe extern "C" fn(config_name: CString, config_file: CString, config: i32) -> Ptr>,
	connector_delete_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: Ptr)>,
	reader_new_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: Ptr, entity_name: CString) -> Ptr>,
	reader_wait_symbol: Symbol<'lib, unsafe extern "C" fn(reader_handle: Ptr, timeout: Duration) -> ReturnCode>,
	take_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: Ptr, entity_name: CString) -> ReturnCode>,
	read_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: Ptr, entity_name: CString) -> ReturnCode>,
	get_samples_length_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: Ptr, entity_name: CString) -> SamplesLength>,
	get_json_sample_symbol: Symbol<'lib, unsafe extern "C" fn(connector_handle: Ptr, entity_name: CString, index: i32) -> CString>,
}

impl<'lib> ConnextLibrary<'lib> {
	pub fn new(library: &'lib Library) -> Result<Self> {
		Ok(ConnextLibrary {
			connector_new_symbol: ConnextLibrary::load_connector_new_symbol(library)?,
			connector_delete_symbol: ConnextLibrary::load_connector_delete_symbol(library)?,
			reader_new_symbol: ConnextLibrary::load_reader_new_symbol(library)?,
			reader_wait_symbol: ConnextLibrary::load_reader_wait_symbol(library)?,
			take_symbol: ConnextLibrary::load_take_symbol(library)?,
			read_symbol: ConnextLibrary::load_read_symbol(library)?,
			get_samples_length_symbol: ConnextLibrary::load_get_samples_length_symbol(library)?,
			get_json_sample_symbol: ConnextLibrary::load_get_json_sample_symbol(library)?,
		})
	}
}

impl ConnextLibrary<'_> {
	fn load_connector_new_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(CString, CString, i32) -> Ptr>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_new")?;
		}
		return Ok(func);
	}

	fn load_connector_delete_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(Ptr)>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_delete")?;
		}
		return Ok(func);
	}

	fn load_reader_new_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(Ptr, CString) -> Ptr>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_getReader")?;
		}
		return Ok(func);
	}

	fn load_reader_wait_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(Ptr, Duration) -> ReturnCode>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnectorReaders_waitForData")?;
		}
		return Ok(func);
	}
	fn load_take_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(Ptr, CString) -> ReturnCode>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_take")?;
		}
		return Ok(func);
	}

	fn load_read_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(Ptr, CString) -> ReturnCode>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_read")?;
		}
		return Ok(func);
	}

	fn load_get_samples_length_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(Ptr, CString) -> SamplesLength>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_getSamplesLength")?;
		}
		return Ok(func);
	}

	fn load_get_json_sample_symbol(library: &Library) -> Result<Symbol<unsafe extern "C" fn(Ptr, CString, i32) -> CString>> {
		let func;
		unsafe {
			func = library.get(b"RTIDDSConnector_getJSONSample")?;
		}
		return Ok(func);
	}
}

#[derive(Debug)]
pub(crate) enum Entity {
	_Connector,
	Reader,
	_Writer,
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

impl Error for Timeout {}

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

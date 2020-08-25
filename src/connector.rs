use num_traits::FromPrimitive;
use std::{
	ffi::{CStr, CString},
	os::raw::c_char,
};

use super::{
	entity::Entity,
	error::NoData,
	reader::{Reader, ReturnCode},
	writer::Writer,
	ConnextLibrary, Entity as EntityType, Result, Operation,
};

#[derive(Debug)]
pub struct Connector<'lib> {
	library: &'lib ConnextLibrary<'lib>,
	pub(crate) connector_handle: isize,
	config_name: CString,
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

		Ok(Self {
			library,
			connector_handle,
			config_name,
		})
	}
}

impl Entity for Connector<'_> {
	fn entity_name(&self) -> CString {
		self.config_name.to_owned()
	}
	fn entity_type() -> EntityType {
		EntityType::Participant
	}
}

impl Connector<'_> {
	pub fn clear(&self, writer: &Writer) {
		let func = &self.library.writer_clear_symbol;
		unsafe {
			func(self.connector_handle, writer.entity_name.as_ptr());
		}
	}

	pub fn write(&self, writer: &Writer) {
		// TODO: Does write need to free written string? Check js example
		let func = &self.library.writer_write_symbol;
		unsafe {
			func(self.connector_handle, writer.entity_name.as_ptr(), std::ptr::null());
		}
	}

	pub fn take(&self, reader: &Reader) -> Result<()> {
		self.next(reader, Operation::Take)
	}

	pub fn read(&self, reader: &Reader) -> Result<()> {
		self.next(reader, Operation::Read)
	}

	fn next(&self, reader: &Reader, operation: Operation) -> Result<()> {
		let func = match operation {
			Operation::Take => &self.library.take_symbol,
			Operation::Read => &self.library.read_symbol,
		};

		let return_code: i32;

		unsafe {
			return_code = func(self.connector_handle, reader.entity_name.as_ptr());
		}

		match ReturnCode::from_i32(return_code) {
			Some(ReturnCode::Ok) => Ok(()),
			Some(ReturnCode::NoData) => {
				Err(NoData {
					entity: EntityType::Reader,
					operation,
				}
				.into())
			} //TODO: Log this as a logic error
			_ => Err(format!("{}:{}", "Unexpected error occured in Connector::take", return_code).into()),
		}
	}

	pub fn get_samples_length(&self, reader: &Reader) -> Result<f64> {
		let get_samples_length = &self.library.get_samples_length_symbol;
		let samples: f64;
		unsafe {
			// TODO: Replace with "checked" version from js? (RTI_Connector_get_sample_count)
			samples = get_samples_length(self.connector_handle, reader.entity_name.as_ptr());
		}
		Ok(samples)
	}

	pub fn get_json_sample(&self, reader: &Reader, index: i32) -> Result<String> {
		if index < 1 {
			return Err("Connext sample index start at 1, ".into());
		}

		let get_json_sample = &self.library.get_json_sample_symbol;
		let sample_ptr: *const c_char;
		let sample: String;

		unsafe {
			sample_ptr = get_json_sample(self.connector_handle, reader.entity_name.as_ptr(), index);
			sample = CStr::from_ptr(sample_ptr).to_string_lossy().into_owned();
		}

		self.free_string(sample_ptr);

		Ok(sample)
	}

	pub fn set_json_instance(&self, writer: &Writer, json: &str) -> Result<()> {
		let json = CString::new(json)?;
		let json_ptr = json.as_ptr(); // Bind to local as per as_ptr() warning to manage pointer lifetime
		let func = &self.library.set_json_instance_symbol;
		unsafe {
			func(self.connector_handle, writer.entity_name.as_ptr(), json_ptr);
		}
		Ok(())
	}

	fn free_string(&self, string_ptr: *const c_char) {
		let func = &self.library.free_string_symbol;

		unsafe {
			func(string_ptr);
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

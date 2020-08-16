use std::{convert::TryInto, ffi::{CString, CStr}, time::Duration, os::raw::c_char};

use super::{connector::Connector, ConnextLibrary, Entity, Ptr, Result, Timeout, NULL_PTR};

use num_traits::FromPrimitive;

#[derive(FromPrimitive, ToPrimitive)]
pub enum ReturnCode {
	Ok = 0,
	Timeout = 10,
	NoData = 11,
}

#[derive(Debug)]
pub struct Reader<'library, 'connector> {
	connector: &'connector Connector<'connector>,
	entity_name: CString,
	library: &'library ConnextLibrary<'library>,
	reader_handle: Ptr,
}

impl<'library, 'connector> Reader<'library, 'connector> {
	pub fn new(library: &'library ConnextLibrary, connector: &'connector Connector, entity_name: &str) -> Result<Self> {
		let entity_name_cstring = CString::new(entity_name)?;
		let reader_new = &library.reader_new_symbol;
		let reader_handle: Ptr;

		unsafe {
			reader_handle = reader_new(connector.connector_handle, entity_name_cstring.as_ptr());
		}
		if reader_handle == NULL_PTR {
			return Err(format!("Couldnt create reader, {}", entity_name).into());
		}

		Ok(Self {
			connector,
			entity_name: entity_name_cstring,
			library,
			reader_handle,
		})
	}

	pub fn wait(&self, timeout: Option<Duration>) -> Result<()> {
		let timeout_millis: i32;
		match timeout {
			Some(x) => timeout_millis = x.as_millis().try_into().unwrap_or(std::i32::MAX),
			None => timeout_millis = -1, // -1, infinite duration
		}

		let return_code: i32;
		let reader_wait = &self.library.reader_wait_symbol;

		unsafe {
			return_code = reader_wait(self.reader_handle, timeout_millis);
		}

		match ReturnCode::from_i32(return_code) {
			Some(ReturnCode::Ok) => return Ok(()),
			Some(ReturnCode::Timeout) => return Err(Box::new(Timeout { entity: Entity::Reader })),
			_ => return Err(format!("{}:{}", "Unexpected error occured in Reader::wait", return_code).into()),
		}
	}

	pub fn take(&self) -> Result<()> {
		self.next(SampleOperation::Take)
	}

	pub fn read(&self) -> Result<()> {
		self.next(SampleOperation::Read)
	}

	// TODO: Consider name with respect to calling convention, may be better placed to return Vec<Sample>
	fn next(&self, operation: SampleOperation) -> Result<()> {
		let operation = match operation {
			SampleOperation::Take => &self.library.take_symbol,
			SampleOperation::Read => &self.library.read_symbol,
		};
		let return_code: i32;
		unsafe {
			return_code = operation(self.connector.connector_handle, self.entity_name.as_ptr());
		}
		match ReturnCode::from_i32(return_code) {
			Some(ReturnCode::Ok) => return Ok(()),
			Some(ReturnCode::NoData) => return Ok(()), //TODO: Log this as a logical error
			_ => return Err(format!("{}:{}", "Unexpected error occured in Reader::next", return_code).into()),
		}
	}

	pub fn get_samples_length(&self) -> Result<f64> {
		let get_samples_length = &self.library.get_samples_length_symbol;
		let samples: f64;
		unsafe {
			// TODO: Replace with "checked" version from js? (RTI_Connector_get_sample_count)
			samples = get_samples_length(self.connector.connector_handle, self.entity_name.as_ptr());
		}
		Ok(samples)
	}

	// TODO: Return json object from Serde, or introduce this is a "friendlier" layer?
	pub fn get_json_sample(&self, index: i32) -> Result<String> {
		if index < 1 {
			return Err(format!("{}", "Connext sample index start at 1, "))
		}

		// TODO: Decide where to introduce "safety", segfault will occur on this operation if the index > samples_length
		// Should this crate provide a direct implementation of the API, or should it hide internals and do bounds checking?

		let get_json_sample = &self.library.get_json_sample_symbol;
		let sample_ptr: *const c_char;
		let sample: String;

		unsafe {
			// TODO: Replace with "checked" version from js?
			sample_ptr = get_json_sample(self.connector.connector_handle, self.entity_name.as_ptr(), index);
			sample = CStr::from_ptr(sample_ptr).to_string_lossy().into_owned();
		}
		
		Ok(sample)
	}
}

impl<'library, 'connector> PartialEq for Reader<'library, 'connector> {
	fn eq(&self, other: &Self) -> bool {
		self.reader_handle == other.reader_handle
	}
}

impl<'library, 'connector> Eq for Reader<'library, 'connector> {}

enum SampleOperation {
	Take,
	Read,
}

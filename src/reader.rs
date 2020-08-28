use std::{convert::TryInto, ffi::CString, fmt, time::Duration};

use super::{connector::Connector, entity::Entity, error::Timeout, ConnextLibrary, Result};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum ReturnCode {
	Ok = 0,
	Error = 1,
	Unsupported = 2,
	BadParameter = 3,
	PreconditionNotMet = 4,
	OutOfResources = 5,
	NotEnabled = 6,
	InmutablePolicy = 7,
	InconsistencyPolicy = 8,
	AlreadyDeleted = 9,
	Timeout = 10,
	NoData = 11,
	IllegalOperation = 12,
	NotAllowedBySec = 13,
}

impl fmt::Display for ReturnCode {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug)]
pub struct Reader<'lib> {
	pub(crate) entity_name: CString,
	library: &'lib ConnextLibrary<'lib>,
	handle: isize,
}

impl Entity for Reader<'_> {
	fn entity_name(&self) -> CString {
		self.entity_name.to_owned()
	}
}

impl<'lib> Reader<'lib> {
	pub fn new(library: &'lib ConnextLibrary, connector: &Connector, entity_name: &str) -> Result<Self> {
		let entity_name = CString::new(entity_name)?;
		let func = &library.reader_new_symbol;
		let handle: isize;

		unsafe {
			handle = func(connector.connector_handle, entity_name.as_ptr());
		}
		if handle == 0 {
			// Safe to unwrap, &str -> CString -> &str conversion
			Err(format!("Couldnt create reader, {}", entity_name.to_str().unwrap()).into())
		} else {
			Ok(Self { entity_name, library, handle })
		}
	}
}

impl Reader<'_> {
	pub fn entity_name_str(&self) -> &str {
		// Safe to unwrap, &str -> CString -> &str conversion
		&self.entity_name.to_str().unwrap()
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
			return_code = reader_wait(self.handle, timeout_millis);
		}

		match ReturnCode::from_i32(return_code) {
			Some(ReturnCode::Ok) => Ok(()),
			Some(ReturnCode::Timeout) => Err(Timeout {}.into()),
			x => Err(format!(
				"{} wait returned {}",
				&self.entity_name_str(),
				match x {
					Some(x) => x.to_string(),
					None => format!("ReturnCode::{}", return_code),
				}
			)
			.into()),
		}
	}
}

impl PartialEq for Reader<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.handle == other.handle
	}
}

impl Eq for Reader<'_> {}

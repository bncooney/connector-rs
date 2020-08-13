use std::{
	convert::TryInto,
	ffi::{CStr, CString},
	fmt::Display,
	os::raw::c_char,
	path::Path,
	time::{Duration, Instant},
};

#[macro_use]
extern crate num_derive;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use libloading::{Library, Symbol};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Timeout;

impl Display for Timeout {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		// TODO: Add subject enum (Connector / Participant, Reader)?
		write!(f, "{}", "Connector wait timed out.")
	}
}

impl std::error::Error for Timeout {}

#[derive(FromPrimitive, ToPrimitive)]
enum ReturnCode {
	Ok = 0,
	Timeout = 10,
	NoData = 11,
}

#[derive(Debug)]
pub struct Connector {
	library: Library,
	connector_handle: isize,
}

impl Connector {
	pub fn new(config_name: &str, config_file: &str) -> Result<Self> {
		let library = load_connector_library()?;
		let connector_handle = connector_new(&library, config_name, config_file)?;

		return Ok(Connector {
			library,
			connector_handle,
		});

		fn load_connector_library() -> Result<Library> {
			let library_path;

			match std::env::consts::OS {
				"windows" => {
					library_path =
						Path::new("rticonnextdds-connector/lib/x64Win64VS2013/rtiddsconnector.dll")
				}
				"macos" => {
					library_path = Path::new(
						"rticonnextdds-connector/lib/x64Darwin16clang8.0/librtiddsconnector.dylib",
					)
				}
				"linux" => {
					library_path = Path::new(
						"rticonnextdds-connector/lib/x64Linux2.6gcc4.4.5/librtiddsconnector.so",
					)
				}
				"android" => library_path = Path::new(
					"rticonnextdds-connector/lib/armv6vfphLinux3.xgcc4.7.2/librtiddsconnector.so",
				),
				_ => panic!("Current platform not supported."),
			}

			Ok(Library::new(&library_path)?)
		}

		fn connector_new(library: &Library, config_name: &str, config_file: &str) -> Result<isize> {
			let connector_handle: isize;

			unsafe {
				let func: Symbol<
					unsafe extern "C" fn(
						config_name: *const c_char,
						config_file: *const c_char,
						config: isize,
					) -> isize,
				> = library.get(b"RTIDDSConnector_new")?;

				connector_handle = func(
					CString::new(config_name)?.as_ptr(),
					CString::new(config_file)?.as_ptr(),
					0,
				);
			}

			if connector_handle == 0 {
				return Err("Couldn't create RTI DDS connection, see stderr.".into());
			}

			Ok(connector_handle)
		}
	}

	fn _wait(
		&self,
		timeout: Option<Duration>,
	) -> std::result::Result<(), Box<dyn std::error::Error>> {
		let timeout_millis: i32;

		match timeout {
			Some(x) => timeout_millis = x.as_millis().try_into().unwrap_or(std::i32::MAX),
			None => timeout_millis = -1,
		}

		let return_code: i32;

		unsafe {
			let func: Symbol<
				unsafe extern "C" fn(connector_handle: isize, timeout_millis: i32) -> i32,
			> = self.library.get(b"RTIDDSConnector_wait").unwrap();
			return_code = func(self.connector_handle, timeout_millis)
		}

		match ReturnCode::from_i32(return_code) {
			Some(ReturnCode::Ok) => return Ok(()),
			Some(ReturnCode::Timeout) => return Err(Box::new(Timeout)),
			_ => return Err("Unexpected error occured in Connector::wait".into()),
		}
	}
}

impl Drop for Connector {
	fn drop(&mut self) {
		unsafe {
			let func: Symbol<unsafe extern "C" fn(connector_handle: isize)> =
				self.library.get(b"RTIDDSConnector_delete").unwrap();
			func(self.connector_handle);
		}
	}
}

#[derive(Debug)]
pub struct Reader {
	reader_handle: isize,
}

impl PartialEq for Reader {
	fn eq(&self, other: &Self) -> bool {
		self.reader_handle == other.reader_handle
	}
}

impl Eq for Reader {}

#[derive(Debug)]
pub struct Writer {
	writer_handle: isize,
}

impl PartialEq for Writer {
	fn eq(&self, other: &Self) -> bool {
		self.writer_handle == other.writer_handle
	}
}

impl Eq for Writer {}

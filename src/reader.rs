use std::{
	ffi::CString
};

use super::{ConnextLibrary, Result, connector::Connector};

#[derive(Debug)]
pub struct Reader<'library> {
    library: &'library ConnextLibrary<'library>,
	reader_handle: isize,
}

impl<'library> Reader<'library> {
	pub fn new(library: &'library ConnextLibrary, connector: &Connector, entity_name: &str) -> Result<Self> {
        let entity_name = CString::new(entity_name)?;
		let reader_new = &library.reader_new_symbol;

        let reader_handle: isize;
		unsafe {
			reader_handle = reader_new(connector.connector_handle, entity_name.as_ptr());
        }
        
		if reader_handle == 0 {
            // Safe to unwrap, &str -> CString -> &str conversion
			return Err(format!("Couldnt create reader, {}", entity_name.to_str().unwrap()).into());
		}

		Ok(Self { library, reader_handle })
	}
}

impl<'library> Drop for Reader<'library> {
	fn drop(&mut self) {
		// let fn_connector_delete = &self.library.connector_delete_symbol;
		unsafe {
			// fn_connector_delete(self.connector_handle);
		}
	}
}

impl<'library> PartialEq for Reader<'library> {
	fn eq(&self, other: &Self) -> bool {
		self.reader_handle == other.reader_handle
	}
}

impl<'library> Eq for Reader<'library> {}
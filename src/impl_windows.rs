use crate::*;
use std::{ffi::{OsStr, OsString}, os::windows::ffi::{OsStrExt, OsStringExt}, slice};



impl ToStableEncoding for OsString {
	fn to_stable_encoding(&self) -> Vec<u8> {
		self.as_os_str().to_stable_encoding()
	}
}

impl ToStableEncoding for OsStr {
	fn to_stable_encoding(&self) -> Vec<u8> {
		let encoded = self.encode_wide().collect::<Vec<_>>();
		let (data, len, cap) = encoded.into_raw_parts();
		unsafe {
			let (data, len, cap) = (data as *mut u8, len * 2, cap * 2);
			Vec::from_raw_parts(data, len, cap)
		}
	}
}



impl IntoStableEncoding for OsString {
	fn into_stable_encoding(self) -> Vec<u8> {
		let encoded = self.encode_wide().collect::<Vec<_>>();
		let (data, len, cap) = encoded.into_raw_parts();
		unsafe {
			let (data, len, cap) = (data as *mut u8, len * 2, cap * 2);
			Vec::from_raw_parts(data, len, cap)
		}
	}
}


impl FromStableEncoding for OsString {
	fn from_stable_encoding<'a>(encoded: impl Into<Cow<'a, [u8]>>) -> Self {
		let encoded = encoded.into();
		let encoded = &*encoded;
		let (data, len) = (encoded.as_ptr(), encoded.len());
		unsafe {
			let encoded = slice::from_raw_parts(data as *const u16, len / 2);
			OsString::from_wide(encoded)
		}
	}
}

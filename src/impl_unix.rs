use crate::*;
use std::{ffi::{OsStr, OsString}, os::unix::ffi::{OsStrExt, OsStringExt}};



impl ToStableEncoding for OsString {
	fn to_stable_encoding(&self) -> Vec<u8> {
		self.as_os_str().to_stable_encoding()
	}
}

impl ToStableEncoding for OsStr {
	fn to_stable_encoding(&self) -> Vec<u8> {
		self.as_bytes().to_vec()
	}
}



impl IntoStableEncoding for OsString {
	fn into_stable_encoding(self) -> Vec<u8> {
		self.into_vec()
	}
}



impl FromStableEncoding for OsString {
	fn from_stable_encoding(encoded: impl Into<Vec<u8>>) -> Self {
		OsString::from_vec(encoded.into())
	}
}

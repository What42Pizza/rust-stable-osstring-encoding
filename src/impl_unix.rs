use std::{ffi::{OsStr, OsString}, os::unix::ffi::{OsStrExt, OsStringExt}};



pub trait ToStableEncoding {
	fn to_stable_encoding(&self) -> Vec<u8>;
}

impl ToStableEncoding for OsString {
	fn to_stable_encoding(&self) -> Vec<u8> {
		self.as_bytes().to_vec()
	}
}

impl ToStableEncoding for OsStr {
	fn to_stable_encoding(&self) -> Vec<u8> {
		self.as_bytes().to_vec()
	}
}



pub trait IntoStableEncoding {
	fn into_stable_encoding(self) -> Vec<u8>;
}

impl IntoStableEncoding for OsString {
	fn into_stable_encoding(self) -> Vec<u8> {
		self.into_vec()
	}
}



pub trait FromStableEncoding {
	fn from_stable_encoding(encoded: impl Into<Vec<u8>>) -> Self;
}

impl FromStableEncoding for OsString {
	fn from_stable_encoding(encoded: impl Into<Vec<u8>>) -> Self {
		OsString::from_vec(encoded.into())
	}
}

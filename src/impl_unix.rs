use crate::*;
use std::{
	ffi::{OsStr, OsString},
	os::unix::ffi::{OsStrExt, OsStringExt},
};



impl ToStableEncoding for OsString {
	fn to_stable_encoding(&self) -> StableOsString {
		self.as_os_str().to_stable_encoding()
	}
}

impl ToStableEncoding for OsStr {
	fn to_stable_encoding(&self) -> StableOsString {
		self.as_bytes().to_vec()
	}
}



impl IntoStableEncoding for OsString {
	fn into_stable_encoding(self) -> StableOsString {
		self.into_vec()
	}
}



impl FromStableEncoding for OsString {
	unsafe fn from_stable_encoding<'a>(encoded: impl Into<Cow<'a, [EncodingWidth]>>) -> Self {
		OsString::from_vec(encoded.into().into_owned())
	}
}

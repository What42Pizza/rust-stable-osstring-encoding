use crate::*;
use std::{ffi::{OsStr, OsString}, os::windows::ffi::{OsStrExt, OsStringExt}, slice};



impl ToStableEncoding for OsString {
	fn to_stable_encoding(&self) -> StableOsString {
		self.as_os_str().to_stable_encoding()
	}
}

impl ToStableEncoding for OsStr {
	fn to_stable_encoding(&self) -> StableOsString {
		self.encode_wide().collect::<Vec<_>>()
	}
}



impl IntoStableEncoding for OsString {
	fn into_stable_encoding(self) -> StableOsString {
		self.as_os_str().to_stable_encoding()
	}
}



impl FromStableEncoding for OsString {
	fn from_stable_encoding<'a>(encoded: impl Into<Cow<'a, [EncodingWidth]>>) -> Self {
		let encoded = encoded.into();
		let encoded = &*encoded;
		OsString::from_wide(encoded)
	}
}

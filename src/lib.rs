#![warn(missing_docs, clippy::all)]

#![doc = include_str!("../readme.md")]



use std::borrow::Cow;



/// Contains the implementation for unix builds, including Linux and Macos
#[cfg(unix)]
pub mod impl_unix;
/// Contains the implementation for windows builds
#[cfg(windows)]
pub mod impl_windows;



/// Allows converting an `OsString` or `OsStr` to a stable encoding
pub trait ToStableEncoding {
	/// Allows converting an `OsString` or `OsStr` to a stable encoding
	fn to_stable_encoding(&self) -> Vec<u8>;
}

/// Allows converting an `OsString` to a stable encoding, bypassing data copies if possible
pub trait IntoStableEncoding {
	/// Allows converting an `OsString` to a stable encoding, bypassing data copies if possible
	fn into_stable_encoding(self) -> Vec<u8>;
}

/// Allows converting an encoded string back into an `OsString`, bypassing data copies if possible
pub trait FromStableEncoding {
	/// Allows converting an encoded string back into an `OsString`, bypassing data copies if possible
	fn from_stable_encoding<'a>(encoded: impl Into<Cow<'a, [u8]>>) -> Self;
}



#[cfg(test)]
mod test {
	use std::ffi::OsString;
	use crate::{FromStableEncoding, IntoStableEncoding, ToStableEncoding};
	
	#[test]
	fn basics() {
		let start = OsString::from("test");
		let as_stable_1 = start.to_stable_encoding();
		let as_stable_2 = start.into_stable_encoding();
		assert_eq!(as_stable_1, as_stable_2);
		let as_stable = &*as_stable_2; // make sure &[u8] can be given to from_stable_encoding()
		let as_os_string = OsString::from_stable_encoding(as_stable);
		let as_str = as_os_string.to_str();
		assert_eq!(as_str, Some("test"));
	}
	
}

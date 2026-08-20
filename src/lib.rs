#![warn(missing_docs, clippy::all)]
#![doc = include_str!("../readme.md")]



#[cfg(not(any(unix, windows)))]
compile_error!(
	"This crate currently only supports Windows and Unix (Linux and Macos). Adding support for your platform is likely very easy, please consider opening an issue for it in \"stable-osstring-encoding\"'s issue tracker."
);



use std::borrow::Cow;



/// Contains the implementation for unix builds, including Linux and Macos
#[cfg(unix)]
pub mod impl_unix;
/// Contains the implementation for windows builds
#[cfg(windows)]
pub mod impl_windows;



/// Defines the encoding width
#[cfg(unix)]
pub type EncodingWidth = u8;
/// Defines the encoding width
#[cfg(windows)]
pub type EncodingWidth = u16;

/// A simple alias for `Vec<EncodingWidth>`
pub type StableOsString = Vec<EncodingWidth>;



/// Converts an `OsString` or `OsStr` to an encoding that is stable across rust compiler versions
pub trait ToStableEncoding {
	/// Converts an `OsString` or `OsStr` to an encoding that is stable across rust compiler versions
	fn to_stable_encoding(&self) -> StableOsString;
}

/// Converts an `OsString` from an encoding that is stable across rust compiler versions, bypassing data copies if possible
pub trait IntoStableEncoding {
	/// Converts an `OsString` from an encoding that is stable across rust compiler versions, bypassing data copies if possible
	fn into_stable_encoding(self) -> StableOsString;
}

/// Converts an `OsString` from an encoding that is stable across rust compiler versions, bypassing data copies if possible
pub trait FromStableEncoding {
	/// Converts an `OsString` from an encoding that is stable across rust compiler versions, bypassing data copies if possible
	fn from_stable_encoding<'a>(encoded: impl Into<Cow<'a, [EncodingWidth]>>) -> Self;
}



#[cfg(test)]
mod test {
	use crate::{FromStableEncoding, IntoStableEncoding, ToStableEncoding};
	use std::ffi::OsString;

	#[test]
	fn basics() {
		let start = OsString::from("test");
		let as_stable_1 = start.to_stable_encoding();
		let as_stable_2 = start.into_stable_encoding();
		assert_eq!(as_stable_1, as_stable_2);

		let as_stable_1 = &*as_stable_1; // make sure &[EncodingWidth] can be given to from_stable_encoding()

		let as_os_string_1 = OsString::from_stable_encoding(as_stable_1);
		let as_os_string_2 = OsString::from_stable_encoding(as_stable_2);
		assert_eq!(as_os_string_1, as_os_string_2);

		let as_str = as_os_string_1.to_str();
		assert_eq!(as_str, Some("test"));
	}
}

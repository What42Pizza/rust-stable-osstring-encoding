#![warn(missing_docs, clippy::all)]
#![doc = include_str!("../readme.md")]



#[cfg(not(any(unix, windows)))]
compile_error!(
	"This crate currently only supports Windows and Unix (Linux and Macos). Adding support for your platform is likely very easy, please consider opening an issue for it in \"stable-osstring-encoding\"'s issue tracker."
);



use std::{
	borrow::Cow,
	ffi::OsString,
	path::{Path, PathBuf},
};



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

/// Crates an `OsString` from an encoding that is stable across rust compiler versions, bypassing data copies if possible
pub trait FromStableEncoding {
	/// Converts an `OsString` from an encoding that is stable across rust compiler versions, bypassing data copies if possible
	/// 
	/// This takes `Into<Cow<[EncodingWidth]>>` so that either a slice can be passed (which always allocates and copies data) or a vec can be passed (which might be able to skip allocating and copying data)
	///
	/// # Safety
	///
	/// The given bytes must be compatible with the underlying of the platform's `OsStr` encoding (reminder: this crate only make it safe to pass data between different rust versions)
	unsafe fn from_stable_encoding<'a>(encoded: impl Into<Cow<'a, [EncodingWidth]>>) -> Self;
}



impl ToStableEncoding for Path {
	fn to_stable_encoding(&self) -> StableOsString {
		self.as_os_str().to_stable_encoding()
	}
}

impl ToStableEncoding for PathBuf {
	fn to_stable_encoding(&self) -> StableOsString {
		self.as_os_str().to_stable_encoding()
	}
}

impl IntoStableEncoding for PathBuf {
	fn into_stable_encoding(self) -> StableOsString {
		self.into_os_string().to_stable_encoding()
	}
}

impl FromStableEncoding for PathBuf {
	unsafe fn from_stable_encoding<'a>(encoded: impl Into<Cow<'a, [EncodingWidth]>>) -> Self {
		unsafe { PathBuf::from(OsString::from_stable_encoding(encoded)) }
	}
}

impl<'a, T> IntoStableEncoding for Cow<'a, T>
where
	T: ToStableEncoding + ToOwned,
	<T as ToOwned>::Owned: IntoStableEncoding,
{
	fn into_stable_encoding(self) -> StableOsString {
		match self {
			Cow::Borrowed(v) => v.to_stable_encoding(),
			Cow::Owned(v) => v.into_stable_encoding(),
		}
	}
}



#[cfg(test)]
mod test {
	use crate::{FromStableEncoding, IntoStableEncoding, ToStableEncoding};
	use std::{ffi::OsString, path::PathBuf};

	#[test]
	fn basics() {
		let start = OsString::from("test");
		let as_stable_1 = start.to_stable_encoding();
		let as_stable_2 = start.into_stable_encoding();
		assert_eq!(as_stable_1, as_stable_2);

		let as_stable_1 = &*as_stable_1; // make sure &[EncodingWidth] can be given to from_stable_encoding()

		let as_os_string_1 = unsafe { OsString::from_stable_encoding(as_stable_1) };
		let as_os_string_2 = unsafe { OsString::from_stable_encoding(as_stable_2) };
		assert_eq!(as_os_string_1, as_os_string_2);

		let as_str = as_os_string_1.to_str();
		assert_eq!(as_str, Some("test"));
	}

	#[test]
	fn path_buf() {
		let start = PathBuf::from(OsString::from("test"));
		let as_stable_1 = start.to_stable_encoding();
		let as_stable_2 = start.into_stable_encoding();
		assert_eq!(as_stable_1, as_stable_2);

		let as_stable_1 = &*as_stable_1; // make sure &[EncodingWidth] can be given to from_stable_encoding()

		let as_os_string_1 = unsafe { PathBuf::from_stable_encoding(as_stable_1) };
		let as_os_string_2 = unsafe { PathBuf::from_stable_encoding(as_stable_2) };
		assert_eq!(as_os_string_1, as_os_string_2);

		let as_str = as_os_string_1.to_str();
		assert_eq!(as_str, Some("test"));
	}
}

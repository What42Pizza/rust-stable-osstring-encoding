#[cfg(unix)]
pub mod impl_unix;
#[cfg(unix)]
pub use impl_unix::*;

#[cfg(windows)]
pub mod impl_windows;
#[cfg(windows)]
pub use impl_windows::*;



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

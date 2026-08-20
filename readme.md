# Rust Stable OsString Encoding

This was made for one very specific reason: the encoding of `OsStr::as_encoded_bytes()`'s output might change based on the Rust compiler version the code is built with (according to [this](https://doc.rust-lang.org/std/ffi/struct.OsStr.html#safety) comment). If you want the the guarantee that the encoded data's encoding will never change depending on the compiler version you're using, this crate provides methods which convert `OsStr` and `OsString` to and from a `Vec<u8>` or `Vec<u16>` (depending on the platform). This is made possible by the platform-specific `OsStrExt` and `OsStringExt` traits, which give and take data that does have an encoding which the standard library promises will not change.

### Important caveats:

- This is not meant for sending data from one platform to another (e.g. from a windows program to a linux program).
- This currently only supports Windows and Unix (Linux and MacOs), but other platforms should be easy to implement if needed.

## Example:

```rust
use std::ffi::OsString;
use stable_osstring_encoding::*;

let my_string = OsString::from("My String");
let stable_string: Vec<_> = my_string.into_stable_encoding();
// send stable_string to another program

// in another program:
let my_string = unsafe { OsString::from_stable_encoding(stable_string) };
```

### Provided methods:

```rust,ignore
// note: `StableOsString` is just an alias for `Vec<u8>` or `Vec<u16>` (depending on the platform)
OsStr::to_stable_encoding(&self) -> StableOsString
OsString::to_stable_encoding(&self) -> StableOsString
OsString::into_stable_encoding(self) -> StableOsString
OsString::from_stable_encoding(encoded: Into<Cow<[EncodingWidth]>>) -> OsString
```

# Rust Stable OsString Encoding

This was made for one very specific reason: the encoding of `OsStr::as_encoded_bytes()`'s output might change based on the Rust compiler version the code is built with (according to [this](https://doc.rust-lang.org/std/ffi/struct.OsStr.html#safety) comment). If you want the guarantee that the data's encoding will never change depending on the compiler version you're using, this crate provides stable conversions between `OsStr`/`OsString` and `Vec<EncodingWidth>` (where `EncodingWidth` is with `u8` or `u16` depending on the platform).

This is made possible by the platform-specific `OsStrExt` and `OsStringExt` traits, which deal with standardized encodings (unlike `OsStr::as_encoded_bytes()`).

### Important caveats:

- This is not meant for sending data from one platform to another (e.g. from a windows program to a linux program).
- This currently only supports Windows and Unix (Linux, MacOs, and more), but other platforms should be easy to implement if needed.

## Example:

```rust
use std::ffi::OsString;
use stable_osstring_encoding::*;

let my_string = OsString::from("My String");
let stable_string: Vec<_> = my_string.into_stable_encoding();
// send stable_string to another program

// in another program:
let my_string = unsafe { OsString::from_stable_encoding(stable_string) }; // (reminder: this crate only make it safe to pass data between different rust versions, but ensuring that the encoded data is not mutated incorrectly is still up to the programmer)
```

### Provided methods:

Note: `StableOsString` is just an alias for `Vec<EncodingWidth>`, and `EncodingWidth` is an alias for either `u8` or `u16` (depending on the platform)

The method `to_stable_encoding(&self) -> StableOsString` is implemented for:

- `OsStr`
- `OsString`
- `Path`
- `PathBuf`

The method `into_stable_encoding(self) -> StableOsString` (avoid allocation when possible) is implemented for:

- `OsString` 
- `PathBuf` 
- `Cow<T>` where `T` and its owned variant implement `ToStableEncoding` and `IntoStableEncoding` respectively

The method `from_stable_encoding(Into<Cow<EncodingWidth>>)` is implemented for:

- `OsString`
- `PathBuf`

### Platform-specific details:

On Windows, the encoding is the output of `std::os::windows::ffi::OsStrExt::encode_wide()`, which is utf-16, and possible malformed. And as is required, turning this output back into an `OsString` is completely lossless.

On Unix (Linux, MacOs, etc), the encoding is the output of `std::os::unix::ffi::OsStrExt::as_bytes()`, which can be any arbitrary data. And as is required, turning this output back into an `OsString` is completely lossless.

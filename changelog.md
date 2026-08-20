- v1.1.2 (26/08/20)
  - Added `ToStableEncoding` implementation for `Path` and `PathBuf`
  - Added `IntoStableEncoding` implementation for `PathBuf`
  - Added `FromStableEncoding` implementation for `PathBuf`
  - Added `IntoStableEncoding` implementation for `Cow<T>` where `T: ToStableEncoding` and `<T as ToOwned>::Owned: IntoStableEncoding`
  - Replaced `IntoStableEncoding` implementation for `Cow<OsStr>` with implementation for `Cow<T>`

<br>

- v1.1.1 (26/08/19)
  - Added `IntoStableEncoding` implementation for `Cow<OsStr>`

<br>

- v1.1.0 (26/08/19)
  - Marked `OsString::from_stable_encoding` as unsafe

<br>

- v1.0.0 (26/08/19)
  - Initial release

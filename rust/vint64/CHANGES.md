## [0.3.0] (2020-02-28)

- Add `encoded_len()`/`decoded_len()` helpers; loop-free encoder ([#103])
- Use an `enum` for the `Error` type ([#102])
- Move `signed` and `zigzag` encoders into their own modules ([#101])

[0.3.0]: https://github.com/iqlusioninc/veriform/pull/104
[#103]: https://github.com/iqlusioninc/veriform/pull/103
[#102]: https://github.com/iqlusioninc/veriform/pull/102
[#101]: https://github.com/iqlusioninc/veriform/pull/101

## [0.2.1] (2020-02-25)

- Add `encode_zigzag` and `decode_zigzag` functions ([#76])

[0.2.1]: https://github.com/iqlusioninc/veriform/pull/82
[#76]: https://github.com/iqlusioninc/veriform/pull/76

## [0.2.0] (2020-02-14)

- Add `VInt64::len()` method ([#63])
- Rename `Vint64` -> `VInt64` ([#62])
- Relocate crate into the Veriform git repo ([#61])

[0.2.0]: https://github.com/iqlusioninc/veriform/pull/64
[#63]: https://github.com/iqlusioninc/veriform/pull/63
[#62]: https://github.com/iqlusioninc/veriform/pull/62
[#61]: https://github.com/iqlusioninc/veriform/pull/61

## 0.1.2 (2020-01-31)

- More documentation improvements

## 0.1.1 (2020-01-30)

- Documentation improvements

## 0.1.0 (2020-01-30)

- Initial release

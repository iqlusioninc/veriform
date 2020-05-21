# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 (2020-05-21)
### Added
- Initial Verihash "Merkleized" message hashing support ([#120], [#124], [#125], [#128], [#145])
- Custom derive support for `veriform::Message` ([#131], [#135], [#136], [#141])
- `Decode*` traits ([#117])
- `log` feature for tracing message decoding ([#117])
- Builtin types: `Timestamp` and `Uuid` ([#98])

### Changed
- Split error type into `veriform::error::{Error, Kind}` ([#127], [#129])
- Ensure encoded/decoded strings are ASCII ([#146])
- Make encoding result immutable ([#105])
- MSRV 1.40+ ([#91])
- Extract `Decodable` trait ([#84])

### Fixed
- Sequence decoding ([#86], [#96])

[#146]: https://github.com/iqlusioninc/veriform/pull/146
[#145]: https://github.com/iqlusioninc/veriform/pull/145
[#141]: https://github.com/iqlusioninc/veriform/pull/141
[#136]: https://github.com/iqlusioninc/veriform/pull/136
[#135]: https://github.com/iqlusioninc/veriform/pull/135
[#131]: https://github.com/iqlusioninc/veriform/pull/131
[#129]: https://github.com/iqlusioninc/veriform/pull/129
[#128]: https://github.com/iqlusioninc/veriform/pull/128
[#127]: https://github.com/iqlusioninc/veriform/pull/127
[#125]: https://github.com/iqlusioninc/veriform/pull/125
[#124]: https://github.com/iqlusioninc/veriform/pull/124
[#120]: https://github.com/iqlusioninc/veriform/pull/120
[#117]: https://github.com/iqlusioninc/veriform/pull/117
[#105]: https://github.com/iqlusioninc/veriform/pull/105
[#98]: https://github.com/iqlusioninc/veriform/pull/98
[#96]: https://github.com/iqlusioninc/veriform/pull/96
[#91]: https://github.com/iqlusioninc/veriform/pull/91
[#86]: https://github.com/iqlusioninc/veriform/pull/86
[#84]: https://github.com/iqlusioninc/veriform/pull/84

### Fixed
- Sequence decoding ([#86], [#96])

## 0.0.1 (2020-02-25)
- Initial release

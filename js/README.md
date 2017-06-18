# zser.js [![Latest Version][npm-shield]][npm-link] [![Build Status][build-image]][build-link] [![Known Vulnerabilities][snyk-image]][snyk-link] [![MIT licensed][license-image]][license-link]

[npm-shield]: https://img.shields.io/npm/v/zser.svg
[npm-link]: https://www.npmjs.com/package/zser
[build-image]: https://secure.travis-ci.org/zcred/zser.svg?branch=master
[build-link]: http://travis-ci.org/zcred/zser
[snyk-image]: https://snyk.io/test/github/zcred/zser/2da5f2dce73ac0f059da4e2ea1d477e06cc190dd/badge.svg?targetFile=js%2Fpackage.json
[snyk-link]: https://snyk.io/test/github/zcred/zser/2da5f2dce73ac0f059da4e2ea1d477e06cc190dd?targetFile=js%2Fpackage.json
[license-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-link]: https://github.com/zcred/zser/blob/master/LICENSE.txt

JavaScript-compatible TypeScript implementation of **zser**: a
security-oriented serialization format with novel authentication properties
based on "Merkleized" data structures.

For more information, see the [toplevel README.md].

[toplevel README.md]: https://github.com/zcred/zser/blob/master/README.md

## Help and Discussion

Have questions? Want to suggest a feature or change?

* [Gitter]: web-based chat about zcred projects including **zser**
* [Google Group]: join via web or email ([zcred+subscribe@googlegroups.com])

[Gitter]: https://gitter.im/zcred/Lobby
[Google Group]: https://groups.google.com/forum/#!forum/zcred
[zcred+subscribe@googlegroups.com]: mailto:zcred+subscribe@googlegroups.com

## Requirements

**zser.js** is presently targeting <b>ES2017</b>. This is because we soon plan
on making use of the [TC39 BigInt] type when it becomes available, and want to
make sure users of this library can handle modern ECMAScript versions.

Please make sure your JS runtime is ES2017 compliant, or use a transpiler
like [babel] support older versions of ECMAScript.

[TC39 BigInt]: https://tc39.github.io/proposal-bigint/
[babel]: https://babeljs.io/docs/plugins/preset-es2017/

## Installation

Via [npm](https://www.npmjs.com/):

```bash
npm install zser
```

Via [Yarn](https://yarnpkg.com/):

```bash
yarn install zser
```

Import **zser** into your project with:

```js
import Zser from "zser";
```

## API

### Zser.parse()

The `Zser.parse()` method parses a `Uint8Array` containing a serialized
**zser** message into a corresponding self-describing object representation.

#### Parameters

* **message**: The `Uint8Array` containing a **zser** message to parse

#### Example

```js
let message = new Uint8Array([0x15, 0x07, 0x02, 0x03, 0x55]);
Zser.parse(message);
// Object { 1: Object { 24: 42 } }
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/zcred/zser

## Copyright

Copyright (c) 2017 [The Zcred Developers][AUTHORS].
See [LICENSE.txt] for further details.

[AUTHORS]: https://github.com/zcred/zcred/blob/master/AUTHORS.md
[LICENSE.txt]: https://github.com/zcred/zser/blob/master/LICENSE.txt

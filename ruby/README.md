# veriform.rb [![Latest Version][gem-shield]][gem-link] [![Build Status][build-image]][build-link] [![MIT licensed][license-image]][license-link]

[gem-shield]: https://badge.fury.io/rb/veriform.svg
[gem-link]: https://rubygems.org/gems/veriform
[build-image]: https://secure.travis-ci.org/zcred/veriform.svg?branch=master
[build-link]: http://travis-ci.org/zcred/veriform
[license-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-link]: https://github.com/zcred/veriform/blob/master/LICENSE.txt

Ruby implementation of **Veriform**: a cryptographically verifiable data
serialization format inspired by Protocol Buffers, useful for things like
credentials, transparency logs, and "blockchain" applications.

For more information, see the [toplevel README.md].

[toplevel README.md]: https://github.com/zcred/veriform/blob/master/README.md

## Help and Discussion

Have questions? Want to suggest a feature or change?

* [Gitter]: web-based chat about zcred projects including **Veriform**
* [Google Group]: join via web or email ([zcred+subscribe@googlegroups.com])

[Gitter]: https://gitter.im/zcred/Lobby
[Google Group]: https://groups.google.com/forum/#!forum/zcred
[zcred+subscribe@googlegroups.com]: mailto:zcred+subscribe@googlegroups.com

## Requirements

This library is tested against the following MRI versions:

- 2.2
- 2.3
- 2.4

Other Ruby versions may work, but are not officially supported.

## Installation

Add this line to your application's Gemfile:

```ruby
gem "veriform"
```

And then execute:

    $ bundle

Or install it yourself as:

    $ gem install veriform

## API

### Veriform.parse

To parse a **veriform** message, use the `Veriform.parse` method:

```ruby
>> Veriform.parse("\x15\x07\x02\x03\x55".b)
=> {1=>{24=>42}}
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/zcred/veriform

## Copyright

Copyright (c) 2017 [The Zcred Developers][AUTHORS].
See [LICENSE.txt] for further details.

[AUTHORS]: https://github.com/zcred/zcred/blob/master/AUTHORS.md
[LICENSE.txt]: https://github.com/zcred/veriform/blob/master/LICENSE.txt

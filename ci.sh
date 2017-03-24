#!/bin/bash --login

set -e -x

case $SUITE in
js)
    node --version
    npm --version
    cd js
    npm install
    npm test
    ;;
ruby)
    rvm use $RUBY_VERSION --install --binary --fuzzy
    ruby -v
    gem install bundler --version 1.14.5 --no-rdoc --no-ri
    bundle --version
    cd ruby
    bundle
    bundle exec rake
    ;;
rust)
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source ~/.cargo/env
    rustup default $RUST_VERSION
    cd rust
    cargo test
    ;;
*)
    echo "*** ERROR: Unknown test suite: '$SUITE'"
    exit 1
    ;;
esac

echo "Success!"
exit 0

#!/bin/bash --login

set -e

case $SUITE in
js)
    echo "Running JavaScript test suite..."
    echo "node version: $(node --version)"
    echo "npm version: $(npm --version)"
    cd js
    npm install
    npm test
    ;;
ruby)
    echo "Running Ruby ($RUBY_VERSION) test suite..."
    rvm use $RUBY_VERSION --install --binary --fuzzy
    ruby -v
    gem install bundler --version 1.14.5 --no-rdoc --no-ri
    bundle --version
    cd ruby
    bundle
    bundle exec rake
    ;;
*)
    echo "*** ERROR: Unknown test suite: '$SUITE'"
    exit 1
    ;;
esac

echo "Success!"
exit 0

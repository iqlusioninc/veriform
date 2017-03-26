#!/bin/bash --login

set -e

case $SUITE in
go)
    go version
    cd go
    go test -v ./...
    ;;
js)
    cd js
    nvm install stable

    # Install Yarn (TODO: use native Travis CI support when available)
    curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add -
    echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list
    sudo apt-get -qq update && sudo apt-get -qq install yarn

    yarn global add typescript typescript-formatter mocha
    yarn install
    yarn test
    tsfmt --verify $(find {src,test} -name "*.ts")
    ;;
python)
    python --version
    pip --version
    cd python
    export PATH=$HOME/.local/bin:$PATH
    pip install -r requirements.txt --user
    pytest
    ;;
ruby)
    ruby -v
    bundle --version
    cd ruby
    bundle
    bundle exec rake
    ;;
rust)
    rustc --version
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

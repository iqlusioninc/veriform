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
    $PYTEST
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

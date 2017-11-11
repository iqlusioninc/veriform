#!/bin/bash

set -e

cd go
go vet ./...
go test -v ./...

go get -u github.com/golang/lint/golint
golint -set_exit_status ./...

go get -u github.com/kisielk/errcheck
errcheck ./...


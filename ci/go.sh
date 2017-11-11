#!/bin/bash

set -e

cd go
go vet ./...
go test -v ./...


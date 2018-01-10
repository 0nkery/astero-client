#!/bin/bash

cd ~/Code/quick-protobuf/
git pull
(cd codegen && cargo install --force)

cd -

cd proto/
git pull origin master
pb-rs -s -d ../src/proto/defs ./*.proto

#!/bin/bash

cd ~/Code/quick-protobuf/
git pull
(cd codegen && cargo install --force)

cd -

cd proto/
git pull
pb-rs -s -d ../src/proto ./*.proto
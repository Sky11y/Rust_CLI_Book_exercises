#!/usr/bin/env bash

for dir in */; do
    echo "cleaning up directory: '$dir'"
    cd $dir && cargo clean && cd ..
done

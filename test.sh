#!/bin/sh

for dir in */; do
    cd $dir
    cargo build --verbose
    cd ..
done

for dir in */; do
    cd $dir
    cargo test --verbose
    cd ..
done

#!/usr/bin/env bash
sed -i "s/version = \"$1\"/version = \"$2\"/g" Cargo.toml titan-*/Cargo.toml titan/Cargo.toml

#!/usr/bin/env sh

cbindgen --config ./config/cbindgen.toml --crate tribufu-sdk --output ./include/tribufu/native.h

#!/bin/zsh

cbindgen --config ./Config/Bindings.toml --crate tribufu --output Source/Header.h
sudo ./Vendor/Premake/Mac/premake5 xcode4

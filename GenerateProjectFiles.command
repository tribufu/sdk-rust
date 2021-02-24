#!/bin/zsh

cbindgen --config ./Config/Bindings.toml --crate TribuFu --output Source/Header.h
sudo ./Vendor/Premake/Mac/premake5 xcode4

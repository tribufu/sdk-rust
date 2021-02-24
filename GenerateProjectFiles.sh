#!/bin/bash

cbindgen --config ./Config/Bindings.toml --crate TribuFu --output Source/Header.h
sudo ./Vendor/Premake/Linux/premake5 gmake2

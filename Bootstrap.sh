#!/usr/bin/env sh

echo "Installing dependencies..."

cargo install cbindgen

echo "Generating bindings..."

cbindgen --config ./Config/Bindings.toml --crate tribufu --output ./Source/Header.h

# Linux

if [ "$(expr substr $(uname -s) 1 5)" = "Linux" ]
then
    echo "Generating GMake project..."

    sudo chmod +x ./Vendor/Premake/Linux/premake5
    ./Vendor/Premake/Linux/premake5 gmake2

# Mac

elif [ "$(uname)" = "Darwin" ]
then
    echo "Generating XCode project..."

    sudo chmod +x ./Vendor/Premake/Mac/premake5
    ./Vendor/Premake/Mac/premake5 xcode4
fi

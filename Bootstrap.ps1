#!/usr/bin/env pwsh

echo "Installing dependencies..."

cargo install cbindgen

echo "Generating bindings..."

cbindgen --config ./Config/Bindings.toml --crate tribufu --output ./Source/Header.h

# Windows

if ($IsWindows)
{
    echo "Generating Visual Studio project..."

    & "./Vendor/Premake/Windows/premake5.exe" "vs2019"
}

# Mac

elseif ($IsMacOS)
{
    echo "Generating XCode project..."

    sudo chmod +x ./Vendor/Premake/Mac/premake5
    & "./Vendor/Premake/Mac/premake5" "xcode4"
}

# Linux

elseif ($IsLinux)
{
    echo "Generating GMake project..."

    sudo chmod +x ./Vendor/Premake/Linux/premake5
    & "./Vendor/Premake/Linux/premake5" "gmake2"
}

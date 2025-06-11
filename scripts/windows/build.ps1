#!/usr/bin/env pwsh

echo "Building for win-x64"
cargo build --workspace

New-Item "bin/win-x64" -ItemType Directory -Force
Remove-Item -Path "bin/win-x64/*" -Force -Recurse

Copy-Item -Path "target/debug/tribufu_sdk.dll.lib" -Destination "bin/win-x64/tribufu_sdk.lib"
Copy-Item -Path "target/debug/tribufu_sdk.dll" -Destination "bin/win-x64/tribufu_sdk.dll"

#msbuild /p:Configuration="debug" /p:Platform="win-x64"

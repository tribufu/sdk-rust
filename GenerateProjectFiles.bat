cbindgen --config .\Config\Bindings.toml --crate tribufu --output Source\Header.h
call .\Vendor\Premake\Windows\premake5.exe vs2019

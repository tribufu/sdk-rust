cbindgen --config .\Config\Bindings.toml --crate TribuFu --output Source\Header.h
call .\Vendor\Premake\Windows\premake5.exe vs2019

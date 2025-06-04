--- @diagnostic disable: undefined-global

workspace "tribufu"
    location "."
    configurations { "debug", "release" }

if _ACTION == "vs2022" then
    platforms {
        "win-arm64",
        "win-x64",
        "win-x86",
    }
end

if _ACTION == "gmake2" then
    platforms {
        "linux-arm64",
        "linux-x64",
        "linux-x86",
    }
end

if _ACTION == "xcode4" then
    platforms {
        "osx-arm64",
        "osx-x64",
    }
end

include "examples/c"
include "examples/cpp"

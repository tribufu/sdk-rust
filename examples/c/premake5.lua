--- @diagnostic disable: undefined-global

project "example_c"
    location "."
    kind "ConsoleApp"
    language "C"

    targetdir("../../bin/%{cfg.platform}")
    objdir("../../target/%{cfg.buildcfg}/obj/%{prj.name}/%{cfg.platform}")

    files
    {
        "./**.c",
        "./**.h",
        "../../include/**.h",
    }

    vpaths
    {
        ["Sources"] = { "./**.c" },
        ["Headers"] = { "./**.h", "../../include/**.h" },
    }

    includedirs
    {
        "../../include",
    }

    libdirs
    {
        "../../bin/%{cfg.platform}",
    }

    -- Profile

    filter { "configurations:debug" }
        runtime "Debug"
        symbols "On"

        defines
        {
            "DEBUG",
            "TRACE",
            "TRIBUFU_DEBUG",
            "TRIBUFU_TRACE",
        }

    filter { "configurations:release" }
        runtime "Release"
        optimize "On"

        defines
        {
            "RELEASE",
            "TRIBUFU_RELEASE",
        }

    -- Platform

    filter { "platforms:win-*" }
        system "windows"
        systemversion "latest"
        staticruntime "On"
        toolset "msc"

        defines
        {
            "TRIBUFU_DESKTOP",
            "TRIBUFU_DYNAMIC",
            "TRIBUFU_LIBRARY",
            "TRIBUFU_WINDOWS",
        }

        links
        {
            "tribufu_sdk.lib",
        }

        prelinkcommands
        {
        }

        postbuildcommands
        {
        }

    filter { "platforms:osx-*" }
        system "macosx"
        systemversion "10.15"
        toolset "clang"

        defines
        {
            "TRIBUFU_APPLE",
            "TRIBUFU_DESKTOP",
            "TRIBUFU_DYNAMIC",
            "TRIBUFU_LIBRARY",
            "TRIBUFU_MAC",
        }

        links
        {
            "tribufu_sdk",
        }

        prelinkcommands
        {
        }

        postbuildcommands
        {
        }

    filter { "platforms:linux-*" }
        system "linux"
        toolset "gcc"

        defines
        {
            "TRIBUFU_DESKTOP",
            "TRIBUFU_DYNAMIC",
            "TRIBUFU_LIBRARY",
            "TRIBUFU_LINUX",
            "TRIBUFU_UNIX",
        }

        links
        {
            "tribufu_sdk",
        }

        prelinkcommands
        {
        }

        postbuildcommands
        {
        }

    filter { "platforms:android-*" }
        system "android"
        toolset "clang"

        defines
        {
            "TRIBUFU_ANDROID",
            "TRIBUFU_DYNAMIC",
            "TRIBUFU_LIBRARY",
            "TRIBUFU_MOBILE",
            "TRIBUFU_UNIX",
        }

        links
        {
            "tribufu_sdk",
        }

        prelinkcommands
        {
        }

        postbuildcommands
        {
        }

    filter { "platforms:ios-*" }
        system "ios"
        systemversion "13.0"
        toolset "clang"

        defines
        {
            "TRIBUFU_APPLE",
            "TRIBUFU_IOS",
            "TRIBUFU_LIBRARY",
            "TRIBUFU_MOBILE",
            "TRIBUFU_MONOLITHIC",
        }

        links
        {
            "tribufu_sdk",
        }

    -- Architecture

    filter { "platforms:*-x86" }
        architecture "x86"

        defines
        {
            "TRIBUFU_32BITS",
            "TRIBUFU_X86",
        }

    filter { "platforms:*-x64" }
        architecture "x64"

        defines
        {
            "TRIBUFU_64BITS",
            "TRIBUFU_X64",
        }

    filter { "platforms:*-arm64" }
        architecture "ARM64"

        defines
        {
            "TRIBUFU_64BITS",
            "TRIBUFU_ARM64",
        }

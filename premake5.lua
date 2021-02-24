workspace "TribuFu"
    architecture "x64"

    configurations
    {
        "debug",
        "release"
    }

outputDir = "%{cfg.buildcfg}"

project "Cpp"
    location "Examples/Cpp"
    kind "ConsoleApp"
    language "C++"

    targetdir ("Binaries/" .. outputDir)
    objdir ("Intermediate/")

    files
    {
        "Examples/%{prj.name}/**.h",
        "Examples/%{prj.name}/**.cpp",
    }

    includedirs
    {
        "Source"
    }

    libdirs
    {
        "Binaries/%{cfg.buildcfg}"
    }

    filter "system:windows"
        staticruntime "On"
        systemversion "latest"

        links
        {
            "TribuFu.lib"
        }

    filter { "system:windows", "configurations:debug" }
        runtime "Debug"
        buildoptions "/MDd"
        symbols "on"

    filter  { "system:windows", "configurations:release" }
        runtime "Release"
        buildoptions "/MD"
        optimize "on"

project "C"
    location "Examples/C"
    kind "ConsoleApp"
    language "C"

    targetdir ("Binaries/" .. outputDir)
    objdir ("Intermediate/")

    files
    {
        "Examples/%{prj.name}/**.h",
        "Examples/%{prj.name}/**.c",
    }

    includedirs
    {
        "Source"
    }

    libdirs
    {
        "Binaries/%{cfg.buildcfg}"
    }

    filter "system:windows"
        staticruntime "On"
        systemversion "latest"

        links
        {
            "TribuFu.lib"
        }

    filter { "system:windows", "configurations:debug" }
        runtime "Debug"
        buildoptions "/MDd"
        symbols "on"

    filter  { "system:windows", "configurations:release" }
        runtime "Release"
        buildoptions "/MD"
        optimize "on"

project "CSharp"
    location "Examples/CSharp"
    kind "ConsoleApp"
    language "C#"

    targetdir ("Binaries/" .. outputDir)
    objdir ("Intermediate/")

    files
    {
        "Examples/%{prj.name}/**.cs",
    }

    includedirs
    {
        "Source"
    }

    libdirs
    {
        "Binaries/%{cfg.buildcfg}"
    }

    filter "system:windows"
        staticruntime "On"
        systemversion "latest"

        defines
        {
            "WINDOWS_PLATFORM"
        }

    filter { "system:windows", "configurations:debug" }
        runtime "Debug"
        buildoptions "/MDd"
        symbols "on"

    filter  { "system:windows", "configurations:release" }
        runtime "Release"
        buildoptions "/MD"
        optimize "on"

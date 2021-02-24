# Copyright (c) TribuFu. All Rights Reserved

import os
import subprocess
import platform
import shutil

from pathlib import Path

Path("./Package/Include").mkdir(parents=True, exist_ok=True)
Path("./Package/Library").mkdir(parents=True, exist_ok=True)

shutil.copy2("./Source/TribuFu.h", "./Package/Include/TribuFu.h")

if platform.system() == "Windows":
    Path("./Package/Library/Windows").mkdir(parents=True, exist_ok=True)

    shutil.copy2("./Binaries/release/TribuFu.dll", "./Package/Library/Windows/TribuFu.dll")
    shutil.copy2("./Binaries/release/TribuFu.dll.lib", "./Package/Library/Windows/TribuFu.dll.lib")
    shutil.copy2("./Binaries/release/TribuFu.lib", "./Package/Library/Windows/TribuFu.lib")

    subprocess.check_call(
        [
            "./Vendor/rcedit.exe",
            "./Package/Library/Windows/TribuFu.dll",
            "--set-file-version",
            "0.0.1",
            "--set-product-version",
            "0.0.1",
            "--set-version-string",
            "CompanyName",
            "TribuFu",
            "--set-version-string",
            "FileDescription",
            "SDK for games and apps access TribuFu services.",
            "--set-version-string",
            "FileVersion",
            "0.0.1",
            "--set-version-string",
            "InternalName",
            "SDK",
            "--set-version-string",
            "LegalCopyright",
            "Copyright (c) TribuFu. All Rights Reserved",
            "--set-version-string",
            "OriginalFilename",
            "TribuFu.dll",
            "--set-version-string",
            "ProductName",
            "TribuFu SDK",
            "--set-version-string",
            "ProductVersion",
            "0.0.1",
        ]
    )

elif platform.system() == "Linux":
    Path("./Package/Library/Linux").mkdir(parents=True, exist_ok=True)

    shutil.copy2("./Binaries/release/libTribuFu.so", "./Package/Library/Linux/libTribuFu.so")
    shutil.copy2("./Binaries/release/libTribuFu.a", "./Package/Library/Linux/libTribuFu.a")

elif platform.system() == "Mac":
    Path("./Package/Library/Mac").mkdir(parents=True, exist_ok=True)

    shutil.copy2("./Binaries/release/libTribuFu.dylib", "./Package/Library/Mac/libTribuFu.dylib")
    shutil.copy2("./Binaries/release/libTribuFu.a", "./Package/Library/Mac/libTribuFu.a")

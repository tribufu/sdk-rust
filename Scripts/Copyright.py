# Copyright (c) TribuFu. All Rights Reserved

import os

old_copyright_notice = "// Copyright (c) TribuFu. All Rights Reserved\n\n"
new_copyright_notice = "// Copyright (c) TribuFu. All Rights Reserved\n\n"

def GetFiles(root):
    source_files = []

    for path, dirs, files in os.walk(os.path.normpath(root)):
        for name in files:
            if name.endswith(".rs"):
                source_files.append(os.path.join(path, name))

    return source_files

for file in GetFiles("Source"):
    reader = open(file, "r")

    file_content = reader.read()

    reader.close()

    new_content = file_content.replace(old_copyright_notice, new_copyright_notice)

    writer = open(file, "w", newline="")

    if old_copyright_notice in file_content or new_copyright_notice in file_content:
        writer.write(new_content)
    else :
        writer.write(new_copyright_notice + file_content)

    writer.close()

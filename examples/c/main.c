// Copyright (c) Tribufu. All Rights Reserved.

#include <stdio.h>
#include <tribufu/api.h>

int main(int argc, char **argv)
{
    char *version = tribufu_api_get_user_agent();
    printf("%s\n", version);
    tribufu_free_string(version);
    return 0;
}

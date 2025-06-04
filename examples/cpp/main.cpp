// Copyright (c) Tribufu. All Rights Reserved.

#include <iostream>
#include <tribufu/api.h>

int main(int argc, char **argv)
{
    tribufu_api_initialize();
    const char *version = tribufu_api_get_user_agent();
    std::cout << version << std::endl;
    tribufu_free_string((char *)version);
    tribufu_api_shutdown();
    return 0;
}

// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

#pragma once

#include <tribufu/prelude.h>

TRIBUFU_API void tribufu_api_default(void);

TRIBUFU_API void tribufu_api_from_env(void);

TRIBUFU_API void tribufu_api_from_env_or_default(void);

TRIBUFU_API const char *tribufu_api_get_user_agent(void);

TRIBUFU_API const char *tribufu_api_get_version(void);

TRIBUFU_API void tribufu_api_new(void);

TRIBUFU_API void tribufu_api_with_api_key(void);

TRIBUFU_API void tribufu_free_string(char *ptr);

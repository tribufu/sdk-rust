// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

#pragma once

#include <tribufu/prelude.h>

typedef void TribufuApiGetUserInfoCallbackData;

typedef void (*TribufuApiGetUserInfoCallback)(void*, const TribufuApiGetUserInfoCallbackData*);

/**
 * Gets the user agent string for the Tribufu API.
 */
TRIBUFU_API const char *tribufu_api_get_user_agent(void);

TRIBUFU_API void tribufu_api_get_user_info(void *context, TribufuApiGetUserInfoCallback callback);

/**
 * Gets the version of the Tribufu API.
 */
TRIBUFU_API const char *tribufu_api_get_version(void);

/**
 * Initialize the Tribufu API instance.
 *
 * This must be called before any other API functions.
 */
TRIBUFU_API bool tribufu_api_initialize(void);

/**
 * Shutdown the Tribufu API instance.
 *
 * This must be called when the API is no longer needed.
 */
TRIBUFU_API void tribufu_api_shutdown(void);

TRIBUFU_API void tribufu_free_string(char *ptr);

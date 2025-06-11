// Copyright (c) Tribufu. All Rights Reserved.

#pragma once

#ifdef __cplusplus
#define TRIBUFU_CPP
#endif

#ifndef EXTERN_C
#ifdef TRIBUFU_CPP
#define EXTERN_C extern "C"
#else
#define EXTERN_C
#endif
#endif

#define TRIBUFU_API EXTERN_C

/*
#ifdef TRIBUFU_LIBRARY
#define TRIBUFU_API DLLEXPORT
#else
#define TRIBUFU_API DLLIMPORT
#endif
*/

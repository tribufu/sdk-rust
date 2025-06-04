// Copyright (c) Tribufu. All Rights Reserved.

#pragma once

#ifdef _WIN32
#ifndef DLLEXPORT
#define DLLEXPORT __declspec(dllexport)
#endif
#ifndef DLLIMPORT
#define DLLIMPORT __declspec(dllimport)
#endif
#endif

#ifdef __MACH__ || __APPLE__ || __linux__ || __FreeBSD__ || __ANDROID__
#ifndef DLLEXPORT
#define DLLEXPORT __attribute__((visibility("default")))
#endif
#ifndef DLLIMPORT
#define DLLIMPORT __attribute__((visibility("default")))
#endif
#endif

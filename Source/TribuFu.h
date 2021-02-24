// Copyright (c) TribuFu. All Rights Reserved

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#define TRIBUFU_SDK 1

#ifdef __cplusplus
namespace TribuFu
{
#endif

#ifdef __cplusplus
    extern "C"
    {
#endif

        void AcceptFriend(const char *id1, const char *id2);

        void AddFriend(const char *id1, const char *id2);

        void EnterTeam(void);

        void GetDevice(const char *id);

        void GetFriends(const char *id);

        void GetMessage(const char *id);

        void GetRole(const char *id);

        void GetUser(const char *id);

        void GetUserDevices(const char *id);

        void GetUserRoles(const char *id);

        int32_t Hello(void);

        void InviteToTeam(void);

        void Login(const char *name, const char *password);

        void Logout(void);

        void Refresh(void);

        void Register(const char *name, const char *email, const char *password);

#ifdef __cplusplus
    }
#endif

#ifdef __cplusplus
}
#endif

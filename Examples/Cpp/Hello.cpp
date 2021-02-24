// Copyright (c) TribuFu. All Rights Reserved

#include <iostream>

#include "TribuFu.h"

using namespace std;

extern int32_t Hello();

int main()
{
    cout << "TribuFu SDK = " << (int)TribuFu::Hello() << "\n";
}

// Copyright (c) TribuFu. All Rights Reserved

using System;
using System.Runtime.InteropServices;

namespace CSharp
{
    class Program
    {
        #if WINDOWS_PLATFORM
            const string TribuFu = "TribuFu";
        #else
            const string TribuFu = "libTribuFu";
        #endif

        [DllImport(TribuFu)]
        public static extern int Hello();

        public static void Main(string[] args)
        {
            Console.WriteLine("TribuFu SDK = " + Hello());
        }
    }
}

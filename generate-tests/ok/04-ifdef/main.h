#ifdef FOO
    disabled 1
#   ifndef BAR
        disabled 2
#   else
        disabled 3
#   endif
#else
    enabled 1
#   ifndef BAZ
        enabled 2
#   else
        disabled 4
#   endif
#endif

#ifdef FOO
    disabled 5
#endif

#ifndef FOO
    enabled 3
#endif

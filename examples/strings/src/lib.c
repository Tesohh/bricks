#include "strings.h"
#include <stdio.h>

void String_print(String str) {
    for (int i = 0; i < str.len; i++) {
        printf("%c", str.buf[i]);
    }
}

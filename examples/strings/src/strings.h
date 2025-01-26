#ifndef STRINGS_H
#define STRINGS_H

#include <stdint.h>
typedef struct String {
    char* buf;
    int64_t len;
} String;

String String_new(char*);
void String_print(String);

#endif

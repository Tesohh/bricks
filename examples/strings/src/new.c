#include "strings.h"
#include <string.h>

String String_new(char* buf) {
    return (String){.buf = buf, .len = strlen(buf)};
}

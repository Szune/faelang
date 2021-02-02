#include <stdbool.h>
#include "types.h"

#define SOME(v) { .value = (v), .is_some = true };;
#define NONE() { .is_some = false };;

#ifndef OPTION_H
#define OPTION_H
#define OPTION(typen) \
typedef struct { \
	bool is_some; \
	typen value; \
} Option_##typen;

#define OPTION_PTR(typen) \
typedef struct { \
	bool is_some; \
	typen* value; \
} Option_##typen##_ptr;


OPTION(u8);
#endif

#include "types.h"

#define OK(v) { .value_or_error.value = (v), .is_success = true };;
#define ERR(v) { .value_or_error.error = (v), .is_success = false };;

#ifndef RESULT_H
#define RESULT_H

#define RESULT(typen) \
typedef struct { \
	bool is_success; \
	union { \
		typen value; \
		char* error; \
	} value_or_error; \
} Result_##typen;

#define RESULT_PTR(typen) \
typedef struct { \
	bool is_success; \
	union { \
		typen* value; \
		char* error; \
	} value_or_error; \
} Result_##typen##_ptr;

RESULT(u8);
#endif

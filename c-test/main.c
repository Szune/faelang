#include <stdio.h>
#include <stdbool.h>
#include "types.h"
#include "option.h"
#include "result.h"
#include "list.h"

Result_u8 add_them(u8 a, u8 b) {
	if(a < 5 || b < 5) {
		Result_u8 res = ERR("Huge error from add_them");
		return res;
	} else {
		Result_u8 res = OK(a + b);
		//Result res = OK(NULL);
		return res;
	}
}

Option_u8 square_em(u8 a) {
	if(a > 12) {
		Option_u8 res = NONE();
		return res;
	} else {
		Option_u8 res = SOME(a * a);
		return res;
	}
}

int main(int argc, char *argv[]) {
	printf("===Args===\n");
	for(int i = 0; i < argc; ++i) {
		printf("Arg: %s\n", argv[i]);
	}
	printf("==========\n");
	List_u8* test = list_u8_create(3);
	list_u8_add(test, 10);
	list_u8_add(test, 15);
	for(u64 i = 0; i < test->length; ++i) {
		printf("Value in 'generic' list: %i\n", list_u8_get(test, i));
	}
	list_u8_free(test);

	Option_u8 scared = square_em(15);
	if(scared.is_some) {
		printf("Scared was some: %i\n", scared.value);
	} else {
		printf("Scared was none :(\n");
	}

	Result_u8 resulto = add_them(5,6);

	if(resulto.is_success) {
		printf("Resulto was success: %i\n", resulto.value_or_error.value);
	} else {
		printf("Resulto was failure '%s' :(\n", resulto.value_or_error.error);
	}

	/*
	u8 val = 158;
	printf("Value: %i\n", val);
	List* list = list_create(5, sizeof(u8));
	list_add(list, (void*)10);
	list_add(list, (void*)15);
	list_add(list, (void*)20);
	for(u64 i = 0; i < list->length; ++i) {
		printf("Value in list: %i\n", (u8)list->items[i]);
	}
	list_free(list);
	*/


	//Result res = {.is_success = true,.value_or_error.value = &val};
	/*
	Result res_ok = OK(&val);
	printf("Result: success = %i, value = %u\n", res_ok.is_success, *(u8*)res_ok.value_or_error.value);
	Result res_err = ERR("Huge error");
	printf("Result: success = %i, value = %s\n", res_err.is_success, res_err.value_or_error.error);
	u8 x = 1 + 1;
	printf("Value: %i\n", x);
	*/

/*
    // Handling Result types with void pointers: 
	Result addem = add_them(11, 12);
	if (addem.is_success && addem.value_or_error.value) {
		printf("Result: success = %i, value = %i\n", addem.is_success, *(u8*) addem.value_or_error.value);
	} else {
		if (addem.value_or_error.error) {
			printf("Result: success = %i, error = %s\n", addem.is_success, addem.value_or_error.error);
		} else {
			printf("Result: success = %i, no error\n", addem.is_success);
		}
	}
*/

/* 
    // Handling Option types with void pointers: 
	Option squared = square_em(15);
	if (squared.is_some && squared.value) {
		printf("Option: is_some = %i, value = %i\n", squared.is_some, *(u8*) squared.value);
	} else {
		printf("Option: is_some = %i\n", squared.is_some);
	}
*/
	return 0;
}

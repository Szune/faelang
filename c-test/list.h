#include <stdlib.h>
#include "types.h"



#define LIST_FN_CREATE(typen) \
List_##typen* list_##typen##_create(u64 initial_capacity) {                   \
	List_##typen* list = malloc(sizeof(List_##typen));                      \
	list->length = 0;                                       \
	list->capacity = initial_capacity;                      \
	list->items = malloc(sizeof(typen) * initial_capacity); \
	return list;                                            \
}

#define LIST_FN_ADD(typen) \
void list_##typen##_add(List_##typen* list, typen item) { \
	if(list) { \
		list->items[list->length++] = item; \
	} \
}


#define LIST_FN_GET(typen) \
typen list_##typen##_get(List_##typen* list, u64 index) { \
    return list->items[index]; \
}

#define LIST_FN_GET_PTR(typen) \
typen list_##typen##_get(List_##typen* list, u64 index) { \
	if(list) { \
		return list->items[index]; \
	} \
	return NULL; \
}

#define LIST_FN_FREE(typen) \
void list_##typen##_free(List_##typen* list) { \
	if(list) { \
		free(list); \
	} \
}

#define LIST_FN_FREE_ALL(typen) \
void list_##typen##_free_all(List* list) { \
	if(list) { \
		if(list->items) { \
			for(u64 i = 0; i < list->length; ++i) { \
				free(list->items[i]); \
			} \
		} \
		free(list); \
	} \
}

#ifndef LIST_H
#define LIST_H

#define LIST_STRUCT(typen) typedef struct {   \
                       u64 length;    \
                       u64 capacity;  \
                       typen* items; \
                   } List_##typen;;

#define LIST_NO_PTR(typen) \
LIST_STRUCT(typen); \
LIST_FN_CREATE(typen); \
LIST_FN_ADD(typen); \
LIST_FN_GET(typen); \
LIST_FN_FREE(typen);

#define LIST_PTR(typen) \
LIST_STRUCT(typen); \
LIST_FN_CREATE(typen); \
LIST_FN_ADD(typen); \
LIST_FN_GET_PTR(typen); \
LIST_FN_FREE(typen); \
LIST_FN_FREE_ALL(typen);

LIST_NO_PTR(u8);

#endif

// end

// old functions
List* list_create(u64 initial_capacity, u64 item_length) {
	// TODO: check malloc for failure, return Result
	List* list = malloc(sizeof(List));
	list->length = 0;
	list->capacity = initial_capacity;
	list->items = malloc(sizeof(item_length) * initial_capacity);
	return list;
}

void list_add(List* list, void* item) {
	if(list) { // TODO: return false if failure to add instead?
		// TODO: check array bound before add
		// TODO: check if list needs to grow before add
		// (growing happens by copying to a newly allocated array with a higher capacity)
		list->items[list->length++] = item;
	}
}

void* list_get(List* list, u64 index) {
	if(list) {
		// TODO: check array bounds
		return list->items[index];
	}
	return NULL; // TODO: return Option instead!
}

void list_free(List* list) {
	if(list) {
		free(list);
	}
}

void list_free_all(List* list) {
	if(list) {
		if(list->items) {
			for(u64 i = 0; i < list->length; ++i) {
				free(list->items[i]);
			}
		}
		free(list);
	}
}

// Elo's core typing definition

#include <stdint.h>
#include <stdbool.h>
#include <mem.h>
#include <panic.h>
#include <str.h>

#ifndef TYPING_H
#define TYPING_H

typedef int64_t                           _ELO_I64_T;
typedef int32_t                           _ELO_I32_T;
typedef int16_t                           _ELO_I16_T;
typedef int8_t                            _ELO_I8_T;
typedef bool                              _ELO_BOOL_T;
typedef uint64_t                          _ELO_U64_T;
typedef uint32_t                          _ELO_U32_T;
typedef uint16_t                          _ELO_U16_T;
typedef uint8_t                           _ELO_U8_T;
typedef float                             _ELO_F32_T;
typedef double                            _ELO_F64_T;
typedef int32_t                           _ELO_INT_T;
typedef uint32_t                          _ELO_UINT_T;
typedef float                             _ELO_FLOAT_T;
typedef Str                               _ELO_STR_T;
typedef uint32_t                          _ELO_CHAR_T;

// Generic type generation
#define __elo_struct(Name, ...) \
	typedef struct { \
		__VA_ARGS__ \
	} Name;

#define __elo_option(Name, T) \
 	typedef struct { \
		bool is_some; \
		T some; \
	} Name;

#define __elo_result(Name, O, F) \
 	typedef struct { \
		bool is_some; \
		union { O ok; F fail; }; \
	} Name;

#endif

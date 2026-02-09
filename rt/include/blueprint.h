// Elo's compiler
// C backend generation blueprints to dynamically generate types

#ifndef BLUEPRINT_H
#define BLUEPRINT_H

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

/*
 * A number of useful functionality is exposed only as static functions in a header file.
 * Reexport them as regular functions here so that rust can access them.
 *
 * TODO does this impact performance?
 */

#define reexport_static1(ret, name, A) \
	ret name##_c(A a) { \
		return name(a); \
	}

#define reexport_static2(ret, name, A, B) \
	ret name##_c(A a, B b) { \
		return name(a, b); \
	}

#define reexport_static3(ret, name, A, B, C) \
	ret name##_c(A a, B b, C c) { \
		return name(a, b, c); \
	}

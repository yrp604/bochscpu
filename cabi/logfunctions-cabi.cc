// 3/25/19: rust cannot currently define extern C variadic functions so were
// stuck with this shit
//
// TODO when rust supports extern variadic functions, replace all of this
// with var args versions

#include <stdio.h>
#include <stdarg.h>

#include "bochs.h"

namespace rust {
extern "C" {
	void logfunctions_error(const char *);
	void logfunctions_ldebug(const char *);
	void logfunctions_info(const char *);
	void logfunctions_panic(const char *);
	void logfunctions_fatal1(const char *);
}
}

logfunctions::logfunctions(void) {}
logfunctions::~logfunctions(void) {}

void logfunctions::error(const char *fmt, ...) {
	char buf[0x1000];

	va_list args;
	va_start(args, fmt);
	vsnprintf(buf, sizeof buf, fmt, args);
	va_end(args);

	rust::logfunctions_error(buf);
}

void logfunctions::fatal1(const char *fmt, ...) {
	char buf[0x1000];

	va_list args;
	va_start(args, fmt);
	vsnprintf(buf, sizeof buf, fmt, args);
	va_end(args);

	rust::logfunctions_fatal1(buf);
}

void logfunctions::info(const char *fmt, ...) {
	char buf[0x1000];

	va_list args;
	va_start(args, fmt);
	vsnprintf(buf, sizeof buf, fmt, args);
	va_end(args);

	rust::logfunctions_info(buf);
}

void logfunctions::ldebug(const char *fmt, ...) {
	char buf[0x1000];

	va_list args;
	va_start(args, fmt);
	vsnprintf(buf, sizeof buf, fmt, args);
	va_end(args);

	rust::logfunctions_ldebug(buf);
}

void logfunctions::panic(const char *fmt, ...) {
	char buf[0x1000];

	va_list args;
	va_start(args, fmt);
	vsnprintf(buf, sizeof buf, fmt, args);
	va_end(args);

	rust::logfunctions_panic(buf);
}

void logfunctions::put(const char *p, const char *q) {}

BOCHSAPI class logfunctions *genlog = NULL;

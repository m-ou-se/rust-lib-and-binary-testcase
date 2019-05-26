#include <dlfcn.h>
#include <stdio.h>

int main(int argc, char **argv) {
	if (argc != 2) {
		printf("Usage: %s <file>\n", argv[0]);
		return 1;
	}
	void *m = dlopen(argv[1], RTLD_NOW);
	if (!m) {
		puts("Can't load file");
		return 1;
	}
	void *f = dlsym(m, "foo");
	if (!f) {
		puts("Can't find symbol 'foo'.");
		return 1;
	}
	((void(*)())f)();
	return 0;
}

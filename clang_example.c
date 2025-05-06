#include "./clang-c/Index.h"

int foo(int x, int y);

int foo(int x, int y) {
	return x + y;
}

int main(int argc, char** argv) {
	CXIndex index = clang_createIndex(0, 0);

	CXTranslationUnit tu;

	enum CXErrorCode err = clang_parseTranslationUnit2(index,
			"./clang-c/Index.h", NULL,
			0, NULL, 0,
			CXTranslationUnit_DetailedPreprocessingRecord, &tu);

	clang_disposeTranslationUnit(tu);
	clang_disposeIndex(index);
	return 0;
}

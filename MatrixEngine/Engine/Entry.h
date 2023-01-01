#ifndef MATRIX_ENGNIE_ENTRY
#define MATRIX_ENGNIE_ENTRY

#include "Application.h"
#include <memory>

int main();
std::unique_ptr<me::Application> createMainApp();
#endif // !MATRIX_ENGNIE_ENTRY

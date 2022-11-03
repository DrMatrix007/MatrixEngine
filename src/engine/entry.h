#ifndef MATRIX_ENGINE_MAIN
#define MATRIX_ENGINE_MAIN



#include "application.h"
#include <memory>


std::unique_ptr<me::Application> create_main_app();


int main();

#endif // !MATRIX_ENGINE_MAIN
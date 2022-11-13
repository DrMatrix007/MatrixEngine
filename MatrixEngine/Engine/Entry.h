#ifndef MATRIX_ENGNIE_ENTRY
#define MATRIX_ENGNIE_ENTRY

#include "Application.h"
#include <memory>

#ifndef linux
#define SDL_MAIN_HANDLED
#endif // linux

#include <SDL2/SDL_main.h>

int main();
std::unique_ptr<me::Application> createMainApp();
#endif // !MATRIX_ENGNIE_ENTRY

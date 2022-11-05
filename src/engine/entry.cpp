#include "entry.h"
#ifdef linux
#define SDL_MAIN_HANDLED
#endif
#include <SDL2/SDL.h>

int main(int argc, char **argv)
{
	SDL_Init(SDL_INIT_EVERYTHING);

	auto app = create_main_app();

	// app->run();
	
	SDL_Quit();
	return 0;
}
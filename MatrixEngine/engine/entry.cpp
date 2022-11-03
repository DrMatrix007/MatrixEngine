#include "entry.h"
#define SDL_MAIN_HANDLED
#include "SDL.h"
int main()
{
	SDL_Init(SDL_INIT_EVERYTHING);

	auto app = create_main_app();

	app->run();

	SDL_Quit();
	return 0;
}
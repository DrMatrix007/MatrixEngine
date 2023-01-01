#ifndef MATRIX_ENGINE_APPLICATION
#define MATRIX_ENGINE_APPLICATION


#include "ECS/ECS.h"
namespace me
{


	class Application
	{
	public:
		Registry& getRegistry();

		void stop();
		void run();

	private:
		Registry _reg;
		bool _running = true;
	};



};

#endif // !MATRIX_ENGINE_APPLICATION
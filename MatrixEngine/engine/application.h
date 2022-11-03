#ifndef MATRIX_ENGINE_APPLICATION
#define MATRIX_ENGINE_APPLICATION

int main();
namespace me
{
	class Application
	{
	public:
		inline void stop() {
			running = false;
		}

	private:
		bool running = true;
		inline void run() {}
		friend int ::main();
	};
}

#endif // !MATRIX_ENGINE_APPLICATION
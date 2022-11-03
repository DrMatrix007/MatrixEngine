#ifndef MATRIX_ENGINE_APPLICATION
#define MATRIX_ENGINE_APPLICATION

int main();
namespace me
{
	class Application
	{
	public:
		inline void stop() {
			m_Running = false;
		}

	private:
		bool m_Running = true;
		void run();	
		friend int ::main();
	};
}

#endif // !MATRIX_ENGINE_APPLICATION
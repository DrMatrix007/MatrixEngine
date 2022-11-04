#ifndef MATRIX_ENGINE_APPLICATION
#define MATRIX_ENGINE_APPLICATION

namespace me
{
	class Application
	{
	public:
		inline void stop() {
			m_Running = false;
		}
		void run();	

	private:
		bool m_Running = true;
	};
}

#endif // !MATRIX_ENGINE_APPLICATION
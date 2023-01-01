#ifndef MATRIX_ENGINE_ISYSTEM
#define MATRIX_ENGINE_ISYSTEM

#include "../Utils/Utils.h"

namespace me
{
	class Registry;
	class Application;
	struct SystemArgs
	{
	public:

		SystemArgs(Registry*, Application*);

		Registry& getRegistry() const;
		me::Application* const getApplication();
	private:
		Registry* const _reg;
		Application* const _app;
	};


	class ISystem
	{
	public:
		void update( SystemArgs&);
		void lateUpdate(SystemArgs&);

		virtual ~ISystem() = default;
	protected:
		virtual void onUpdate( SystemArgs&) abstract;
		virtual void onLateUpdate(SystemArgs&) {};


	private:
		bool _hasStarted = false;
	};
}
#endif // !MATRIX_ENGINE_ISYSTEM
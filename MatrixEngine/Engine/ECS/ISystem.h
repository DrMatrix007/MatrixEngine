#ifndef MATRIX_ENGINE_ISYSTEM
#define MATRIX_ENGINE_ISYSTEM

namespace me
{
	class Registry;
	class Application;
	struct SystemArgs
	{
	public:

		SystemArgs(Registry*, Application*);

		Registry& getRegistry() const;
		Application& getApplication() const;
	private:
		Registry* const _reg;
		Application* const _app;
	};


	class ISystem
	{
	public:
		void update(const SystemArgs&);
		void lateUpdate(const SystemArgs&);

		virtual ~ISystem() = default;
	protected:
		virtual void onUpdate(const SystemArgs&) abstract;
		virtual void onLateUpdate(const SystemArgs&) {};


	private:
		bool _hasStarted = false;
	};
}
#endif // !MATRIX_ENGINE_ISYSTEM
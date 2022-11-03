#ifndef MATRIX_ENGNIE_COMPONENT
#define MATRIX_ENGNIE_COMPONENT

namespace me::ecs
{
	struct component_args
	{
		float delta_time;
	};

	class component
	{
	public:

		inline void init()
		{
			if (!has_init)
			{
				has_init = true;
				init();
			}
		}
		
		inline void update(const component_args& args)
		{
			on_update(args);
		}
		inline void destroy()
		{
			on_destroy();
		}
	private:
		bool has_init = false;
		virtual void on_init() = 0;
		virtual void on_destroy() = 0;
		virtual void on_update(const component_args&) = 0;
	};

}

#endif // !MATRIX_ENGNIE_COMPONENT

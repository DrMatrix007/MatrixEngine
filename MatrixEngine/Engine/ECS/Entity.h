#ifndef MATRIX_ENGINE_ENTITY
#define MATRIX_ENGINE_ENTITY
namespace me
{
	class Entity
	{
	public:
		Entity();
		Entity(const Entity&) = default;


	private:
		friend bool operator<(const Entity& a,const Entity& b);
		unsigned long long id;
		static unsigned long long counter;
	};
}	


#define MATRIX_ENGINE_ENTITY
#endif // !MATRIX_ENGINE_ENTITY
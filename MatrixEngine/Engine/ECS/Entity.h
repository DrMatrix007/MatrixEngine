#ifndef MATRIX_ENGINE_ENTITY
#define MATRIX_ENGINE_ENTITY
namespace me
{
	class Entity
	{
	public:
		Entity()
		{
			id = counter++;
		}
		Entity(const Entity&) = default;


	private:
		friend bool operator<(const Entity& a,const Entity& b);
		unsigned long long id;
		static unsigned long long counter;
	};
	bool operator<(const Entity& a,const Entity& b)
	{
		return a.id < b.id;
	}
	unsigned long long Entity::counter = 0;
}


#define MATRIX_ENGINE_ENTITY
#endif // !MATRIX_ENGINE_ENTITY
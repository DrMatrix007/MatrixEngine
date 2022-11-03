#ifndef MATRIX_ENGINE_ENTITY
#define MATRIX_ENGINE_ENTITY

namespace me::ecs
{
	class entity
	{
	public:
		entity();
		entity(const entity&);
	private:
		unsigned long long value;
		friend inline bool operator<(const entity& a, const entity& b);
		friend inline bool operator==(const entity& a, const entity& b);
	};
	inline bool operator<(const entity& a, const entity& b)
	{
		return a.value < b.value;
	}
	inline bool operator==(const entity& a, const entity& b)
	{
		return a.value == b.value;
	}
}

#endif // !MATRIX_ENGINE_ENTITY

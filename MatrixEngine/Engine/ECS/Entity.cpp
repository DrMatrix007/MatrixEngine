#include "Entity.h"
using me::Entity;
me::Entity::Entity()
{
	id = counter++;
}

namespace me
{
	bool operator<(const Entity& a, const Entity& b)
	{
		return a.id < b.id;
	}
	unsigned long long Entity::counter = 0;

}


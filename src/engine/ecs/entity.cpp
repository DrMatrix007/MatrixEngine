#include "entity.h"

using namespace me::ecs;

entity::entity() {
	static unsigned long long c = 0;
	value = ++c;
}
entity::entity(const entity& e) {
	value = e.value;
}



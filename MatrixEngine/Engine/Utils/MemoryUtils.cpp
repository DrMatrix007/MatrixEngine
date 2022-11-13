#include "MemoryUtils.h"
#include <iostream>

namespace me
{
	Locker<std::ostream*,NoDelete> logout(&std::cout);
}

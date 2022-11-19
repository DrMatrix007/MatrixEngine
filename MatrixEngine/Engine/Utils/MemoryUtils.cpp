#include "MemoryUtils.h"
#include <iostream>

namespace me
{
	Locker<std::ostream*,NoDelete> cout(&std::cout);
}

#ifndef MATRIX_ENGINE_SYSTEM
#define MATRIX_ENGINE_SYSTEM

#include "../utils/utils.h"

namespace  me::ecs {

    class base_system {
        
    };

    template<typename ...Components>
    class system : public base_system {
        static_assert(me::utils::unique_template<Components...>::unique, "no duplicates of component args!");

    };
}




#endif
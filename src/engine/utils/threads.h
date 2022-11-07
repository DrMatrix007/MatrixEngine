#ifndef MATRIX_ENGINE_THREADS
#define MATRIX_ENGINE_THREADS

#include <vector>
#include <thread>

namespace me
{

    class thread_pool
    {

    public:
        void join();
        thread_pool() = default;
        thread_pool(std::thread&&);
        thread_pool(std::vector<std::thread>&&);
        void push(std::thread&&);
        // thread_pool(std::initializer_list<std::thread>);
    private:
        std::vector<std::thread> threads;
    };
}

#endif
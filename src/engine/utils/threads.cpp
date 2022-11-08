#include "threads.h"

me::thread_pool::thread_pool(std::thread &&t) : threads(1)
{
    threads.push_back(std::move(t));
}

// me::thread_pool::thread_pool(std::initializer_list<std::thread> l) : threads(l)
// {
// }
me::thread_pool::thread_pool(std::vector<std::thread> &&l) : threads(std::move(l))
{
}
void me::thread_pool::push(std::thread &&t)
{
    threads.push_back(std::move(t));
}

void me::thread_pool::join()
{
    for (auto &i : threads)
    {
        if (i.joinable())
        {
            i.join();
        }
    }
}

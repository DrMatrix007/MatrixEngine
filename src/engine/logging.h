#ifndef MATRIX_ENGINE_LOGGING
#define MATRIX_ENGINE_LOGGING

#include <iostream>
#include <mutex>
namespace me
{
    class threadstream
    {
    public:
        inline threadstream(std::ostream &s) : stream(s)
        {
        }
        template <typename T>
        inline threadstream operator<<(const T &data)
        {
            auto [g,io] = stream.write();
            io << data;
            return *this;
        }

    private:
        locker<std::ostream&> stream;
    
    };

    threadstream meout(std::cout);
}

#endif
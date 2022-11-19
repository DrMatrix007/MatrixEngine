#include "entry.h"
#include "Utils/Locker.h"
#include <iostream>


int main()
{
    std::unique_ptr<me::Application> app(createMainApp());
    
    if (app.get())
    {
        app->run();
    }

    std::cout << "Done!" << std::endl;

    return 0;
}

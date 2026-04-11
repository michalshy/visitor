#include "visitor.h"
#include <stdexcept>

int main() {
    if(visitor::App app = visitor::App(); app.Initialize()) {
        app.Run();
    }
    else
    {
        throw std::runtime_error("App couldn't be initialized");
    }
    return 0;
}
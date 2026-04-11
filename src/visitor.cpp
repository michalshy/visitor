#include "visitor.h"
#include <memory>

namespace visitor
{
    // This should be configurable later
    App::App() : tui(std::make_unique<Tui>()) {
        
    }

    void App::Run() {
        tui->Run();
    }
}
#include "visitor.h"
#include <memory>

namespace visitor
{
    // This should be configurable later
    App::App() : tui(std::make_unique<Tui>()) {
        tui->Initialize();
    }

    bool App::Initialize() {
        if(!tui->Initialize())
            return false;
        return true;
    }

    void App::Run() {
        tui->Run();
    }
}
#ifndef VISITOR_TUI_H
#define VISITOR_TUI_H

#include "pane.h"
#include <array>
#include <memory>

namespace ftxui {
    class ScreenInteractive;
    class Event;
}

namespace visitor {
    class Tui {
        struct Impl;
        std::unique_ptr<Impl> pimpl;
        
    public:
        Tui();
        ~Tui();
        bool Initialize();
        void Run();
    private:
        bool HandleEvent(ftxui::Event);
        void Quit();
        void Switch();
    };
}

#endif
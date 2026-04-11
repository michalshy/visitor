#ifndef VISITOR_H
#define VISITOR_H

#include <memory>
#include "ui/tui.h"

namespace visitor
{
    class App {
        std::unique_ptr<Tui> tui;
    public:
        App();

        App(App&) = delete;
        App(App&&) = delete;
        ~App() = default;
        
        App& operator=(const App&) = delete;
        App& operator=(App&&) = delete;
        
        void Run();
    };
}


#endif
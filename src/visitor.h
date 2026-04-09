#ifndef VISITOR_H
#define VISITOR_H

#include "core/core.h"
#include "view/iview.h"
#include <memory>

namespace visitor
{
    class App {
        std::unique_ptr<Core> core;
        std::unique_ptr<interfaces::IView> view;
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
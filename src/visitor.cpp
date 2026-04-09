#include "visitor.h"
#include "core/core.h"
#include "view/tui.h"
#include <memory>

#include <ftxui/component/screen_interactive.hpp>
#include <ftxui/dom/elements.hpp>
#include <ftxui/dom/node.hpp>
#include <ftxui/component/component.hpp>

using namespace ftxui;

namespace visitor
{
    // This should be configurable later
    App::App() : core(std::make_unique<Core>()), view(std::make_unique<TuiView>()) {
        
    }

    void App::Run() {
        auto screen = ScreenInteractive::TerminalOutput();
        auto document = text("visitor - file manager") | center;
        screen.Loop(Renderer([&] { return document; }));
    }
}
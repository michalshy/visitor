#include "tui.h"

#include <ftxui/component/screen_interactive.hpp>
#include <ftxui/dom/elements.hpp>
#include <ftxui/dom/node.hpp>
#include <ftxui/component/component.hpp>

using namespace ftxui;

namespace visitor
{
    void Tui::Run() {
        auto screen = ScreenInteractive::Fullscreen();
        screen.Loop(Renderer([] { return text("visitor - file manager") | center; }));
    }
}
#include "tui.h"

#include <ftxui/component/screen_interactive.hpp>
#include <ftxui/dom/elements.hpp>
#include <ftxui/dom/node.hpp>
#include <ftxui/component/component.hpp>

using namespace ftxui;

namespace visitor
{
    bool Tui::Initialize() {
        for(auto& pane: panes) {
            if(!pane.Initialize())
                return false;
        }

        return true;
    }

    void Tui::Run() {
        auto screen = ScreenInteractive::Fullscreen();

        auto left = Menu({
            .entries = panes[0].Names(),
            .selected = panes[0].Index()
        });

        auto right = Menu({
            .entries = panes[1].Names(),
            .selected = panes[1].Index()
        });

        auto container = Container::Horizontal({ left, right });

        screen.Loop(Renderer([&] { return hbox({ left->Render() | border, right->Render() | border }); }));
    }
}
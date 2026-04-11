#include "tui.h"

#include <ftxui/component/screen_interactive.hpp>
#include <ftxui/dom/elements.hpp>
#include <ftxui/dom/node.hpp>
#include <ftxui/component/component.hpp>
#include <memory>

using namespace ftxui;

namespace visitor
{
    struct Tui::
    Impl {
        ScreenInteractive screen;
        std::array<Pane, 2> panes{ Pane(PaneKind::HOME), Pane(PaneKind::CURRENT) };
        size_t active{0};

        Impl() : screen(ScreenInteractive::Fullscreen()) {}
    };

    Tui::Tui() : pimpl(std::make_unique<Impl>()) {}

    Tui::~Tui() = default;


    bool Tui::Initialize() {
        for(auto& pane: pimpl->panes) {
            if(!pane.Initialize())
                return false;
        }

        return true;
    }

    void Tui::Run() {
        auto opt = MenuOption::Vertical();
        opt.entries = pimpl->panes[0].Names();
        opt.selected = pimpl->panes[0].Index();
        auto left = Menu(opt);

        opt = MenuOption::Vertical();
        opt.entries = pimpl->panes[1].Names();
        opt.selected = pimpl->panes[1].Index();
        auto right = Menu(opt);

        auto container = Container::Horizontal({ left, right });

        auto hint = [](const std::string& key, const std::string& action) {
            return hbox({
                text(" " + key + " ") | bgcolor(Color::White) | color(Color::Black),
                text(" " + action + " ") | color(Color::White),
            });
        };

        auto layout = Renderer(container, [=, this] {
            const auto& path_left = pimpl->panes[0].Path();
            const auto& path_right = pimpl->panes[1].Path();
            const size_t active = pimpl->active;

            auto title = text("visitor — terminal file manager") | bold | center;

            auto make_path_bar = [&](const std::string& p, bool is_active) {
                auto e = text(" " + p + " ") | flex;
                return is_active
                    ? e | bgcolor(Color::Blue)  | color(Color::White)
                    : e | bgcolor(Color::GrayDark) | color(Color::GrayLight);
            };

            auto paths = hbox({
                make_path_bar(path_left,  active == 0),
                text(" "),
                make_path_bar(path_right, active == 1),
            });

            auto header = vbox({ title, paths });

            auto body = hbox({
                left->Render()  | border | flex,
                right->Render() | border | flex,
            }) | flex;

            auto footer = hbox({
                filler(),
                hint("Enter", "Open"), separator(),
                hint("Bksp",  "Parent"), separator(),
                hint("< >",     "Switch"), separator(),
                hint("F5",    "Refresh"), separator(),
                hint("F7",    "MkDir"), separator(),
                hint("F8",    "Delete"), separator(),
                hint("Esc",   "Quit"),
                filler(),
            }) | bgcolor(Color::GrayDark);

            return vbox({
                header,
                separator(),
                body,
                separator(),
                footer,
            }) | flex;
        });

        auto root = CatchEvent(layout, [this](Event e) { return HandleEvent(e); });
        
        pimpl->screen.Loop(root);
    }

    bool Tui::HandleEvent(Event event) {
        auto& active_pane = pimpl->panes[pimpl->active];
        if(event.is_mouse()) return true;
        if (event == Event::ArrowLeft)  { pimpl->active = 0; return false; }
        if (event == Event::ArrowRight) { pimpl->active = 1; return false; }
        if (event == Event::Return) { active_pane.Execute(); return true; }
        if (event == Event::Escape) { Quit(); return true; }
        return false;
    }

    void Tui::Quit() {
        pimpl->screen.Exit();
    }

    void Tui::Switch() {
        if(pimpl->active == pimpl->panes.size() - 1) {
            pimpl->active = 0;
            return;
        }
        pimpl->active++;
    }
}
#include <ftxui/component/screen_interactive.hpp>
#include <ftxui/dom/elements.hpp>
#include <ftxui/dom/node.hpp>
#include <ftxui/component/component.hpp>

using namespace ftxui;

int main() {
    auto screen = ScreenInteractive::TerminalOutput();

    auto document = text("visitor - file manager") | center;

    screen.Loop(Renderer([&] { return document; }));
}
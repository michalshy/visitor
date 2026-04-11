#ifndef VISITOR_TUI_H
#define VISITOR_TUI_H

#include "pane.h"
#include <array>

namespace visitor {
    class Tui {
        std::array<Pane, 2> panes;
    public:
        Tui() : panes({ Pane(PaneKind::HOME), Pane(PaneKind::CURRENT) }) {}
        bool Initialize();
        void Run();
    };
}

#endif
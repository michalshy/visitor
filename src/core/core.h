#ifndef VISITOR_CORE_H
#define VISITOR_CORE_H

#include "pane.h"
#include <tuple>

namespace visitor
{
    // Orchestrates filesystem, manages other functionalities,
    // presistence, scripting etc.
    class Core {
        std::tuple<Pane, Pane> panes;
    };
}

#endif
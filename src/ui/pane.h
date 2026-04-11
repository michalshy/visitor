#ifndef VISITOR_PANE_H
#define VISITOR_PANE_H

#include <cstddef>
#include <filesystem>
#include <string>
#include <vector>
#include "core/common.h"

namespace visitor
{   
    enum class PaneKind {
        HOME,
        CURRENT,
    };

    class Pane {
        std::filesystem::path current;
        int index{0};
        std::vector<Entry> entries;
        std::vector<std::string> entries_names;

    public:
        explicit Pane(PaneKind kind);

        std::vector<std::string>* Names() { return &entries_names; }
        int* Index() { return &index; };

        bool Initialize();
        void Render();    
    };
}

#endif
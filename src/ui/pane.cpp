#include "pane.h"
#include "core/common.h"
#include "core/operations.h"
#include <filesystem>

namespace fs = std::filesystem;

namespace visitor
{
    Pane::Pane(PaneKind kind) {
        // establish current directory
        switch (kind) {
            case PaneKind::CURRENT:
                current = opr::get_current();
            break;
            case PaneKind::HOME:
                current = opr::get_home();
            break;
        }
    }

    bool Pane::Initialize() {
        // fill vectors
        for (const auto& entry : fs::directory_iterator(current)) {
            std::string name = entry.path().filename(); 
            Entry e;
            e.name = name;
            if (entry.is_directory()) {
                e.is_directory = true;
            }
            else {
                e.is_directory = false;
            }
            entries.push_back(e);
            entries_names.push_back(name);
        }
        return true;
    }

    void Pane::Render() {

    }
}
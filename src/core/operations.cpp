#include "operations.h"
#include <filesystem>

namespace visitor
{
    namespace opr
    {
        std::filesystem::path get_current() {
            return std::filesystem::current_path();
        }

        std::filesystem::path get_home() {
            //TODO: implement later, for now fallback to get_current
            return get_current();
        }
    }
}
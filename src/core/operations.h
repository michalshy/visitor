#ifndef VISITOR_OPERATIONS_H
#define VISITOR_OPERATIONS_H

#include <filesystem>

namespace visitor
{
    namespace opr 
    {
        std::filesystem::path get_current();
        std::filesystem::path get_home();
    }
}

#endif
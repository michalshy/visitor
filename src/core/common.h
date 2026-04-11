#ifndef VISITOR_COMMON_H
#define VISITOR_COMMON_H

#include <string>
namespace visitor
{
    struct Entry {
        std::string name;
        bool is_directory;
    };
}

#endif
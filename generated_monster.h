#pragma once

#include <cstdint>
#include <string>
#include <vector>

enum class Color : int8_t {
    Red = 0,
    Green = 1,
    Blue = 2,
};

struct Monster {
    int32_t hp;
    int32_t mana;
    std::string name;
};


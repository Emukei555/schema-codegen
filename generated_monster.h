#pragma once

#include <cstdint>
#include <string>
#include <vector>

enum class Color : int8_t {
    Red = 10,
    Green = 11,
    Blue = -5,
};

struct Vec3 {
    float x;
    float y;
    float z;
};

struct Monster {
    int32_t hp;
    int32_t mana;
    std::string name;
    Vec3 pos;
    Color color;
};


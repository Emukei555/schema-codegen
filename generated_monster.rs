#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum Color {
    Red = 10,
    Green = 11,
    Blue = -5,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Monster {
    pub hp: i32,
    pub mana: i32,
    pub name: String,
    pub pos: Vec3,
    pub color: Color,
    pub inventory: Vec<i8>,
    pub weapons: Vec<String>,
}


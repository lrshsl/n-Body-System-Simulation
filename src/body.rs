use std::collections::VecDeque;

use macroquad::{color::Color, math::Vec2};

#[derive(Clone, PartialEq)]
pub struct Body {
    pub pos: Vec2,
    pub mass: f32,
    pub color: Color,
    pub vel: Vec2,
    pub acc: Vec2,
    pub trace: VecDeque<Vec2>,
}

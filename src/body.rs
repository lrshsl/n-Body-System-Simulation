use std::collections::VecDeque;

use macroquad::{color::Color, math::DVec2};

#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    pub pos: DVec2,
    pub mass: f64,
    pub color: Color,
    pub vel: DVec2,
    pub acc: DVec2,
    pub trace: VecDeque<DVec2>,
}

impl Body {
    pub fn radius(&self) -> f64 {
        self.mass
    }
}

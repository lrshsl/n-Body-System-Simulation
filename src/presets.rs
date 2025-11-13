use std::collections::VecDeque;

use macroquad::{
    color::{BLUE, GREEN, RED},
    math::{Vec2, vec2},
};

use crate::{body::Body, consts::DEFAULT_SETTINGS};

pub fn n3_one_large() -> [Body; 3] {
    [
        Body {
            pos: vec2(800.0, 100.0),
            mass: 5.0,
            color: GREEN,
            vel: vec2(150.0, 0.0),
            acc: Vec2::ZERO,
            trace: VecDeque::with_capacity(DEFAULT_SETTINGS.tail_length),
        },
        Body {
            pos: vec2(1600.0, 800.0),
            mass: 5.0 * 50.0,
            color: RED,
            vel: vec2(-150.0 / 50.0, 100.0 / 50.0),
            acc: Vec2::ZERO,
            trace: VecDeque::with_capacity(DEFAULT_SETTINGS.tail_length),
        },
        Body {
            pos: vec2(300.0, 1550.0),
            mass: 5.0,
            color: BLUE,
            vel: vec2(0.0, -100.0),
            acc: Vec2::ZERO,
            trace: VecDeque::with_capacity(DEFAULT_SETTINGS.tail_length),
        },
    ]
}

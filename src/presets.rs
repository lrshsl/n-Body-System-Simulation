use std::collections::VecDeque;

use macroquad::{
    color::{BLUE, DARKGREEN, GREEN, PURPLE, YELLOW},
    math::{Vec2, vec2},
};

use crate::body::Body;

macro_rules! body {
    (
        color: $clr:expr,
        pos: $pos:expr,
        vel: $vel:expr,
        acc: $acc:expr,
        mass: $mass:expr,
        trace: $trace:expr,
    ) => {
        Body {
            pos: $pos,
            mass: $mass,
            color: $clr,
            vel: $vel,
            acc: $acc,
            trace: $trace,
        }
    };
    (
        color: $clr:expr,
        pos: $pos:expr,
        vel: $vel:expr,
        mass: $mass:expr,
    ) => {
        Body {
            pos: $pos,
            mass: $mass,
            color: $clr,
            vel: $vel,
            acc: Vec2::ZERO,
            trace: VecDeque::new(),
        }
    };
    (
        color: $clr:expr,
        pos: $pos:expr,
        vel: $vel:expr,
    ) => {
        Body {
            pos: $pos,
            mass: 10.0,
            color: $clr,
            vel: $vel,
            acc: Vec2::ZERO,
            trace: VecDeque::new(),
        }
    };
}

pub const fn web_default() -> [Body; 4] {
    [
        body! {
            color: PURPLE,
            pos: vec2(1200.0, 500.0),
            vel: vec2(-60.0, -10.0),
            mass: 40.0,
        },
        body! {
            color: GREEN,
            pos: vec2(600.0, 100.0),
            vel: vec2(40.0, -30.0),
        },
        body! {
            color: BLUE,
            pos: vec2(300.0, 800.0),
            vel: vec2(20.0, 0.0),
        },
        body! {
            color: DARKGREEN,
            pos: vec2(600.0, 1000.0),
            vel: vec2(30.0, -30.0),
        },
    ]
}

pub const fn n2_one_large() -> [Body; 2] {
    [
        body! {
            color: GREEN,
            pos: vec2(600.0, 100.0),
            vel: vec2(120.0, -50.0),
            mass: 5.0,
        },
        body! {
            color: YELLOW,
            pos: vec2(1600.0, 1000.0),
            vel: Vec2::ZERO,
            mass: 5.0 * 50.0,
        },
    ]
}

pub const fn n3_one_large() -> [Body; 3] {
    [
        body! {
            color: GREEN,
            pos: vec2(600.0, 100.0),
            vel: vec2(120.0, -50.0),
            mass: 5.0,
        },
        body! {
            color: YELLOW,
            pos: vec2(1600.0, 1000.0),
            vel: Vec2::ZERO,
            mass: 5.0 * 50.0,
        },
        body! {
            color: BLUE,
            pos: vec2(1000.0, 1550.0),
            vel: vec2(-161.0, -100.0),
            mass: 5.0,
        },
    ]
}

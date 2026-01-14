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

pub fn web_default() -> [Body; 4] {
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

pub fn two_balanced() -> [Body; 2] {
    [
        body! {
            color: GREEN,
            pos: vec2(1000.0, 1200.0),
            vel: vec2(0.0, -75.0),
            mass: 50.0,
        },
        body! {
            color: BLUE,
            pos: vec2(1600.0, 1200.0),
            vel: vec2(0.0, 75.0),
            mass: 50.0,
        },
    ]
}

pub fn circular() -> [Body; 3] {
    const G: f32 = 1e7;
    let r = 1000.0;
    let m_sun = 50.0;
    let pos_sun = vec2(1600.0, 600.0);
    let pos_sat1 = pos_sun.with_x(pos_sun.x - r);
    let pos_sat2 = pos_sun.with_x(pos_sun.x + r);
    let vel_sat = (G * m_sun / r).sqrt(); // circular orbit: v = sqrt((G M) / r)
    [
        body! {
            color: GREEN,
            pos: pos_sat1,
            vel: vec2(0.0, vel_sat),
            mass: 1.0,
        },
        body! {
            color: BLUE,
            pos: pos_sat2,
            vel: vec2(0.0, -vel_sat),
            mass: 1.0,
        },
        body! {
            color: YELLOW,
            pos: pos_sun,
            vel: Vec2::ZERO,
            mass: m_sun,
        },
    ]
}

pub fn n2_one_large() -> [Body; 2] {
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

pub fn n3_one_large() -> [Body; 3] {
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

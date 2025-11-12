#![feature(f128)]

use std::collections::VecDeque;

use macroquad::{prelude::*, ui::root_ui};

#[derive(Clone, PartialEq)]
pub struct Body {
    pos: Vec2,
    mass: f32,
    color: Color,
    vel: Vec2,
    acc: Vec2,
    trace: VecDeque<Vec2>,
}

pub struct MainState {
    next_tail_update: f64,
}

pub struct Settings {
    time_scale: f32,
    tail_length: usize,
    tail_delta_ms: f64,
}

const DEFAULT_SETTINGS: Settings = Settings {
    time_scale: 1.0,
    tail_delta_ms: 1e-6,
    tail_length: 200,
};

#[macroquad::main("n-Body-Problem Simulation")]
async fn main() {
    let mut bodies = [
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
            mass: 5.0 * 5.0,
            color: RED,
            vel: vec2(-150.0 / 5.0, 100.0 / 5.0),
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
    ];

    let mut state = MainState {
        next_tail_update: 0.0,
    };
    let settings = DEFAULT_SETTINGS;

    // Tweak initial settings
    loop {
        draw_bodies(&bodies);

        if root_ui().button(vec2(10.0, 10.0), "Start".to_owned()) {
            break;
        }

        next_frame().await;
    }

    // Simulation loop
    loop {
        update_bodies(&mut bodies, &settings);
        update_tails(&mut bodies, &settings, &mut state);
        draw_bodies(&bodies);

        next_frame().await
    }
}

fn update_bodies<const N: usize>(bodies: &mut [Body; N], settings: &Settings) {
    let dt = get_frame_time();

    let others = bodies.clone();
    for c in bodies.iter_mut() {
        // Reduced mass
        let masses = others.iter().cloned().map(|x| x.mass);
        let m_red = masses.clone().product::<f32>() / masses.sum::<f32>();

        // Sum all forces
        let f_tot = others
            .iter()
            .filter(|&other| other != c)
            .map(|other| (other.pos - c.pos) * m_red) // F_n = (r_n - r_1) m_red
            .sum::<Vec2>(); // F_tot = sum F_n

        // Update acceleration, velocity and position accordingly
        c.acc = (f_tot / c.mass) * dt * settings.time_scale;
        c.vel += c.acc * dt;
        c.pos += c.vel * dt;
    }
}

fn update_tails(bodies: &mut [Body], settings: &Settings, state: &mut MainState) {
    // Tail
    if get_time() >= state.next_tail_update {
        for c in bodies.iter_mut() {
            update_tail(c, &settings);
            state.next_tail_update = get_time() + settings.tail_delta_ms;
        }
    }
}

fn update_tail(c: &mut Body, settings: &Settings) {
    if c.trace.len() > settings.tail_length {
        c.trace.pop_front();
    }
    c.trace.push_back(c.pos);
}

fn draw_bodies(bodies: &[Body]) {
    for c in bodies.into_iter() {
        let step = 1.0 / c.trace.len() as f32;

        draw_circle(c.pos.x, c.pos.y, c.mass, c.color);
        for (i, pt) in c.trace.iter().enumerate() {
            draw_circle(pt.x, pt.y, 2.0, c.color.with_alpha(i as f32 * step));
        }
    }
}

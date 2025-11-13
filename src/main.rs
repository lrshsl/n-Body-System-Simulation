#![feature(generic_const_exprs)]
#![feature(anonymous_lifetime_in_impl_trait)]

use std::collections::VecDeque;

use macroquad::{prelude::*, ui::root_ui};

use body::Body;
use consts::DEFAULT_SETTINGS;

use crate::{
    draw_functions::{draw_forces, draw_velocities},
    main_state::MainState,
    update_logic::{update_bodies, update_tails},
};

mod body;
mod consts;
mod draw_functions;
mod draw_primitives;
mod main_state;
mod settings;
mod update_logic;

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
        show_forces: false,
    };
    let settings = DEFAULT_SETTINGS;

    // Tweak initial settings
    loop {
        draw_bodies(&bodies);
        draw_forces(&bodies);
        draw_velocities(&bodies);

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
        if state.show_forces {
            draw_forces(&bodies);
        }

        next_frame().await
    }
}

fn get_forces<const N: usize>(body: &Body, bodies: &[Body; N], m_red: f32) -> [Vec2; N - 1] {
    let mut all_forces = [Vec2::new(0.0, 0.0); N - 1];
    let bodies = bodies.iter();

    for (i, other) in bodies.filter(|&b| b != body).enumerate() {
        // F_n = (r_n - r_1) m_red
        all_forces[i] = (other.pos - body.pos) * m_red
    }
    all_forces
}

fn get_mass_reduced(bodies: impl Iterator<Item = &Body> + Clone) -> f32 {
    // Reduced mass
    let masses = bodies.clone().map(|x| x.mass);
    masses.clone().product::<f32>() / masses.sum::<f32>() // m_red = (m_1 m_2 .. m_n) / (m_1 + m_2 + .. + m_n)
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

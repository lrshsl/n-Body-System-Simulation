#![feature(generic_const_exprs)]
#![feature(anonymous_lifetime_in_impl_trait)]

use std::collections::VecDeque;

use macroquad::{prelude::*, ui::root_ui};

use body::Body;
use consts::DEFAULT_SETTINGS;

use crate::{
    draw_functions::{draw_bodies, draw_forces, draw_velocities},
    main_state::MainState,
    update_logic::{update_bodies, update_tails},
};

mod body;
mod consts;
mod draw_functions;
mod draw_primitives;
mod main_state;
mod presets;
mod settings;
mod update_logic;

#[macroquad::main("n-Body-Problem Simulation")]
async fn main() {
    let mut bodies = presets::n3_one_large();
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
        // F_(1, 2) = - G [ (m_1 m_2) / |r_1 - r_2|^3 ] (r_1 - r_2)
        //
        // G is taken care of later, this loop only calculates the forces depending on
        // the particular bodies that interact
        //
        let (m_1, m_2) = (body.mass, other.mass);
        let (r_1, r_2) = (body.pos, other.pos);
        let r_21 = r_1 - r_2;
        let d = r_21.length();
        all_forces[i] = -m_1 * m_2 / d.powi(3) * r_21;
    }
    all_forces
}

fn get_mass_reduced(bodies: impl Iterator<Item = &Body> + Clone) -> f32 {
    // Reduced mass
    let masses = bodies.clone().map(|x| x.mass);
    masses.clone().product::<f32>() / masses.sum::<f32>() // m_red = (m_1 m_2 .. m_n) / (m_1 + m_2 + .. + m_n)
}

#![feature(generic_const_exprs)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![allow(incomplete_features)]

use macroquad::{miniquad::window::screen_size, prelude::*};

use body::Body;
use consts::DEFAULT_SETTINGS;

use crate::{
    draw_functions::{draw_bodies, draw_forces, draw_velocities},
    main_state::MainState,
    ui::{ButtonDrawOptions, button, parameters_panel},
    update_logic::{update_bodies, update_tails},
};

mod body;
mod consts;
mod draw_functions;
mod draw_primitives;
mod main_state;
mod presets;
mod settings;
mod ui;
mod update_logic;

fn cfg() -> Conf {
    Conf {
        window_title: "N-Body-System Simulator".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(cfg)]
async fn main() {
    let mut bodies = presets::n3_one_large();
    let mut state = MainState::default();
    let mut settings = DEFAULT_SETTINGS;

    // Tweak initial settings
    loop {
        draw_bodies(&bodies);
        draw_velocities(&bodies);
        if state.show_forces {
            draw_forces(&bodies, &settings);
        }

        if button(
            "Start Simulation",
            vec2(100.0, 100.0),
            ButtonDrawOptions::default(),
        ) {
            break;
        }
        parameters_panel(&mut settings, &mut state);

        next_frame().await;
    }

    // Simulation loop
    loop {
        update_bodies(&mut bodies, &settings);
        update_tails(&mut bodies, &settings, &mut state);

        parameters_panel(&mut settings, &mut state);

        draw_bodies(&bodies);
        if state.show_forces {
            draw_forces(&bodies, &settings);
        }

        next_frame().await
    }
}

fn get_forces<const N: usize>(body: &Body, bodies: &[Body; N]) -> [Vec2; N - 1] {
    let mut all_forces = [Vec2::new(0.0, 0.0); N - 1];
    let bodies = bodies.iter();

    for (i, other) in bodies.filter(|&b| b != body).enumerate() {
        // F_(1, 2) = - G [ (m_1 m_2) / |r_1 - r_2|^3 ] (r_1 - r_2)
        //
        let (m_1, m_2) = (body.mass, other.mass);
        let (r_1, r_2) = (body.pos, other.pos);
        let r_21 = r_1 - r_2;
        let d = r_21.length();
        all_forces[i] = -m_1 * m_2 / d.powi(3) * r_21;
    }
    all_forces
}

#![feature(mut_ref)]
#![feature(generic_const_exprs)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![allow(incomplete_features)]

use macroquad::prelude::*;

use body::Body;
use consts::DEFAULT_SETTINGS;

use crate::{
    consts::MINIMAL_DRAG_RADIUS,
    draw_functions::{draw_bodies, draw_forces, draw_velocities},
    main_state::{DragState::*, MainState},
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
        fullscreen: false,
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
        draw_bodies(&bodies, &state);
        draw_velocities(&bodies);
        if state.show_forces {
            draw_forces(&bodies, &settings, &state);
        }

        update_initial_state(&mut bodies, &mut state);
        if let Some(DraggingBody(pos, ref mut body)) = state.drag_state {
            let b = bodies
                .iter_mut()
                .find(|b| *b == body)
                .expect("No such body");
            b.pos = pos;
            body.pos = pos;
        }
        if let Some(DraggingForce(pos, ref mut body)) = state.drag_state {
            let b = bodies
                .iter_mut()
                .find(|b| *b == body)
                .expect("No such body");
            b.vel = pos - b.pos;
            body.vel = pos - b.pos;
        }

        if button(
            "Start Simulation",
            vec2(100.0, 100.0),
            ButtonDrawOptions::default(),
        ) {
            break;
        }
        parameters_panel(&mut settings, &mut state);

        if state.debug_mode {
            draw_fps();
        }

        next_frame().await;
    }

    // Simulation loop
    loop {
        update_bodies(&mut bodies, &settings);
        update_tails(&mut bodies, &settings, &mut state);

        parameters_panel(&mut settings, &mut state);

        draw_bodies(&bodies, &state);
        if state.show_forces {
            draw_forces(&bodies, &settings, &state);
        }
        if state.debug_mode {
            draw_fps();
        }

        next_frame().await
    }
}

fn update_initial_state<const N: usize>(bodies: &mut [Body; N], state: &mut MainState) {
    let pos: Vec2 = mouse_position().into();
    match state.drag_state {
        None => {
            if is_mouse_button_pressed(MouseButton::Left) {
                for b in bodies.iter() {
                    let r = b.mass.max(MINIMAL_DRAG_RADIUS);
                    if pos.distance_squared(b.pos) < r * r {
                        state.drag_state = Some(DraggingBody(pos, b.clone()));
                    }
                    if pos.distance_squared(b.pos + b.vel)
                        < MINIMAL_DRAG_RADIUS * MINIMAL_DRAG_RADIUS
                    {
                        state.drag_state = Some(DraggingForce(pos, b.clone()));
                    }
                }
            }
        }
        Some(DraggingBody(ref mut start, _)) => {
            *start = pos;
            if is_mouse_button_released(MouseButton::Left) {
                state.drag_state = None
            }
        }
        Some(DraggingForce(ref mut start, _)) => {
            *start = pos;
            if is_mouse_button_released(MouseButton::Left) {
                state.drag_state = None
            }
        }
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

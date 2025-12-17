#![feature(mut_ref)]
#![feature(generic_const_exprs)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![allow(incomplete_features)]

use macroquad::prelude::*;

use body::Body;
use consts::DEFAULT_SETTINGS;
use micro_ui::{ButtonDrawOptions, button};

use crate::{
    consts::MINIMAL_DRAG_RADIUS,
    draw_functions::{draw_bodies, draw_forces, draw_velocities},
    main_state::{
        DragState::{self, *},
        MainState,
    },
    parameters_panel::parameters_panel,
    update_logic::{update_bodies, update_tails},
};

mod body;
mod consts;
mod draw_functions;
mod draw_primitives;
mod main_state;
mod parameters_panel;
mod presets;
mod settings;
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
    let mut bodies = presets::n2_one_large();
    let mut state = MainState::default();
    let mut settings = DEFAULT_SETTINGS;

    // Tweak initial settings
    loop {
        draw_bodies(&bodies, &state);
        draw_velocities(&bodies);
        if state.show_forces {
            draw_forces(&bodies, &settings, &state);
        }

        if let Some(drag_state) = register_keyboard_input(&bodies, state.drag_state.clone()) {
            state.drag_state = Some(react_on_input(&mut bodies, drag_state));
        } else {
            state.drag_state = None
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

fn react_on_input(bodies: &mut [Body], new_state: DragState) -> DragState {
    match new_state {
        DraggingBody(pos, mut body) => {
            let b = bodies
                .iter_mut()
                .find(|b| **b == body)
                .expect("No such body");
            b.pos = pos;
            body.pos = pos;
            DraggingBody(pos, body)
        }
        DraggingForce(pos, mut body) => {
            let b = bodies
                .iter_mut()
                .find(|b| **b == body)
                .expect("No such body");
            b.vel = pos - b.pos;
            body.vel = pos - b.pos;
            DraggingForce(pos, body)
        }
    }
}

fn register_keyboard_input(
    bodies: &[Body],
    previous_drag_state: Option<DragState>,
) -> Option<DragState> {
    let pos: Vec2 = mouse_position().into();
    match previous_drag_state {
        None => {
            if is_mouse_button_pressed(MouseButton::Left) {
                for b in bodies.iter() {
                    let r = b.mass.max(MINIMAL_DRAG_RADIUS);
                    if pos.distance_squared(b.pos) < r * r {
                        return Some(DraggingBody(pos, b.clone()));
                    }
                    if pos.distance_squared(b.pos + b.vel)
                        < MINIMAL_DRAG_RADIUS * MINIMAL_DRAG_RADIUS
                    {
                        return Some(DraggingForce(pos, b.clone()));
                    }
                }
            }
            None
        }
        Some(DraggingBody(_, b)) => {
            if is_mouse_button_released(MouseButton::Left) {
                None
            } else {
                Some(DraggingBody(pos, b))
            }
        }
        Some(DraggingForce(_, b)) => {
            if is_mouse_button_released(MouseButton::Left) {
                None
            } else {
                Some(DraggingForce(pos, b))
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

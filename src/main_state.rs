use macroquad::math::DVec2;

use crate::body::Body;

#[derive(Debug, Clone)]
pub enum DragState {
    DraggingBody(DVec2, Body),
    DraggingForce(DVec2, Body),
}

pub struct MainState {
    pub show_parameters: bool,
    pub show_forces: bool,
    pub next_tail_update: f64,
    pub show_mass: bool,
    pub show_force_magnitude: bool,
    pub debug_mode: bool,
    pub drag_state: Option<DragState>,

    pub gravitational_constant: f64,
    pub time_scale: f64,
    pub tail_length: usize,
    pub tail_delta_ms: f64,
    pub should_restart: bool,
    pub should_pause: bool,
}

impl Default for MainState {
    fn default() -> Self {
        Self {
            show_forces: false,
            show_parameters: false,
            next_tail_update: 0.0,
            show_mass: true,
            show_force_magnitude: true,
            debug_mode: false,
            drag_state: None,
            gravitational_constant: 1e7,
            time_scale: 1.0,
            tail_delta_ms: 1e-2,
            tail_length: 500,
            should_restart: false,
            should_pause: false,
        }
    }
}

pub struct MainState {
    pub show_parameters: bool,
    pub show_forces: bool,
    pub next_tail_update: f64,
    pub show_mass: bool,
    pub show_force_magnitude: bool,
}

impl Default for MainState {
    fn default() -> Self {
        Self {
            show_forces: false,
            show_parameters: true,
            next_tail_update: 0.0,
            show_mass: true,
            show_force_magnitude: true,
        }
    }
}

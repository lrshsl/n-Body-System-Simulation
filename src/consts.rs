use crate::settings::Settings;

pub const FORCE_ARROW_WIDTH: f32 = 3.0;
pub const FORCE_ARROW_TIP_ANGLE: f32 = 30_f32.to_radians();
pub const FORCE_ARROW_TIP_SIZE_REL: f32 = 0.2;

pub const DEFAULT_SETTINGS: Settings = Settings {
    gravitational_constant: 1e7,
    time_scale: 1.0,
    tail_delta_ms: 1e-1,
    tail_length: 500,
};

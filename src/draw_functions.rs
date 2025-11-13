use macroquad::{color::Color, math::Vec2, shapes::draw_circle};

use crate::{
    body::Body,
    consts::{
        DEFAULT_BODY_SIZE, DEFAULT_FORCE_ARROW_LENGTH, FORCE_ARROW_TIP_ANGLE,
        FORCE_ARROW_TIP_SIZE_REL, FORCE_ARROW_WIDTH,
    },
    draw_primitives::arrow::draw_arrows,
    get_forces,
    main_state::MainState,
    settings::Settings,
};

pub fn draw_velocities(bodies: &[Body]) {
    draw_arrows(
        bodies.iter().map(|b| (b.pos, b.vel, b.color)),
        FORCE_ARROW_WIDTH,
        FORCE_ARROW_TIP_SIZE_REL,
        FORCE_ARROW_TIP_ANGLE,
    );
}

pub fn draw_forces<const N: usize>(bodies: &[Body; N], settings: &Settings, state: &MainState)
where
    [(); N - 1]:,
{
    let avg_mass = bodies.iter().map(|b| b.mass).sum::<f32>() / N as f32;
    for b in bodies.iter() {
        let other_colors = bodies.iter().filter(|&o| o != b).map(|o| o.color);
        draw_arrows(
            get_forces(b, bodies)
                .into_iter()
                .zip(other_colors) // Use colors from the target
                .map(|(f, c): (Vec2, Color)| {
                    (
                        b.pos,
                        if state.show_force_magnitude {
                            f / avg_mass * settings.gravitational_constant
                        } else {
                            f.normalize() * DEFAULT_FORCE_ARROW_LENGTH
                        },
                        c,
                    )
                }),
            1.0,
            FORCE_ARROW_TIP_SIZE_REL,
            FORCE_ARROW_TIP_ANGLE,
        );
    }
}

pub fn draw_bodies(bodies: &[Body], state: &MainState) {
    for c in bodies.iter() {
        let step = 1.0 / c.trace.len() as f32;
        let radius = if state.show_mass {
            c.mass
        } else {
            DEFAULT_BODY_SIZE
        };

        draw_circle(c.pos.x, c.pos.y, radius, c.color);
        for (i, pt) in c.trace.iter().enumerate() {
            draw_circle(pt.x, pt.y, 2.0, c.color.with_alpha(i as f32 * step));
        }
    }
}

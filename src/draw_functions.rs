use macroquad::shapes::draw_circle;

use crate::{
    body::Body,
    consts::{FORCE_ARROW_TIP_ANGLE, FORCE_ARROW_TIP_SIZE_REL, FORCE_ARROW_WIDTH},
    draw_primitives::arrow::draw_arrows,
    get_forces,
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

pub fn draw_forces<const N: usize>(bodies: &[Body; N], settings: &Settings)
where
    [(); N - 1]:,
{
    for b in bodies.iter() {
        let other_colors = bodies.iter().filter(|&o| o != b).map(|o| o.color);
        draw_arrows(
            get_forces(b, bodies)
                .into_iter()
                .zip(other_colors) // Use colors from the target
                .map(|(f, c)| (b.pos, f / b.mass * settings.gravitational_constant, c)),
            1.0,
            FORCE_ARROW_TIP_SIZE_REL,
            FORCE_ARROW_TIP_ANGLE,
        );
    }
}

pub fn draw_bodies(bodies: &[Body]) {
    for c in bodies.into_iter() {
        let step = 1.0 / c.trace.len() as f32;

        draw_circle(c.pos.x, c.pos.y, c.mass, c.color);
        for (i, pt) in c.trace.iter().enumerate() {
            draw_circle(pt.x, pt.y, 2.0, c.color.with_alpha(i as f32 * step));
        }
    }
}

use crate::{
    body::Body,
    consts::{FORCE_ARROW_TIP_ANGLE, FORCE_ARROW_TIP_SIZE, FORCE_ARROW_WIDTH},
    draw_primitives::arrow::draw_arrows,
    get_forces, get_mass_reduced,
};

pub fn draw_velocities(bodies: &[Body]) {
    draw_arrows(
        bodies.iter().map(|b| (b.pos, b.vel, b.color)),
        FORCE_ARROW_WIDTH,
        FORCE_ARROW_TIP_SIZE,
        FORCE_ARROW_TIP_ANGLE,
    );
}

pub fn draw_forces<const N: usize>(bodies: &[Body; N])
where
    [(); N - 1]:,
{
    let mass_reduced = get_mass_reduced(bodies.iter());
    for b in bodies.iter() {
        let other_colors = bodies.iter().filter(|&o| o != b).map(|o| o.color);
        draw_arrows(
            get_forces(b, bodies, mass_reduced)
                .into_iter()
                .zip(other_colors) // Use colors from the target
                .map(|(f, c)| (b.pos, f * 1e-2, c)),
            1.0,
            FORCE_ARROW_TIP_SIZE,
            FORCE_ARROW_TIP_ANGLE,
        );
    }
}

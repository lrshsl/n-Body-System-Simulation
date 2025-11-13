use macroquad::{
    math::Vec2,
    time::{get_frame_time, get_time},
};

use crate::{Body, get_forces, get_mass_reduced, main_state::MainState, settings::Settings};

pub fn update_bodies<const N: usize>(bodies: &mut [Body; N], settings: &Settings)
where
    [(); N - 1]:,
{
    let dt = get_frame_time();

    let bodies_iter = bodies.iter();
    let m_red = get_mass_reduced(bodies_iter.clone());

    // Sum up all forces per body
    let mut forces = [Vec2::ZERO; N];
    for (cur_i, cur_b) in bodies_iter.enumerate() {
        // Sum all forces
        let f_tot = get_forces(&cur_b, &bodies, m_red).iter().sum::<Vec2>(); // F_tot = sum F_n
        forces[cur_i] = f_tot;
    }

    // Apply F_tot to each body
    for (b, f_tot) in bodies.iter_mut().zip(forces) {
        // Calculate effect on acceleration
        let acc = (f_tot / b.mass) * settings.gravitational_constant;

        // Update acceleration, velocity and position accordingly
        b.acc = acc * dt * settings.time_scale;
        b.vel += b.acc * dt;
        b.pos += b.vel * dt;
    }
}

pub fn update_tails(bodies: &mut [Body], settings: &Settings, state: &mut MainState) {
    // Tail
    if get_time() >= state.next_tail_update {
        for c in bodies.iter_mut() {
            update_tail(c, &settings);
            state.next_tail_update = get_time() + settings.tail_delta_ms;
        }
    }
}

fn update_tail(c: &mut Body, settings: &Settings) {
    if c.trace.len() > settings.tail_length {
        c.trace.pop_front();
    }
    c.trace.push_back(c.pos);
}

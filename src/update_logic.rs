use macroquad::{
    math::Vec2,
    time::{get_frame_time, get_time},
};

use crate::{Body, get_forces, main_state::MainState, settings::Settings};

pub fn update_bodies<const N: usize>(bodies: &mut [Body; N], settings: &Settings)
where
    [(); N - 1]:,
{
    let dt = get_frame_time();

    let bodies_iter = bodies.clone().into_iter();

    // Sum up all forces per body
    let mut forces = [Vec2::ZERO; N];
    for (i, b) in bodies_iter.clone().enumerate() {
        // Sum all forces F_tot = sum F_n
        let f_tot_non_scaled = get_forces(&b, bodies).iter().sum::<Vec2>();

        // Apply G only once
        forces[i] = f_tot_non_scaled * settings.gravitational_constant;
    }

    // Apply F_tot to each body
    let time_step = dt * settings.time_scale;
    for (b, f_tot) in bodies.iter_mut().zip(forces) {
        // Calculate new acceleration
        b.acc = f_tot / b.mass;

        // Update acceleration, velocity and position accordingly
        b.vel += b.acc * time_step;
        b.pos += b.vel * time_step;
    }

    // Collisions
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            let (left, right) = bodies.split_at_mut(j);
            let (a, b) = (&mut left[i], &mut right[0]);

            let (ax, ay) = a.pos.into();
            let (bx, by) = b.pos.into();
            let r = a.radius() + b.radius();

            // Check if collision might be possible
            if (ax - bx).abs() < r && (ay - by).abs() < r {
                handle_collision(a, b, dt * settings.time_scale);
            }
        }
    }
}

fn get_f_tot<const N: usize>(body: &Body, bodies: &[Body; N], g: f32) -> Vec2 {
    let mut f_tot = Vec2::ZERO;
    let bodies = bodies.iter();

    for other in bodies.filter(|&b| b != body) {
        // F_(1, 2) = - G [ (m_1 m_2) / |r_1 - r_2|^3 ] (r_1 - r_2)
        //
        let (m_1, m_2) = (body.mass, other.mass);
        let (p_1, p_2) = (body.pos, other.pos);

        let pos_diff = p_1 - p_2;
        let d = pos_diff.length();

        f_tot += -m_1 * m_2 / d.powi(3) * pos_diff;
    }
    f_tot
}

fn handle_collision(a: &mut Body, b: &mut Body, time_scale: f32) {
    let d_squared = a.pos.distance_squared(b.pos);
    let r_squared = (a.radius() + b.radius()) * (a.radius() + b.radius());
    if d_squared <= r_squared {
        // Step back
        a.pos -= a.vel * time_scale;

        let d = b.pos - a.pos;
        let angle_a = a.vel.angle_between(d);
        let angle_b = b.vel.angle_between(d);
        a.vel = -Vec2::from_angle(2.0 * angle_a).rotate(a.vel);
        b.vel = -Vec2::from_angle(2.0 * angle_b).rotate(b.vel);
    }
}

pub fn update_tails(bodies: &mut [Body], settings: &Settings, state: &mut MainState) {
    // Tail
    if get_time() >= state.next_tail_update {
        for c in bodies.iter_mut() {
            update_tail(c, settings);
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

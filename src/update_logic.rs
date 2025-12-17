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
    for (cur_i, cur_b) in bodies_iter.clone().enumerate() {
        // Sum all forces
        let f_tot = get_forces(&cur_b, bodies).iter().sum::<Vec2>(); // F_tot = sum F_n
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

fn handle_collision(a: &mut Body, b: &mut Body, time_scale: f32) {
    let d2 = a.pos.distance_squared(b.pos);
    let r2 = (a.radius() + b.radius()) * (a.radius() + b.radius());
    if d2 <= r2 {
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

#![feature(generic_const_exprs)]

use std::collections::VecDeque;

use macroquad::{prelude::*, ui::root_ui};

#[derive(Clone, PartialEq)]
pub struct Body {
    pos: Vec2,
    mass: f32,
    color: Color,
    vel: Vec2,
    acc: Vec2,
    trace: VecDeque<Vec2>,
}

pub struct MainState {
    next_tail_update: f64,
}

pub struct Settings {
    gravitational_constant: f32,
    time_scale: f32,
    tail_length: usize,
    tail_delta_ms: f64,
}

const FORCE_ARROW_WIDTH: f32 = 3.0;
const FORCE_ARROW_TIP_ANGLE: f32 = 30_f32.to_radians();
const FORCE_ARROW_TIP_SIZE: f32 = 30.0;

const DEFAULT_SETTINGS: Settings = Settings {
    gravitational_constant: 1e1,
    time_scale: 1.0,
    tail_delta_ms: 1e-6,
    tail_length: 200,
};

#[macroquad::main("n-Body-Problem Simulation")]
async fn main() {
    let mut bodies = [
        Body {
            pos: vec2(800.0, 100.0),
            mass: 5.0,
            color: GREEN,
            vel: vec2(150.0, 0.0),
            acc: Vec2::ZERO,
            trace: VecDeque::with_capacity(DEFAULT_SETTINGS.tail_length),
        },
        Body {
            pos: vec2(1600.0, 800.0),
            mass: 5.0 * 5.0,
            color: RED,
            vel: vec2(-150.0 / 5.0, 100.0 / 5.0),
            acc: Vec2::ZERO,
            trace: VecDeque::with_capacity(DEFAULT_SETTINGS.tail_length),
        },
        Body {
            pos: vec2(300.0, 1550.0),
            mass: 5.0,
            color: BLUE,
            vel: vec2(0.0, -100.0),
            acc: Vec2::ZERO,
            trace: VecDeque::with_capacity(DEFAULT_SETTINGS.tail_length),
        },
    ];

    let mut state = MainState {
        next_tail_update: 0.0,
    };
    let settings = DEFAULT_SETTINGS;

    // Tweak initial settings
    loop {
        draw_bodies(&bodies);
        draw_velocities(&bodies);

        if root_ui().button(vec2(10.0, 10.0), "Start".to_owned()) {
            break;
        }

        next_frame().await;
    }

    // Simulation loop
    loop {
        update_bodies(&mut bodies, &settings);
        update_tails(&mut bodies, &settings, &mut state);
        draw_bodies(&bodies);

        next_frame().await
    }
}

fn draw_velocities(bodies: &[Body]) {
    draw_arrows(bodies.iter().map(|b| (b.pos, b.vel, b.color)));
}

fn draw_forces(bodies: &[Body]) {
    draw_arrows(bodies.iter().map(|b| (b.pos, b.vel, b.color)));
}

fn get_forces<const N: usize>(body: &Body, bodies: &[Body; N], m_red: f32) -> [Vec2; N - 1] {
    let mut all_forces = [Vec2::new(0.0, 0.0); N - 1];
    let bodies = bodies.iter();

    for (i, f) in bodies
        .filter(|&other| other != body)
        .map(|other| (other.pos - body.pos) * m_red)
        .enumerate()
    // F_n = (r_n - r_1) m_red
    {
        all_forces[i] = f
    }
    all_forces
}

fn update_bodies<const N: usize>(bodies: &mut [Body; N], settings: &Settings)
where
    [(); N - 1]:,
{
    let dt = get_frame_time();

    let bodies_iter = bodies.iter();

    // Reduced mass
    let masses = bodies_iter.clone().map(|x| x.mass);
    let m_red = masses.clone().product::<f32>() / masses.sum::<f32>();

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

fn update_tails(bodies: &mut [Body], settings: &Settings, state: &mut MainState) {
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

fn draw_arrows(data: impl IntoIterator<Item = (Vec2, Vec2, Color)>) {
    for (pos, vel, color) in data.into_iter() {
        let end = pos + vel;
        draw_line(pos.x, pos.y, end.x, end.y, FORCE_ARROW_WIDTH, color);
        for angle_dev in [1.0, -1.0] {
            let a = vel.to_angle() + FORCE_ARROW_TIP_ANGLE * angle_dev;
            let pt = end + FORCE_ARROW_TIP_SIZE * vec2(-a.cos(), -a.sin());
            draw_line(end.x, end.y, pt.x, pt.y, FORCE_ARROW_WIDTH, color);
        }
    }
}

fn draw_bodies(bodies: &[Body]) {
    for c in bodies.into_iter() {
        let step = 1.0 / c.trace.len() as f32;

        draw_circle(c.pos.x, c.pos.y, c.mass, c.color);
        for (i, pt) in c.trace.iter().enumerate() {
            draw_circle(pt.x, pt.y, 2.0, c.color.with_alpha(i as f32 * step));
        }
    }
}

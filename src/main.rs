use std::collections::VecDeque;

use macroquad::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Body {
    pos: Vec2,
    mass: f32,
    color: Color,
    vel: Vec2,
    acc: Vec2,
    trace: VecDeque<Vec2>,
}

const N_TRACE: usize = 100;

#[macroquad::main("TBP")]
async fn main() {
    let mut bodies = [
        Body {
            pos: vec2(300.0, 100.0),
            mass: 5.0,
            color: GREEN,
            vel: vec2(150.0, 0.0),
            acc: Vec2::ZERO,
            trace: VecDeque::with_capacity(N_TRACE),
        },
        Body {
            pos: vec2(300.0, 250.0),
            mass: 20.0,
            color: RED,
            vel: vec2(0.0, 0.0),
            acc: Vec2::ZERO,
            trace: VecDeque::with_capacity(N_TRACE),
        },
        Body {
            pos: vec2(300.0, 400.0),
            mass: 5.0,
            color: BLUE,
            vel: vec2(-150.0, 0.0),
            acc: Vec2::ZERO,
            trace: VecDeque::with_capacity(N_TRACE),
        },
    ];
    let time_scale = 30.0;
    loop {
        let dt = get_frame_time();

        for c in bodies.clone().into_iter() {
            draw_circle(c.pos.x, c.pos.y, c.mass, c.color);
            for (i, pt) in c.trace.iter().enumerate() {
                let step = 1.0 / N_TRACE as f32;
                draw_circle(pt.x, pt.y, 1.0, c.color.with_alpha(step * i as f32));
            }
        }

        let others = bodies.clone();
        for c in bodies.iter_mut() {
            update(c, dt);

            // Masse reduite
            let masses = others.iter().cloned().map(|x| x.mass);
            let m_red = masses.clone().product::<f32>() / masses.sum::<f32>();

            // Sum all forces
            let f_tot = others
                .iter()
                .filter(|&other| other != c)
                .map(|other| (other.pos - c.pos) * m_red) // F = (r2 - r1) m_red
                .map(|f| f * dt * time_scale)
                .sum::<Vec2>();
            c.acc = f_tot / c.mass;

            // Tail
            if c.trace.len() > N_TRACE {
                c.trace.pop_front();
            }
            c.trace.push_back(c.pos);
        }

        next_frame().await
    }
}

fn update(body: &mut Body, dt: f32) {
    body.vel += body.acc * dt;
    body.pos += body.vel * dt;
}

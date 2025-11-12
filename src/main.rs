use std::collections::VecDeque;

use macroquad::prelude::*;

#[derive(Clone)]
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
    let mut c1 = Body {
        pos: vec2(300.0, 100.0),
        mass: 20.0,
        color: GREEN,
        vel: vec2(200.0, 0.0),
        acc: Vec2::ZERO,
        trace: VecDeque::with_capacity(N_TRACE),
    };
    let mut c2 = Body {
        pos: vec2(300.0, 400.0),
        mass: 2.0,
        color: RED,
        vel: vec2(-200.0, 0.0),
        acc: Vec2::ZERO,
        trace: VecDeque::with_capacity(N_TRACE),
    };
    let time_scale = 30.0;
    loop {
        let dt = get_frame_time();

        for c in [&c1, &c2] {
            draw_circle(c.pos.x, c.pos.y, c.mass, c.color);
            for (i, pt) in c.trace.iter().enumerate() {
                let step = 1.0 / N_TRACE as f32;
                draw_circle(pt.x, pt.y, 1.0, c.color.with_alpha(step * i as f32));
            }
        }

        let others = [c1.clone(), c2.clone()];
        for c in [&mut c1, &mut c2] {
            update(c, dt);

            // Masse reduite
            let masses = others.iter().map(|x| x.mass);
            let m_red = masses.clone().product::<f32>() / masses.sum::<f32>();

            // Sum all forces
            c.acc = Vec2::ZERO;
            for other in others.iter() {
                let f = (other.pos - c.pos) * m_red;
                c.acc += f * dt * time_scale;
            }

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

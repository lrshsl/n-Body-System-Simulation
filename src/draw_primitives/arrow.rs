use macroquad::{
    color::Color,
    math::{Vec2, vec2},
    shapes::draw_line,
};

pub fn draw_arrows(
    data: impl IntoIterator<Item = (Vec2, Vec2, Color)>,
    width: f32,
    tip_size: f32,
    tip_angle: f32,
) {
    for (pos, vel, color) in data.into_iter() {
        draw_arrow(pos, vel, color, width, tip_size, tip_angle);
    }
}

pub fn draw_arrow(
    origin: Vec2,
    direction: Vec2,
    color: Color,
    width: f32,
    tip_size_rel: f32,
    tip_angle: f32,
) {
    let end = origin + direction;
    draw_line(origin.x, origin.y, end.x, end.y, width, color);
    for angle_dev in [1.0, -1.0] {
        let a = direction.to_angle() + tip_angle * angle_dev;
        let pt = end + tip_size_rel * direction.length() * vec2(-a.cos(), -a.sin());
        draw_line(end.x, end.y, pt.x, pt.y, width, color);
    }
}

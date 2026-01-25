use macroquad::{
    color::Color,
    math::{DVec2, vec2},
    shapes::draw_line,
};

pub fn draw_arrows(
    data: impl IntoIterator<Item = (DVec2, DVec2, Color)>,
    width: f32,
    tip_size: f32,
    tip_angle: f32,
) {
    for (pos, vel, color) in data.into_iter() {
        draw_arrow(pos, vel, color, width, tip_size, tip_angle);
    }
}

pub fn draw_arrow(
    origin: DVec2,
    direction: DVec2,
    color: Color,
    width: f32,
    tip_size_rel: f32,
    tip_angle: f32,
) {
    let end = origin + direction;
    draw_line(
        origin.x as f32,
        origin.y as f32,
        end.x as f32,
        end.y as f32,
        width,
        color,
    );
    for angle_dev in [1.0, -1.0] {
        let a = (direction.to_angle() + angle_dev) as f32 * tip_angle;
        let pt =
            end.as_vec2() + tip_size_rel * (direction.length() as f32) * vec2(-a.cos(), -a.sin());
        draw_line(end.x as f32, end.y as f32, pt.x, pt.y, width, color);
    }
}

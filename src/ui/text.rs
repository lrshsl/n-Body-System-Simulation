use macroquad::{
    math::Vec2,
    text::{TextParams, draw_text_ex, get_text_center},
};

pub fn draw_text_center(text: &str, center: Vec2, opts: TextParams) {
    let local_center = get_text_center(
        text,
        opts.font,
        opts.font_size,
        opts.font_scale,
        opts.rotation,
    );
    draw_text_ex(
        text,
        center.x - local_center.x,
        center.y - local_center.y,
        opts,
    );
}

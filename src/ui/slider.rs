use macroquad::{
    color::{Color, WHITE},
    color_u8,
    input::{is_mouse_button_down, mouse_position},
    math::{FloatExt as _, Rect, Vec2, vec2},
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::TextParams,
};

use crate::ui::draw_text_center;

#[derive(Clone, Copy)]
pub struct SliderDrawOptions {
    pub size: Vec2,
    pub text_color: Color,
    pub font_size: u16,
    pub fill_color: Color,
    pub border: bool,
    pub border_width: f32,
    pub border_color: Color,
}

impl Default for SliderDrawOptions {
    fn default() -> Self {
        Self {
            size: vec2(600.0, 60.0),
            text_color: WHITE,
            font_size: 40,
            fill_color: color_u8!(0, 255, 0, 255),
            border: true,
            border_width: 5.0,
            border_color: WHITE,
        }
    }
}

pub fn slider(
    text: &'static str,
    topleft: Vec2,
    value: f32,
    min: f32,
    max: f32,
    opts: SliderDrawOptions,
) -> f32 {
    // Draw border
    if opts.border {
        draw_rectangle_lines(
            topleft.x,
            topleft.y,
            opts.size.x,
            opts.size.y,
            opts.border_width,
            opts.border_color,
        );
    }

    // Draw progress
    let w = opts.size.x - 2.0 * opts.border_width;
    draw_rectangle(
        topleft.x + opts.border_width,
        topleft.y + opts.border_width,
        w * value.clamp(min, max).remap(min, max, 0.0, 1.0),
        opts.size.y - 2.0 * opts.border_width,
        opts.fill_color,
    );

    // Draw text
    let text_opts = TextParams {
        font_size: opts.font_size,
        color: opts.text_color,
        ..Default::default()
    };
    let text = format!("{text} : {value:.2}");
    draw_text_center(&text, topleft + opts.size * 0.5, text_opts);

    // Check for drag event in this area
    let rect = Rect::new(topleft.x, topleft.y, opts.size.x, opts.size.y);
    let dragged = is_mouse_button_down(macroquad::input::MouseButton::Left)
        && rect.contains(mouse_position().into());

    if dragged {
        let pos: Vec2 = mouse_position().into();
        min.lerp(
            max,
            (pos.x - topleft.x) / (opts.size.x - 2.0 * opts.border_width),
        )
    } else {
        value
    }
}

use macroquad::{
    color::{Color, DARKGRAY, WHITE},
    input::{is_mouse_button_pressed, mouse_position},
    math::{Rect, Vec2, vec2},
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::{TextParams, draw_text_ex, get_text_center},
};

#[derive(Clone, Copy)]
pub struct ButtonDrawOptions {
    pub size: Vec2,
    pub text_color: Color,
    pub font_size: u16,
    pub fill: bool,
    pub fill_color: Option<Color>,
    pub border: bool,
    pub border_width: f32,
    pub border_color: Color,
}

impl Default for ButtonDrawOptions {
    fn default() -> Self {
        Self {
            size: vec2(300.0, 60.0),
            text_color: WHITE,
            font_size: 40,
            fill: false,
            fill_color: None,
            border: true,
            border_width: 5.0,
            border_color: WHITE,
        }
    }
}

pub fn button(text: &'static str, topleft: Vec2, opts: ButtonDrawOptions) -> bool {
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

    // Draw background
    if opts.fill {
        draw_rectangle(
            topleft.x + opts.border_width,
            topleft.y + opts.border_width,
            opts.size.x - 2.0 * opts.border_width,
            opts.size.y - 2.0 * opts.border_width,
            opts.fill_color.unwrap_or(DARKGRAY),
        );
    }

    // Draw text
    let params = TextParams {
        font_size: opts.font_size,
        color: opts.text_color,
        ..Default::default()
    };
    let center = get_text_center(
        text,
        params.font,
        params.font_size,
        params.font_scale,
        params.rotation,
    );
    draw_text_ex(
        text,
        topleft.x + opts.size.x * 0.5 - center.x,
        topleft.y + opts.size.y * 0.5 - center.y,
        params,
    );

    // Check for click event in this area
    let rect = Rect::new(topleft.x, topleft.y, opts.size.x, opts.size.y);
    is_mouse_button_pressed(macroquad::input::MouseButton::Left)
        && rect.contains(mouse_position().into())
}

use macroquad::{
    color::WHITE,
    math::{Vec2, vec2},
    miniquad::window::screen_size,
    shapes::draw_rectangle_lines,
};

use crate::{
    main_state::MainState,
    settings::Settings,
    ui::button::{ButtonDrawOptions, button},
};

pub fn parameters_panel(settings: &mut Settings, state: &mut MainState) {
    let screen_size: Vec2 = screen_size().into();

    if button(
        if state.show_parameters { "x" } else { "=" },
        vec2(screen_size.x - 100.0, 100.0),
        ButtonDrawOptions {
            size: vec2(50.0, 50.0),
            border_width: 1.5,
            ..Default::default()
        },
    ) {
        state.show_parameters = !state.show_parameters;
    }

    if !state.show_parameters {
        return;
    }

    let topleft = vec2(0.7, 0.1) * screen_size;
    let panel_size = vec2(0.3, 0.8) * screen_size;

    let margin = vec2(0.01, 0.01) * screen_size;

    draw_rectangle_lines(
        topleft.x,
        topleft.y,
        panel_size.x,
        panel_size.y,
        3.0,
        WHITE.with_alpha(30.0),
    );
    if button(
        "Show Forces",
        topleft + margin,
        ButtonDrawOptions::default(),
    ) {
        state.show_forces = !state.show_forces;
    }
}

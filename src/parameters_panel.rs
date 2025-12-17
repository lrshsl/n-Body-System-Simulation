use macroquad::{
    color::WHITE,
    math::{Vec2, vec2},
    miniquad::window::screen_size,
    shapes::draw_rectangle_lines,
};

use micro_ui::{
    SliderDrawOptions,
    ButtonDrawOptions, button,
    slider,
};
use crate::{
    main_state::MainState,
    settings::Settings,
};

pub fn parameters_panel(settings: &mut Settings, state: &mut MainState) {
    let screen_size: Vec2 = screen_size().into();
    let margin = vec2(0.01, 0.01) * screen_size;

    if button(
        if state.show_parameters { "x" } else { "=" },
        vec2(screen_size.x - margin.x - 50.0, margin.y),
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

    let button_opts = ButtonDrawOptions::default();
    let one_button_width = (button_opts.size + margin).with_y(0.0);
    let one_button_height = (button_opts.size + margin).with_x(0.0);

    let panel_width = 3.0 * one_button_width.x + margin.x;
    let topleft = vec2(screen_size.x - panel_width - margin.x, 0.05 * screen_size.y);
    let panel_size = vec2(panel_width, 0.9 * screen_size.y);

    // Draw border
    draw_rectangle_lines(
        topleft.x,
        topleft.y,
        panel_size.x,
        panel_size.y,
        3.0,
        WHITE.with_alpha(30.0),
    );

    // Draw content
    if button("Show Forces", topleft + margin, button_opts) {
        state.show_forces = !state.show_forces;
    }
    if button(
        "Show mass",
        topleft + margin + one_button_width,
        button_opts,
    ) {
        state.show_mass = !state.show_mass;
    }
    if button(
        "Show F-Magnitude",
        topleft + margin + 2.0 * one_button_width,
        button_opts,
    ) {
        state.show_force_magnitude = !state.show_force_magnitude;
    }
    if button(
        "Debug Mode",
        topleft + margin + one_button_height,
        button_opts,
    ) {
        state.debug_mode = !state.debug_mode;
    }

    settings.tail_length = slider(
        "Tail Size",
        topleft + margin + 2.0 * one_button_height,
        settings.tail_length as f32,
        0.0,
        1e4,
        SliderDrawOptions::default(),
    ) as usize;
    settings.tail_delta_ms = 1.0
        - slider(
            "Tail Resolution",
            topleft + margin + 3.0 * one_button_height,
            1.0 - settings.tail_delta_ms as f32,
            0.0,
            1.0,
            SliderDrawOptions::default(),
        ) as f64;
    settings.time_scale = slider(
        "Time Scale",
        topleft + margin + 4.0 * one_button_height,
        settings.time_scale,
        0.0,
        10.0,
        SliderDrawOptions::default(),
    );
}

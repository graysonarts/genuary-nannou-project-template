use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use nannou::prelude::*;

use crate::model::{Model, RunMode};

pub fn projname() -> Option<String> {
    let result = std::env::current_exe()
        .ok()
        .as_ref()
        .map(PathBuf::as_path)
        .map(Path::file_stem)?
        .map(OsStr::to_string_lossy)
        .map(|name| format!("{} {}", name, env!("VERGEN_GIT_DESCRIBE")));
    // .map(|x| x.map(|y| y.to_string()));

    result
}

pub fn label(app: &App, model: &Model, draw: &Draw) -> () {
    let win_rect = app.main_window().rect();

    let text_rect = win_rect.pad_bottom(20.0).pad_right(10.0);

    if model.run_mode == RunMode::Debug {
        // Draw Origin
        draw.line()
            .start([-10.0, 0.0].into())
            .end([10.0, 0.0].into())
            .color(WHITE);
        draw.line()
            .start([0.0, -10.0].into())
            .end([0.0, 10.0].into())
            .color(WHITE);
    }

    // Draw bottom bar
    let bottom_bar_rect = Rect::from_x_y_w_h(0.0, win_rect.bottom() + 14.0, win_rect.w(), 27.0);
    draw.rect()
        .xy(bottom_bar_rect.xy())
        .wh(bottom_bar_rect.wh())
        .color(BLACK);

    // Draw Label
    draw.text(&model.label)
        .color(WHITE)
        .font_size(12)
        .align_text_bottom()
        .right_justify()
        .wh(text_rect.wh());
}

pub fn key_released(_app: &App, _model: &mut Model, _key: Key) {}

pub fn setup_logging() {
    pretty_env_logger::init();
}

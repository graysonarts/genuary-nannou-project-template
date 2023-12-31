mod framework;
mod model;
use framework::*;
use model::*;
use nannou::prelude::*;
use once_cell::sync::Lazy;

#[cfg(not(feature = "small"))]
mod settings {
    pub const SCALE: u32 = 2;
    pub const SIZE: u32 = 512 * SCALE;
}
#[cfg(feature = "small")]
mod settings {
    pub const SCALE: u32 = 1;
    pub const SIZE: u32 = 512 * SCALE;
}

use settings::*

pub static ORIGIN_OFFSET: Lazy<Vec3> =
Lazy::new(|| Vec3::new((SIZE as f32) / 2.0, (SIZE as f32) / 2.0, 0.0));

fn main() {
    setup_logging();
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    // Create the main window which is 1024x1024 my standard size for Genuary Sketches
    app.new_window()
        .size(SIZE, SIZE)
        .key_released(key_released)
        .build()
        .unwrap();

    Model::new(RunMode::Production)
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PURPLE);

    label(app, model, &draw);
    draw.to_frame(app, &frame).unwrap();

    if model.save_frame.get() {
        model.save_current_frame(app, &frame);
    }
}

pub fn sketch_key_released(_app: &App, _model: &mut Model, _key: Key) {
    // This will be called if the framework key released handles doesn't handle the key pressed.
}

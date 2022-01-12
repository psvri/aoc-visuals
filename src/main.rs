mod aoc_2018;
mod commons;
mod menu;
use bevy::prelude::*;
use commons::aoc_common::{AOCState, AocFont};

use commons::fps::FpsPlugin;
use commons::window_setup::WindowSetup;

fn setup() -> App {
    let mut app = App::new();
    app.add_startup_system(AocFont::setup_font_resource.system().label("font_init"));
    app.add_plugin(FpsPlugin);
    app.add_plugin(WindowSetup);
    app.add_plugins(DefaultPlugins);
    app.add_state(AOCState {
        year: 0,
        day: 0,
        part: 1,
    });
    app = menu::setup_app(app);
    app = aoc_2018::setup_app(app);
    app
}

fn main() {
    let mut app = setup();
    app.run();
}

use bevy::prelude::App;

mod day1;
mod day2;
mod day3;

pub fn setup_app(mut app: App) -> App {
    app = day1::setup(app);
    app = day2::setup(app);
    app = day3::setup(app);
    app
}

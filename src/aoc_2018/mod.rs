use bevy::prelude::AppBuilder;

mod day1;
mod day2;

pub fn setup_app(mut app: AppBuilder) -> AppBuilder {
    app = day1::setup(app);
    app = day2::setup(app);
    app
}

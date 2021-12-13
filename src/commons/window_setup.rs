use super::{
    aoc_common::{AOCName, BorderSize, ScalableObject},
    constants::*,
};
use bevy::{
    prelude::*,
    render::camera::{Camera, CameraProjection, OrthographicProjection},
};
pub struct WindowSetup;

impl Plugin for WindowSetup {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "Advent of code visuals".to_string(),
            vsync: true,
            ..Default::default()
        });
        app.add_startup_system(camera_setup.system());
        app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));
        app.add_system(camera_zoom.system());
        app.add_system(scale_with_zoom.system());
        app.insert_resource(BorderSize {
            max_x: WINDOW_WIDTH / 2.0,
            max_y: WINDOW_HEIGHT / 2.0,
            current_x: 0.0,
            current_y: 0.0,
        });
    }
}

fn camera_setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(AOCName("AOC Camera".to_string()));
}

fn camera_zoom(
    border_size: Res<BorderSize>,
    window: Res<WindowDescriptor>,
    mut query: Query<(&mut OrthographicProjection, &mut Camera), With<AOCName>>,
) {
    let (mut projection, mut camera) = query.single_mut().unwrap();
    if border_size.current_x > border_size.max_x {
        projection.scale = 1.0
            + ((border_size.current_x as f32 - border_size.max_x as f32)
                / border_size.max_x as f32);
    } else if border_size.current_y > border_size.max_y {
        projection.scale = 1.0
            + ((border_size.current_y as f32 - border_size.max_y as f32)
                / border_size.max_y as f32);
    }
    projection.update(window.width, window.height);
    camera.projection_matrix = projection.get_projection_matrix();
}

fn scale_with_zoom(
    border_size: Res<BorderSize>,
    mut query: Query<&mut Sprite, With<ScalableObject>>,
) {
    for mut sprite in query.iter_mut() {
        if border_size.current_x > border_size.max_x {
            sprite.size[0] = 1.0
                + ((border_size.current_x as f32 - border_size.max_x as f32)
                    / border_size.max_x as f32);
            sprite.size[1] = 1.0
                + ((border_size.current_x as f32 - border_size.max_x as f32)
                    / border_size.max_x as f32);
            sprite.size[0] *= 10.0;
            sprite.size[1] *= 10.0;
        } else if border_size.current_y > border_size.max_y {
            sprite.size[0] = 1.0
                + ((border_size.current_y as f32 - border_size.max_y as f32)
                    / border_size.max_y as f32);
            sprite.size[1] = 1.0
                + ((border_size.current_y as f32 - border_size.max_y as f32)
                    / border_size.max_y as f32);
            sprite.size[0] *= 10.0;
            sprite.size[1] *= 10.0;
        }
    }
}

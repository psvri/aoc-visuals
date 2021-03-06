use super::constants::*;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup_fps)
            .add_system(fps_update_system);
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

fn setup_fps(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load(FONT_PATH);
    commands
        .spawn_bundle(Text2dBundle {
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: handle.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: handle,
                            font_size: 40.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            transform: Transform::from_xyz(
                (WINDOW_WIDTH / 2.0) - 175.0,
                (WINDOW_HEIGHT / 2.0) - 30.0,
                0.0,
            ),
            ..Default::default()
        })
        .insert(FpsText);
}

fn fps_update_system(diagnostics: Res<Diagnostics>, mut query: Query<(&mut Text, With<FpsText>)>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.0.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}

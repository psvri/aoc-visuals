use crate::commons::aoc_common::{AOCState, AocFont, BorderSize, InputLines, ScalableObject};
use crate::commons::constants::WINDOW_WIDTH;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn string_to_i32(data: &str) -> i32 {
    data.parse().unwrap()
}

fn step_calculate_part_1(answer: &mut Answer, change: i32) {
    answer.frequency += change;
}

fn step_calculate_part_2(answer: &mut Answer, change: i32, vistied: &mut VisitedNodes) {
    answer.frequency += change;
    if vistied.vistied_frequencies.contains(&answer.frequency) {
        vistied.found = true;
        println!("I found value {}", answer.frequency);
    } else {
        vistied.vistied_frequencies.insert(answer.frequency);
    }
}

#[derive(Default, Component )]
pub struct Answer {
    pub frequency: i32,
}

#[derive(Component)]
pub struct VisitedNodes {
    pub vistied_frequencies: HashSet<i32>,
    pub found: bool,
}

impl VisitedNodes {
    pub fn new() -> Self {
        let found = false;
        let mut vistied_frequencies = HashSet::<i32>::new();
        vistied_frequencies.insert(0);
        Self {
            vistied_frequencies,
            found,
        }
    }
}

pub fn setup(mut app: App) -> App {
    app.add_system_set(
        SystemSet::on_enter(AOCState {
            year: 2018,
            day: 1,
            part: 1,
        })
        .with_system(app_setup),
    );
    app.add_system_set(
        SystemSet::on_update(AOCState {
            year: 2018,
            day: 1,
            part: 1,
        })
        .with_system(step_part1_system)
        .with_system(scale_sprite),
    );
    app.add_system_set(
        SystemSet::on_enter(AOCState {
            year: 2018,
            day: 1,
            part: 2,
        })
        .with_system(app_setup),
    );
    app.add_system_set(
        SystemSet::on_update(AOCState {
            year: 2018,
            day: 1,
            part: 2,
        })
        .with_system(step_part2_system)
        .with_system(scale_sprite),
    );
    app
}

fn app_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    aoc_font: Res<AocFont>,
) {
    commands.insert_resource(Answer { frequency: 0 });
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "This text is in the 2D scene.",
                TextStyle {
                    font: aoc_font.font_handle.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            transform: Transform::from_xyz(200.0 - (WINDOW_WIDTH / 2.0), 0.0, 0.0),
            ..Default::default()
        })
        .insert(InputLines::from_slice(include_bytes!(
            "../../inputs/2018/day1.txt"
        )))
        .insert(VisitedNodes::new())
        .insert(ScalableObject);
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                color: Color::rgb(1.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScalableObject);
}

fn step_part1_system(
    mut answer: ResMut<Answer>,
    mut query: Query<(&mut Text, &mut InputLines)>,
    mut border_size: ResMut<BorderSize>,
) {
    let (mut text, mut input_line) = query.single_mut();
    if let Some(i) = input_line.next(false) {
        step_calculate_part_1(&mut answer, string_to_i32(i));
        border_size.current_y = scale_log_value(answer.frequency as f32);
    }
    text.sections[0].value = format!("answer: {}", answer.frequency);
}

fn step_part2_system(
    mut answer: ResMut<Answer>,
    mut query: Query<(&mut Text, &mut InputLines, &mut VisitedNodes)>,
    mut border_size: ResMut<BorderSize>,
) {
    let (mut text, mut input_line, mut vistied_nodes) = query.single_mut();
    for _ in 0..50 {
        if !vistied_nodes.found {
            if let Some(i) = input_line.next(true) {
                step_calculate_part_2(&mut answer, string_to_i32(i), &mut vistied_nodes);
                border_size.current_y = scale_log_value(answer.frequency as f32);
            }
        }
        text.sections[0].value = format!("answer: {}", answer.frequency);
    }
}

fn scale_sprite(answer: Res<Answer>, mut query: Query<&mut Sprite, With<ScalableObject>>) {
    let mut sprite = query.single_mut();
    sprite.custom_size = Some(Vec2::new(100.0, scale_log_value(answer.frequency as f32)));
}

fn scale_log_value(data: f32) -> f32 {
    data.log2() * 20.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(string_to_i32("+1"), 1);
        assert_eq!(string_to_i32("-1"), -1);
    }

    #[test]
    fn test_step_calculate_part_1() {
        let mut answer = Answer { frequency: 0 };
        step_calculate_part_1(&mut answer, 1);
        assert_eq!(answer.frequency, 1);
    }

    #[test]
    fn test_step_calculate_part_2() {
        let mut answer = Answer { frequency: 0 };
        let mut vistied = VisitedNodes::new();
        step_calculate_part_2(&mut answer, 1, &mut vistied);
        assert_eq!(answer.frequency, 1);
        assert_eq!(vistied.found, false);
        assert!(vistied.vistied_frequencies.contains(&0));
        assert!(vistied.vistied_frequencies.contains(&1));
        step_calculate_part_2(&mut answer, -1, &mut vistied);
        assert_eq!(answer.frequency, 0);
        assert_eq!(vistied.found, true);
    }

    #[test]
    #[ignore]
    fn calculate_part1() {
        let mut inputs = InputLines::from_slice(include_bytes!("../../inputs/2018/day1.txt"));
        let mut answer = Answer { frequency: 0 };
        while let Some(x) = inputs.next(false) {
            step_calculate_part_1(&mut answer, string_to_i32(x));
        }
        println!("{}", answer.frequency);
    }

    #[test]
    #[ignore]
    fn calculate_part2() {
        let mut inputs = InputLines::from_slice(include_bytes!("../../inputs/2018/day1.txt"));
        let mut answer = Answer { frequency: 0 };
        let mut vistied = VisitedNodes::new();
        while !vistied.found {
            let x = inputs.next(true).unwrap();
            step_calculate_part_2(&mut answer, string_to_i32(x), &mut vistied);
        }
        println!("{}", answer.frequency);
        println!("{}", inputs.read_pos);
    }
}

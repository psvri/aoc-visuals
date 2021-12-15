use std::collections::HashMap;

use bevy::prelude::*;

use crate::commons::aoc_common::{AOCName, AOCState, AocFont, InputLines};
use crate::commons::constants::WINDOW_WIDTH;

#[derive(Default)]
pub struct Counts {
    twos: u32,
    threes: u32,
}

#[derive(Default)]
pub struct BoxIds {
    pub ids: Vec<String>,
    pub common_id: String,
}

fn generate_mapping(word: &str) -> HashMap<char, u32> {
    let mut counts = HashMap::<char, u32>::new();

    word.chars().for_each(|letter| {
        let count = counts.entry(letter).or_insert(0);
        *count += 1;
    });

    counts
}

fn step_calculate_part_1(answer: &mut Counts, id: &str) {
    let mapping = generate_mapping(id);
    if mapping.values().any(|x| *x == 3) {
        answer.threes += 1;
    }

    if mapping.values().any(|x| *x == 2) {
        answer.twos += 1;
    }
}

fn check_diff(str1: &str, str2: &str) -> (bool, String) {
    let mut found_1_diff = false;
    let mut common_letters = String::new();

    for i in str1.chars().zip(str2.chars()) {
        if i.0 != i.1 {
            if found_1_diff {
                return (false, "".to_string());
            } else {
                found_1_diff = true;
            }
        } else {
            common_letters.push(i.1);
        }
    }
    (found_1_diff, common_letters)
}

fn step_calculate_part_2(answer: &mut BoxIds, id: String) {
    if answer.common_id.is_empty() {
        for box_id in &answer.ids {
            let (is_prototype, common_str) = check_diff(box_id, &id);
            if is_prototype {
                answer.common_id = common_str;
            }
        }
    }
    answer.ids.push(id);
}

pub fn setup(mut app: AppBuilder) -> AppBuilder {
    app.add_system_set(
        SystemSet::on_enter(AOCState {
            year: 2018,
            day: 2,
            part: 1,
        })
        .with_system(app_setup_part1.system()),
    );
    app.add_system_set(
        SystemSet::on_update(AOCState {
            year: 2018,
            day: 2,
            part: 1,
        })
        .with_system(step_part1.system())
        .with_system(update_sprite_part1.system())
        .with_system(update_text_part1.system()),
    );
    app.add_system_set(
        SystemSet::on_enter(AOCState {
            year: 2018,
            day: 2,
            part: 2,
        })
        .with_system(app_setup_part2.system()),
    );
    app.add_system_set(
        SystemSet::on_update(AOCState {
            year: 2018,
            day: 2,
            part: 2,
        })
        .with_system(step_part2.system()),
    );
    app
}

fn app_setup_part1(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    aoc_font: Res<AocFont>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(-WINDOW_WIDTH / 4.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(100.0, 0.0)),
            ..Default::default()
        })
        .insert(AOCName("twos".to_owned()));
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_xyz(WINDOW_WIDTH / 4.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(100.0, 0.0)),
            ..Default::default()
        })
        .insert(AOCName("threes".to_owned()));
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Duals: ".to_string(),
                        style: TextStyle {
                            font: aoc_font.font_handle.clone(),
                            font_size: 40.0,
                            color: Color::RED,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: aoc_font.font_handle.clone(),
                            font_size: 40.0,
                            color: Color::RED,
                        },
                    },
                    TextSection {
                        value: "\nTriplets: ".to_string(),
                        style: TextStyle {
                            font: aoc_font.font_handle.clone(),
                            font_size: 40.0,
                            color: Color::GREEN,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: aoc_font.font_handle.clone(),
                            font_size: 40.0,
                            color: Color::GREEN,
                        },
                    },
                    TextSection {
                        value: "\nChecksum: ".to_string(),
                        style: TextStyle {
                            font: aoc_font.font_handle.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: aoc_font.font_handle.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            ..Default::default()
        })
        .insert(AOCName("Output".to_string()));
    commands.insert_resource(Counts::default());
    commands
        .spawn()
        .insert(InputLines::from_slice(include_bytes!(
            "../../inputs/2018/day2.txt"
        )));
}

fn step_part1(mut conuts: ResMut<Counts>, mut query: Query<&mut InputLines>) {
    let mut line = query.single_mut().unwrap();
    if let Some(x) = line.next(false) {
        step_calculate_part_1(&mut conuts, x)
    }
}

fn update_text_part1(conuts: Res<Counts>, mut query: Query<&mut Text, With<AOCName>>) {
    let mut text = query.single_mut().unwrap();
    text.sections[1].value = conuts.twos.to_string();
    text.sections[3].value = conuts.threes.to_string();
    text.sections[5].value = (conuts.twos * conuts.threes).to_string();
}

fn update_sprite_part1(conuts: Res<Counts>, mut query: Query<(&mut Sprite, &AOCName)>) {
    for (mut sprite, name) in query.iter_mut() {
        if name.0 == "twos" {
            sprite.size.y = conuts.twos as f32;
        }
        if name.0 == "threes" {
            sprite.size.y = conuts.threes as f32;
        }
    }
}

fn app_setup_part2(mut commands: Commands, aoc_font: Res<AocFont>) {
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Answer is: ".to_string(),
                        style: TextStyle {
                            font: aoc_font.font_handle.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: aoc_font.font_handle.clone(),
                            font_size: 40.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            ..Default::default()
        })
        .insert(AOCName("Output".to_string()))
        .insert(InputLines::from_slice(include_bytes!(
            "../../inputs/2018/day2.txt"
        )));
    commands.insert_resource(BoxIds::default());
}

fn step_part2(mut counts: ResMut<BoxIds>, mut query: Query<(&mut InputLines, &mut Text)>) {
    let (mut line, mut text) = query.single_mut().unwrap();
    if let Some(x) = line.next(false) {
        step_calculate_part_2(&mut counts, x.to_string());
        if counts.common_id.is_empty() {
            text.sections[1].value = x.to_string();
        } else {
            text.sections[1].value = counts.common_id.to_string();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_mapping() {
        let mapping = generate_mapping("acbbcbaccd");
        assert_eq!(*mapping.get(&'a').unwrap(), 2);
        assert_eq!(*mapping.get(&'b').unwrap(), 3);
        assert_eq!(*mapping.get(&'c').unwrap(), 4);
        assert_eq!(*mapping.get(&'d').unwrap(), 1);
    }

    #[test]
    fn test_step_calculate_part_1() {
        let mut count = Counts {
            ..Default::default()
        };
        step_calculate_part_1(&mut count, "aabcdd");
        assert_eq!(count.twos, 1);
        assert_eq!(count.threes, 0);
        step_calculate_part_1(&mut count, "ababab");
        assert_eq!(count.twos, 1);
        assert_eq!(count.threes, 1);
        step_calculate_part_1(&mut count, "bababc");
        assert_eq!(count.twos, 2);
        assert_eq!(count.threes, 2);
        step_calculate_part_1(&mut count, "abcdef");
        assert_eq!(count.twos, 2);
        assert_eq!(count.threes, 2);
    }

    #[test]
    fn test_check_diff() {
        assert_eq!(check_diff("fghij", "fguij"), (true, "fgij".to_string()));
        assert!(!check_diff("abcde", "axcye").0);
        assert!(!check_diff("klmno", "axcye").0);
    }

    #[test]
    #[ignore]
    fn calculate_part1() {
        let mut inputs = InputLines::from_slice(include_bytes!("../../inputs/2018/day2.txt"));
        let mut count = Counts::default();
        while let Some(x) = inputs.next(false) {
            step_calculate_part_1(&mut count, x);
        }
        println!("Result is {}", count.twos * count.threes);
    }

    #[test]
    #[ignore]
    fn calculate_part2() {
        let mut inputs = InputLines::from_slice(include_bytes!("../../inputs/2018/day2.txt"));
        let mut box_ids = BoxIds::default();
        println!("{:?}", box_ids.ids);
        println!("{}", box_ids.common_id);
        while let Some(x) = inputs.next(false) {
            step_calculate_part_2(&mut box_ids, x.to_string());
        }
        println!("Result is {}", box_ids.common_id);
    }
}

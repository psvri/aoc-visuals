use bevy::prelude::*;

use crate::commons::aoc_common::{AOCState, AocFont, InputLines};

#[derive(Debug, Component)]
struct Fabric {
    pub grid: Vec<Vec<u16>>,
}

impl Fabric {
    fn new(size: u32) -> Self {
        let mut grid = Vec::<Vec<u16>>::with_capacity(size as usize);
        for _ in 0..size {
            grid.push((0..size).map(|_| 0).collect::<Vec<u16>>());
        }

        Self { grid }
    }

    fn create_string(&self) -> String {
        let mut string = String::new();
        for line in &self.grid {
            for number in line {
                if *number > 0 {
                    string.push_str("  .  ");
                } else {
                    string.push_str("     ");
                }
            }
            string.push_str("\n");
        }
        string
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Claim {
    pub id: u16,
    pub pos: [u16; 2],
    pub size: [u16; 2],
}

impl Claim {
    pub fn parse_command(claim_str: &str) -> Claim {
        let mut claim_str_iter = claim_str.split(" ");
        let id_str = claim_str_iter.next().unwrap();
        claim_str_iter.next();
        let pos_str = claim_str_iter.next().unwrap();
        let size_str = claim_str_iter.next().unwrap();

        let id = id_str[1..].parse::<u16>().unwrap();
        let pos = pos_str[..pos_str.len() - 1]
            .split(',')
            .map(|x| x.parse::<u16>().unwrap())
            .collect::<Vec<u16>>();
        let size = size_str
            .split('x')
            .map(|x| x.parse::<u16>().unwrap())
            .collect::<Vec<u16>>();

        Claim {
            id,
            pos: pos.try_into().unwrap(),
            size: size.try_into().unwrap(),
        }
    }
}

fn update_fabric_wth_claim(fabric: &mut Fabric, claim: Claim) {
    for x in 0..claim.size[0] as usize {
        for y in 0..claim.size[1] as usize {
            fabric.grid[claim.pos[1] as usize + y][claim.pos[0] as usize + x] += 1;
        }
    }
}

fn get_overlaps(fabric: &Fabric) -> u64 {
    fabric
        .grid
        .iter()
        .map(|x| x.iter().map(|y| (*y >= 2) as u64).sum::<u64>())
        .sum::<u64>()
}

fn check_non_overlapping_claim(fabric: &Fabric, claim: &Claim) -> bool {
    for x in 0..claim.size[0] as usize {
        for y in 0..claim.size[1] as usize {
            if fabric.grid[claim.pos[1] as usize + y][claim.pos[0] as usize + x] >= 2 {
                return false;
            }
        }
    }
    true
}

pub fn setup(mut app: App) -> App {
    app.add_system_set(
        SystemSet::on_enter(AOCState {
            year: 2018,
            day: 3,
            part: 1,
        })
        .with_system(app_setup_part1),
    );
    app.add_system_set(
        SystemSet::on_update(AOCState {
            year: 2018,
            day: 3,
            part: 1,
        })
        .with_system(step_part1),
    );
    app
}

fn app_setup_part1(mut commands: Commands, aoc_font: Res<AocFont>) {
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![create_text_section(100, &aoc_font)],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            transform: Transform::from_xyz(-150.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Fabric::new(1000))
        .insert(InputLines::from_slice(include_bytes!(
            "../../inputs/2018/day3.txt"
        )));
}

fn step_part1(mut query: Query<(&mut InputLines, &mut Fabric, &mut Text)>) {
    let (mut line, mut fabric, mut text) = query.single_mut();
    if let Some(claim_str) = line.next(false) {
        update_fabric_wth_claim(&mut fabric, Claim::parse_command(claim_str));
        //text.sections[0].value = fabric.create_string();
        println!("{}", get_overlaps(&fabric));
    }
}

fn format_text(number: u16) -> String {
    format!("{:0>4} ", number)
}

fn create_text_section(size: u32, aoc_font: &Res<AocFont>) -> TextSection {
    let mut final_text = String::new();
    for j in 0..size {
        for i in 0..size {
            let mut text = "  .  ".to_string();
            if i == (size - 1) {
                text.push('\n');
            }
            final_text.push_str(&text)
        }
    }
    TextSection {
        value: final_text,
        style: TextStyle {
            font: aoc_font.font_handle.clone(),
            font_size: 7.0,
            color: Color::WHITE,
        },
    }
}

#[cfg(test)]
mod test {
    use crate::commons::aoc_common::InputLines;

    use super::*;

    #[test]
    fn test_parse_command() {
        assert_eq!(
            Claim::parse_command("#1 @ 1,3: 4x4"),
            Claim {
                id: 1,
                pos: [1, 3],
                size: [4, 4]
            }
        );
        assert_eq!(
            Claim::parse_command("#100 @ 104,310: 40x400"),
            Claim {
                id: 100,
                pos: [104, 310],
                size: [40, 400]
            }
        );
    }

    #[test]
    fn test_update_fabric() {
        let mut fabric = Fabric::new(8);
        println!("{:?}", fabric);
        update_fabric_wth_claim(&mut fabric, Claim::parse_command("#1 @ 1,3: 4x4"));
        update_fabric_wth_claim(&mut fabric, Claim::parse_command("#2 @ 3,1: 4x4"));
        update_fabric_wth_claim(&mut fabric, Claim::parse_command("#3 @ 5,5: 2x2"));

        assert_eq!(fabric.grid[0], [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(fabric.grid[1], [0, 0, 0, 1, 1, 1, 1, 0]);
        assert_eq!(fabric.grid[2], [0, 0, 0, 1, 1, 1, 1, 0]);
        assert_eq!(fabric.grid[3], [0, 1, 1, 2, 2, 1, 1, 0]);
        assert_eq!(fabric.grid[4], [0, 1, 1, 2, 2, 1, 1, 0]);
        assert_eq!(fabric.grid[5], [0, 1, 1, 1, 1, 1, 1, 0]);
        assert_eq!(fabric.grid[6], [0, 1, 1, 1, 1, 1, 1, 0]);
        assert_eq!(fabric.grid[7], [0, 0, 0, 0, 0, 0, 0, 0]);

        assert_eq!(get_overlaps(&fabric), 4);

        assert!(!check_non_overlapping_claim(
            &fabric,
            &Claim::parse_command("#1 @ 1,3: 4x4")
        ));
        assert!(!check_non_overlapping_claim(
            &fabric,
            &Claim::parse_command("#2 @ 3,1: 4x4")
        ));
        assert!(check_non_overlapping_claim(
            &fabric,
            &Claim::parse_command("#3 @ 5,5: 2x2")
        ));
    }

    #[test]
    #[ignore]
    fn calculate_part1() {
        let mut inputs = InputLines::from_slice(include_bytes!("../../inputs/2018/day3.txt"));
        let mut fabric = Fabric::new(1000);
        while let Some(claim) = inputs.next(false) {
            update_fabric_wth_claim(&mut fabric, Claim::parse_command(claim));
        }

        println!("Answer for day3 part1 is {}", get_overlaps(&fabric));
    }

    #[test]
    #[ignore]
    fn calculate_part2() {
        let mut inputs = InputLines::from_slice(include_bytes!("../../inputs/2018/day3.txt"));
        let mut fabric = Fabric::new(1000);
        while let Some(claim) = inputs.next(false) {
            update_fabric_wth_claim(&mut fabric, Claim::parse_command(claim));
        }

        inputs.refresh();

        loop {
            match inputs.next(false) {
                Some(claim_str) => {
                    let claim = Claim::parse_command(claim_str);
                    if check_non_overlapping_claim(&fabric, &claim) {
                        println!("Answer for day3 part2 is {}", claim.id);
                        break;
                    }
                }
                None => {
                    println!("Answer for day3 part2 not found");
                    break;
                }
            }
        }
    }
}

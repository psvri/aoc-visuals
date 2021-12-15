mod aoc_2018;
mod commons;
use bevy::prelude::*;
use commons::aoc_common::{AOCName, AOCState, AocFont};
use commons::constants::{DAYS, PARTS, WINDOW_HEIGHT, YEARS};
use commons::fps::FpsPlugin;
use commons::window_setup::WindowSetup;

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

enum MenuButtonType {
    Year,
    Day,
    Part,
    Launch,
}

#[derive(Default)]
struct MenuData {
    buttons: Vec<Entity>,
    problem: (u16, u8, u8),
}

impl MenuData {
    pub fn new(
        commands: &mut Commands,
        button_materials: &Res<ButtonMaterials>,
        aoc_font: &Res<AocFont>,
    ) -> Self {
        let mut menu = Self::default();
        let year_header = commands
            .spawn_bundle(Self::create_text_bundle("Year", (150.0, 0.0), aoc_font))
            .id();
        let day_header = commands
            .spawn_bundle(Self::create_text_bundle("Day", (500.0, 0.0), aoc_font))
            .id();
        let part_header = commands
            .spawn_bundle(Self::create_text_bundle("Part", (950.0, 0.0), aoc_font))
            .id();
        let launch_button = Self::create_launch_button(
            (1000.0, WINDOW_HEIGHT / 2.0),
            commands,
            aoc_font,
            button_materials,
            MenuButtonType::Launch,
        );
        menu.buttons.push(year_header);
        menu.buttons.push(day_header);
        menu.buttons.push(part_header);
        menu.buttons.push(launch_button);
        menu
    }

    fn create_text_bundle(
        text_value: &str,
        pos: (f32, f32),
        aoc_font: &Res<AocFont>,
    ) -> TextBundle {
        TextBundle {
            text: Text::with_section(
                text_value,
                TextStyle {
                    font: aoc_font.font_handle.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(50.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(pos.0),
                    bottom: Val::Px(pos.1),
                    ..Default::default()
                },
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn insert_button(
        &mut self,
        button_text: &str,
        button_type: MenuButtonType,
        commands: &mut Commands,
        aoc_font: &Res<AocFont>,
        button_materials: &Res<ButtonMaterials>,
        pos: (f32, f32),
    ) {
        let button_entity = Self::create_button_entity(
            button_text,
            pos,
            commands,
            aoc_font,
            button_materials,
            button_type,
        );
        self.buttons.push(button_entity);
    }

    fn create_button_entity(
        button_text: &str,
        pos: (f32, f32),
        commands: &mut Commands,
        aoc_font: &Res<AocFont>,
        button_materials: &Res<ButtonMaterials>,
        button_type: MenuButtonType,
    ) -> Entity {
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(pos.0),
                        bottom: Val::Px(pos.1),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        button_text,
                        TextStyle {
                            font: aoc_font.font_handle.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            })
            .insert(button_type)
            .id()
    }

    fn create_launch_button(
        pos: (f32, f32),
        commands: &mut Commands,
        aoc_font: &Res<AocFont>,
        button_materials: &Res<ButtonMaterials>,
        button_type: MenuButtonType,
    ) -> Entity {
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(250.0), Val::Px(250.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(pos.0),
                        bottom: Val::Px(pos.1),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection {
                                    value: "Launch".to_string(),
                                    style: TextStyle {
                                        font: aoc_font.font_handle.clone(),
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                },
                                TextSection {
                                    value: "\nYear: ".to_string(),
                                    style: TextStyle {
                                        font: aoc_font.font_handle.clone(),
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
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
                                TextSection {
                                    value: "\nDay: ".to_string(),
                                    style: TextStyle {
                                        font: aoc_font.font_handle.clone(),
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
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
                                TextSection {
                                    value: "\nPart: ".to_string(),
                                    style: TextStyle {
                                        font: aoc_font.font_handle.clone(),
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
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
                    .insert(AOCName("Launch".to_string()));
            })
            .insert(button_type)
            .id()
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: ResMut<MenuData>) {
    cleanup_vec_entity(&mut commands, &menu_data.buttons);
}

fn cleanup_vec_entity(commands: &mut Commands, buttons: &[Entity]) {
    for button in buttons {
        commands.entity(*button).despawn_recursive();
    }
}

fn setup_menu(
    mut commands: Commands,
    aoc_font: Res<AocFont>,
    button_materials: Res<ButtonMaterials>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    let mut menu_data = MenuData::new(&mut commands, &button_materials, &aoc_font);
    for (index, year) in YEARS.iter().enumerate() {
        menu_data.insert_button(
            &year.to_string(),
            MenuButtonType::Year,
            &mut commands,
            &aoc_font,
            &button_materials,
            (100.0, 50.0 * (index + 1) as f32),
        );
    }
    for (index, day) in DAYS.iter().enumerate() {
        menu_data.insert_button(
            &day.to_string(),
            MenuButtonType::Day,
            &mut commands,
            &aoc_font,
            &button_materials,
            (
                300.0 + 150.0 * (index as f32 / 12.0).floor(),
                50.0 * ((index as f32 % 12.0) + 1.0) as f32,
            ),
        );
    }
    for (index, day) in PARTS.iter().enumerate() {
        menu_data.insert_button(
            &day.to_string(),
            MenuButtonType::Part,
            &mut commands,
            &aoc_font,
            &button_materials,
            (900.0, 50.0 * (index + 1) as f32),
        );
    }

    commands.insert_resource(menu_data);
}

fn menu(
    mut state: ResMut<State<AOCState>>,
    mut menu_selection: ResMut<MenuData>,
    button_materials: Res<ButtonMaterials>,
    mut query_parent: Query<
        (
            &Interaction,
            &mut Handle<ColorMaterial>,
            &MenuButtonType,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    query_child: Query<&Text>,
) {
    for (interaction, mut material, name, children) in query_parent.iter_mut() {
        let mut value: &str = "0";
        for &child in children.iter() {
            value = &query_child.get(child).unwrap().sections[0].value;
        }

        match (*interaction, &name) {
            (Interaction::Clicked, MenuButtonType::Launch) => {
                *material = button_materials.pressed.clone();
                println!("launching state: {:?}", menu_selection.problem);
                state
                    .set(AOCState {
                        year: menu_selection.problem.0,
                        day: menu_selection.problem.1,
                        part: menu_selection.problem.2,
                    })
                    .unwrap();
            }
            (Interaction::Clicked, MenuButtonType::Year) => {
                *material = button_materials.pressed.clone();
                menu_selection.problem.0 = value.parse::<u16>().unwrap();
            }
            (Interaction::Clicked, MenuButtonType::Day) => {
                *material = button_materials.pressed.clone();
                menu_selection.problem.1 = value.parse::<u8>().unwrap();
            }
            (Interaction::Clicked, MenuButtonType::Part) => {
                *material = button_materials.pressed.clone();
                menu_selection.problem.2 = value.parse::<u8>().unwrap();
            }
            (Interaction::Hovered, _) => {
                *material = button_materials.hovered.clone();
            }
            (Interaction::None, _) => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn update_launch_botton(
    menu_selection: ResMut<MenuData>,
    mut query: Query<&mut Text, With<AOCName>>,
) {
    let mut text = query.single_mut().unwrap();
    let (year, day, month) = menu_selection.problem;
    text.sections[2].value = year.to_string();
    text.sections[4].value = day.to_string();
    text.sections[6].value = month.to_string();
}

fn setup_app() -> AppBuilder {
    let mut app = App::build();
    app.add_startup_system(AocFont::setup_font_resource.system().label("font_init"));
    app.add_plugin(FpsPlugin);
    app.add_plugin(WindowSetup);
    app.add_plugins(DefaultPlugins);
    app.init_resource::<ButtonMaterials>();
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_state(AOCState {
        year: 0,
        day: 0,
        part: 1,
    });
    app.add_system_set(
        SystemSet::on_enter(AOCState {
            year: 0,
            day: 0,
            part: 1,
        })
        .with_system(setup_menu.system()),
    );
    app.add_system_set(
        SystemSet::on_update(AOCState {
            year: 0,
            day: 0,
            part: 1,
        })
        .with_system(menu.system())
        .with_system(update_launch_botton.system()),
    );
    app.add_system_set(
        SystemSet::on_exit(AOCState {
            year: 0,
            day: 0,
            part: 1,
        })
        .with_system(cleanup_menu.system()),
    );
    app = aoc_2018::day1::setup(app);
    app = aoc_2018::day2::setup(app);
    app
}

fn main() {
    let mut app = setup_app();
    app.run();
}

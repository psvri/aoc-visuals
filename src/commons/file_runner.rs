use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

struct FileInput(String);

fn my_runner(mut app: App, file_name: &str) {
    println!("Type stuff into the console");
    let buffered_reader = BufReader::new(File::open(file_name).unwrap());
    for line in buffered_reader.lines() {
        {
            let mut input = app.world.get_resource_mut::<FileInput>().unwrap();
            input.0 = line.unwrap();
        }
        app.update();
    }
}

fn print_system(input: Res<FileInput>) {
    println!("You typed: {}", input.0);
}

pub fn run() {
    App::build()
        .insert_resource(FileInput(String::new()))
        .set_runner(|app| my_runner(app, "inputs/2018/day1.txt"))
        .add_system(print_system.system())
        .run();
}

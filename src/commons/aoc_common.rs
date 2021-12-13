use std::io::BufRead;
use std::io::BufReader;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct AOCState {
    pub year: u16,
    pub day: u8,
    pub part: u8,
}

pub struct BorderSize {
    pub max_x: f32,
    pub max_y: f32,
    pub current_x: f32,
    pub current_y: f32,
}

pub struct ScalableObject;

pub struct AOCName(pub String);

pub struct InputLines {
    pub input_lines: Vec<String>,
    pub read_pos: usize,
}

impl InputLines {
    pub fn from_slice(input_slice: &[u8]) -> Self {
        let input_lines = BufReader::new(input_slice)
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>();
        let read_pos = 0;
        Self {
            input_lines,
            read_pos,
        }
    }

    pub fn next(&mut self, repeat: bool) -> Option<&String> {
        let mut pos = self.read_pos;
        if repeat {
            pos = self.read_pos % self.input_lines.len();
        }
        if pos < self.input_lines.len() {
            let value = &self.input_lines[pos];
            self.read_pos += 1;
            Some(value)
        } else {
            None
        }
    }
}

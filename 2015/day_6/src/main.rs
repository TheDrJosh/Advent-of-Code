use chumsky::{error::Simple, Parser};

struct Point {
    x: u32,
    y: u32,
}

struct Region {
    from: Point,
    to: Point,
}

enum Instruction {
    Toggle(Region),
    Set(bool, Region),
}

fn parser() -> impl Parser<char, Instruction, Error = Simple<char>> {
    todo!()
}

fn main() {
    let src = std::fs::read_to_string("./input.txt").unwrap();
}

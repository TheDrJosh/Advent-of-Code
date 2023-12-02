fn floor(dir: &str) -> i32 {
    dir.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .sum()
}

fn basement(dir: &str) -> u32 {
    let iter = dir.chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => unreachable!(),
    });

    let mut steps = 1;
    let mut floor = 0;

    for (i, step) in iter.enumerate() {
        floor += step;

        if floor == -1 {
            break;
        }

        steps += 1;
    }

    steps
}

fn main() {
    let buf = std::fs::read_to_string("./input.txt").unwrap();
    let res = basement(&buf);
    println!("Result: {res}");
}

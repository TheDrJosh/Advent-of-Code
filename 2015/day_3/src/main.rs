use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn make_directions(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|c| match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
}

fn visited_houses(dirs: Vec<Direction>) -> HashSet<(i32, i32)> {
    let mut houses_visited = HashSet::<(i32, i32)>::new();
    let mut loc = (0, 0);

    for dir in dirs {
        houses_visited.insert(loc);
        match dir {
            Direction::North => loc.1 += 1,
            Direction::East => loc.0 += 1,
            Direction::South => loc.1 -= 1,
            Direction::West => loc.0 -= 1,
        }
    }

    houses_visited
}

fn map_houses(dirs: Vec<Direction>) -> usize {
    let houses_visited = visited_houses(dirs);

    houses_visited.len()
}

fn robo_santa_helper(dirs: Vec<Direction>) -> usize {
    let dirs1 = dirs
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, dir)| if i % 2 == 0 { Some(dir) } else { None })
        .collect::<Vec<_>>();
    let dirs2 = dirs
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, dir)| if i % 2 != 0 { Some(dir) } else { None })
        .collect::<Vec<_>>();

    let mut house_visited_1 = visited_houses(dirs1);
    let house_visited_2 = visited_houses(dirs2);

    house_visited_1.extend(house_visited_2);

    house_visited_1.len()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let dirs = make_directions(&input);

    let houses_visited = robo_santa_helper(dirs);

    println!("Results: {houses_visited}");
}

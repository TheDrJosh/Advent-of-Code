fn calc_wraping_paper(l: u32, w: u32, h: u32) -> u32 {
    let s1 = l * w;
    let s2 = w * h;
    let s3 = h * l;

    let extra = s1.min(s2).min(s3);

    2 * s1 + 2 * s2 + 2 * s3 + extra
}

fn calc_ribbon(l: u32, w: u32, h: u32) -> u32 {
    let max_index = [l, w, h]
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)
        .unwrap();
    
    let mut sides = vec![l, w, h];
    sides.remove(max_index);

    let parim = 2 * sides[0] + 2 * sides[1];
    let bow = l * w * h;

    parim + bow
}

fn parse_dim(dim: &str) -> (u32, u32, u32) {
    let mut splits = dim.trim().split('x');
    (
        splits.next().unwrap().parse().unwrap(),
        splits.next().unwrap().parse().unwrap(),
        splits.next().unwrap().parse().unwrap(),
    )
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let res: u32 = input
        .trim()
        .split('\n')
        .map(|l| parse_dim(l))
        .map(|(l, w, h)| calc_ribbon(l, w, h))
        .sum();
    println!("Result: {res}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wraping_paper() {
        assert_eq!(calc_wraping_paper(2, 3, 4), 58);
        assert_eq!(calc_wraping_paper(1, 1, 10), 43);
    }
    #[test]
    fn ribbon() {
        assert_eq!(calc_ribbon(2, 3, 4), 34);
        assert_eq!(calc_ribbon(1, 1, 10), 14);
    }
}

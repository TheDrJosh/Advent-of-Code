use std::io::Write;

fn get_value_5_zeros(input: &str) -> u32 {
    for i in 0.. {
        let digest = md5::compute(format!("{input}{i}"));

        // println!("{i}: {digest:?} | {:?}", digest.0);

        if digest.0[0] == 0 && digest.0[1] == 0 && digest.0[2] < 16 {
            return i;
        }
    }
    unreachable!()
}

fn get_value_6_zeros(input: &str) -> u32 {
    for i in 0.. {
        let digest = md5::compute(format!("{input}{i}"));

        // println!("{i}: {digest:?} | {:?}", digest.0);

        if digest.0[0] == 0 && digest.0[1] == 0 && digest.0[2] == 0 {
            return i;
        }
    }
    unreachable!()
}

fn main() {
    print!("Input: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let res = get_value_6_zeros(input.trim());
    println!("Result: {res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(get_value_5_zeros("abcdef"), 609043);
        assert_eq!(get_value_5_zeros("pqrstuv"), 1048970);
    }
}

#[derive(Debug, PartialEq)]
enum StringType {
    Naughty,
    Nice,
}

fn naughty_or_nice(input: &str) -> StringType {
    if input.chars().filter(|c| "aeiou".contains(*c)).count() < 3 {
        return StringType::Naughty;
    }

    if input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy")
    {
        return StringType::Naughty;
    }

    let mut has_double = false;

    for i in 0..(input.chars().count() - 1) {
        if input.chars().nth(i) == input.chars().nth(i + 1) {
            has_double = true;
        }
    }

    if !has_double {
        return StringType::Naughty;
    }

    return StringType::Nice;
}

fn naughty_or_nice2(input: &str) -> StringType {
    println!("{input}");

    let mut has_double = false;

    for i in 0..(input.chars().count() - 1) {
        let sub_str = &input[i..(i + 2)];

        let finds = input
            .match_indices(sub_str)
            .filter(|(index, _)| *index != i)
            .count();

        if finds != 0 {
            has_double = true;
        }
    }

    println!("has double: {has_double}");

    if !has_double {
        return StringType::Naughty;
    }

    let mut has_skipped_double = false;

    for i in 0..(input.chars().count() - 2) {
        if input.chars().nth(i) == input.chars().nth(i + 2) {
            has_skipped_double = true;
        }
    }

    println!("has skipped input: {has_skipped_double}");

    if !has_skipped_double {
        return StringType::Naughty;
    }

    return StringType::Nice;
}

fn main() {
    let inputs = std::fs::read_to_string("./input.txt").unwrap();
    let mut nice_count = 0;
    for input in inputs.trim().lines() {
        if naughty_or_nice2(input) == StringType::Nice {
            nice_count += 1;
        }
    }
    println!("Result: {nice_count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naughty_or_nice_tests() {
        assert_eq!(naughty_or_nice("ugknbfddgicrmopn"), StringType::Nice);
        assert_eq!(naughty_or_nice("aaa"), StringType::Nice);
        assert_eq!(naughty_or_nice("jchzalrnumimnmhp"), StringType::Naughty);
        assert_eq!(naughty_or_nice("haegwjzuvuyypxyu"), StringType::Naughty);
        assert_eq!(naughty_or_nice("dvszwmarrgswjxmb"), StringType::Naughty);
    }
    #[test]
    fn naughty_or_nice2_tests() {
        assert_eq!(naughty_or_nice2("qjhvhtzxzqqjkmpb"), StringType::Nice);
        assert_eq!(naughty_or_nice2("xxyxx"), StringType::Nice);
        assert_eq!(naughty_or_nice2("uurcxstgmygtbstg"), StringType::Naughty);
        assert_eq!(naughty_or_nice2("ieodomkazucvgmuy"), StringType::Naughty);

        unimplemented!()
    }
}

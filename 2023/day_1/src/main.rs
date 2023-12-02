fn parse_calibration_value(value: &str) -> u32 {
    let first_num = value.chars().find(|c| c.is_numeric()).unwrap();
    let last_num = value.chars().rev().find(|c| c.is_numeric()).unwrap();
    format!("{first_num}{last_num}").parse().unwrap()
}

fn parse_calibration_document(doc: &str) -> u32 {
    doc.lines()
        .filter(|s| !s.is_empty())
        .map(parse_calibration_value)
        .sum()
}

fn check_digit(val: &str, index: usize) -> Option<char> {
    if val.chars().nth(index).unwrap().is_numeric() {
        return Some(val.chars().nth(index).unwrap());
    }

    if val[index..].starts_with("one") {
        return Some('1');
    }
    if val[index..].starts_with("two") {
        return Some('2');
    }
    if val[index..].starts_with("three") {
        return Some('3');
    }
    if val[index..].starts_with("four") {
        return Some('4');
    }
    if val[index..].starts_with("five") {
        return Some('5');
    }
    if val[index..].starts_with("six") {
        return Some('6');
    }
    if val[index..].starts_with("seven") {
        return Some('7');
    }
    if val[index..].starts_with("eight") {
        return Some('8');
    }
    if val[index..].starts_with("nine") {
        return Some('9');
    }

    None
}

fn parse_calibration_value_fixed(value: &str) -> u32 {
    let mut first_num = None;
    for i in value.char_indices().map(|c| c.0) {
        if let Some(num) = check_digit(value, i) {
            first_num = Some(num);
            break;
        }
    }

    let mut last_num = None;
    for i in value.char_indices().rev().map(|c| c.0) {
        if let Some(num) = check_digit(value, i) {
            last_num = Some(num);
            break;
        }
    }

    format!("{}{}", first_num.unwrap(), last_num.unwrap())
        .parse()
        .unwrap()
}

fn parse_calibration_document_fixed(doc: &str) -> u32 {
    doc.lines()
        .filter(|s| !s.is_empty())
        .map(parse_calibration_value_fixed)
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let result = parse_calibration_document(&input);

    println!("Result 1: {result}");

    let result = parse_calibration_document_fixed(&input);

    println!("Result 2: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let example = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        let sum = parse_calibration_document(example);

        assert_eq!(sum, 142);
    }

    #[test]
    fn test_2() {
        let example = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

        let sum = parse_calibration_document_fixed(example);

        assert_eq!(sum, 281);
    }
}

pub fn is_valid(string: &str) -> bool {
    if string.chars().any(|ch| !(ch.is_digit(10) || ch.is_whitespace())) { return false };

    let digits: Vec<u32> = string.chars().filter_map(|ch| ch.to_digit(10)).collect();

    if digits.len() < 2 { return false };

    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &num)| if (i + 1) % 2 == 0 { num * 2} else { num })
        .map(|num| if num > 9 { num / 10 + num - 10 } else { num })
        .sum();

    sum % 10 == 0
}

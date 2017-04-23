pub fn is_valid(string: &str) -> bool {
    let mut sum: u32 = 0;
    let mut count: u32 = 0;

    for (i, ch) in string.chars().rev().filter(|&ch| ch != ' ').enumerate() {
        let num = match ch.to_digit(10) {
            Some(n) => n,
            None => return false,
        };
        count += 1;

        if (i + 1) % 2 == 0 {
            let prod = num * 2;
            if prod > 9 {
                sum += prod / 10 + prod - 10;
            } else {
                sum += prod
            }
        } else {
            sum += num
        }
    }

    if count < 2 {
        return false
    }

    sum % 10 == 0
}
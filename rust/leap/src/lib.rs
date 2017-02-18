fn is_divisible_by(num: usize, div: usize) -> bool {
    num % div == 0
}


pub fn is_leap_year(year: usize) -> bool {
    if is_divisible_by(year, 100) {
        is_divisible_by(year, 400)
    } else {
        is_divisible_by(year, 4)
    }
}

fn is_div_any<'a>(num: &'a usize, numbers: &'a Vec<usize>) -> bool {
    numbers.iter().any(|&x| num % x == 0)
}

pub fn sum_of_multiples<'a>(n: usize, numbers: &'a Vec<usize>) -> usize {
    (1..n).filter(|&x| is_div_any(&x, &numbers)).collect::<Vec<_>>().iter().sum()
    // variant with fold
    // (1..n).fold(0, |sum, x| if is_div_any(&x, &numbers) { sum + x } else { sum })
}

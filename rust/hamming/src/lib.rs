pub fn hamming_distance<'a>(left: &'a str, right: &'a str) -> Result<usize, &'a str> {
    if left.len() == right.len() {
        Ok(left.chars().zip(right.chars())
            .filter(|&(l, r)| l != r)
            .count())
    } else {
        Err(&"Strings must have equal lengths")
    }
}

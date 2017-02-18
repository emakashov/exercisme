fn if_factor(num: usize, factor: usize, word: &str) -> &str {
    if num % factor == 0 { word } else { "" }
}


pub fn raindrops(num: usize) -> String {
    let result = format!("{}{}{}",
                         if_factor(num, 3, "Pling"),
                         if_factor(num, 5, "Plang"),
                         if_factor(num, 7, "Plong"));

    if !result.is_empty() { result } else { num.to_string() }
}

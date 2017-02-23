fn is_ask<'a>(question: &'a str) -> bool {
    question.ends_with("?")
}


fn is_shout<'a>(question: &'a str) -> bool {
    let letters: Vec<char> = question.chars()
        .filter(|ch| ch.is_alphabetic())
        .collect();
    letters.len() > 0 && letters.iter().all(|ch| ch.is_uppercase())
}


fn is_contains_weird_characters<'a>(question: &'a str) -> bool {
    let chs = "%^*@#$(*^";
    question.chars().any(|ch| chs.contains(ch))
}


fn is_yell<'a>(question: &'a str) -> bool {
    is_shout(&question) || is_contains_weird_characters(&question)
}


fn is_anything<'a>(question: &'a str) -> bool {
    question.is_empty()
}


pub fn reply(question: &str) -> &str {
    if is_ask(&question) {
        "Sure."
    } else if is_yell(&question) {
        "Whoa, chill out!"
    } else if is_anything(&question) {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}

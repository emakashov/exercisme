use std::collections::BTreeSet;
use std::ascii::AsciiExt;

pub fn is_pangram(sentence: &str) -> bool {
    let set: BTreeSet<char> = sentence.to_lowercase()
        .chars()
        .filter(|&ch| ch.is_ascii() && ch.is_alphabetic())
        .collect();

    set.len() == 26
}

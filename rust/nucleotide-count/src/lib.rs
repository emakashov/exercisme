use std::collections::HashMap;

static VALID: &'static str =  "ACGT";

fn valid(x: char) -> bool {
    VALID.contains(x)
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !valid(nucleotide) { return Err(nucleotide) }

    let mut counter: usize = 0;

    for ch in dna.chars() {
        if !valid(ch) {
            return Err(ch)
        } else if ch == nucleotide {
            counter += 1
        }
    }
    Ok(counter)
}

type Counter = HashMap<char, usize>;

pub fn nucleotide_counts(dna: &str) -> Result<Counter, char>  {
    let mut map = Counter::new();

    for ch in VALID.chars() {
        map.insert(ch, 0);
    }

    for ch in dna.chars() {
        if !valid(ch) {
            return Err(ch)
        } else if let Some(counter) = map.get_mut(&ch) {
            *counter += 1;
        }
    }

    Ok(map)
}

fn capitalize(string: String) -> String {
    let mut chars = string.chars();
    match chars.next() {
        None => String::new(),
        Some(ch) => ch.to_uppercase().collect::<String>() + chars.as_str(),
    }
}


fn bottles_on_the_wall(num: usize) -> String {
    format!("{} on the wall", bottles_of_beer(num))
}


fn bottles_of_beer(num: usize) -> String {
    let count: String = match num {
        0 => "no more".to_string(),
        _ => num.to_string(),
    };

    let s: String = match num {
        1 => "".to_string(),
        _ => "s".to_string(),
    };

    format!("{} bottle{} of beer", count, s)
}


fn lets_do_it(num:  usize) -> String {
    let count: String = match num {
        0 => "it".to_string(),
        _ => "one".to_string()
    };

    match num {
        99 => "Go to the store and buy some more".to_string(),
        _ => format!("Take {} down and pass it around", count).to_string(),
    }
}


pub fn verse(num: usize) -> String {
    let next_round = match num {
        0 => 99,
        _ => num - 1,
    };

    format!("{}, {}.\n{}, {}.\n",
            capitalize(bottles_on_the_wall(num)),
            bottles_of_beer(num),
            lets_do_it(next_round),
            bottles_on_the_wall(next_round)
    )
}


pub fn sing(begin: usize, end: usize) -> String {
    assert!(begin <= 99);
    assert!(end <= 99);
    assert!(end < begin);

    let verses: Vec<String> = (end..begin + 1)
        .map(|x| verse(x))
        .rev()
        .collect();

    verses.join("\n")
}

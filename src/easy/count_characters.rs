use std::collections::HashMap;

pub fn run(input: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();

    for c in input.chars() {
        *map.entry(c).or_default() += 1;
    }

    map
}

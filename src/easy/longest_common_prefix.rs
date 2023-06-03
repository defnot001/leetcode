pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

    strs.sort_unstable();

    println!("{:#?}", strs);

    let mut common = vec![];

    let first_word = strs.first().unwrap().as_bytes();
    let last_word = strs.last().unwrap().as_bytes();

    let str_len = if first_word.len() < last_word.len() {
        first_word.len()
    } else {
        last_word.len()
    };

    for i in 0..str_len {
        let first_word_char = first_word[i];
        let last_word_char = last_word[i];

        println!("{} - {}", first_word_char, last_word_char);

        if first_word_char == last_word_char {
            common.push(first_word_char);
        } else {
            break;
        }
    }

    String::from_utf8_lossy(&common).to_string()
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;

    #[test]
    fn longest_common_prefix_works() {
        let test1 = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];

        let test2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

        assert_eq!(longest_common_prefix(test1), "fl".to_string());
        assert_eq!(longest_common_prefix(test2), "".to_string());
    }
}

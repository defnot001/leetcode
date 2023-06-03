mod easy;

fn main() {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];

    let strs2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

    println!(
        "{:#?}",
        easy::longest_common_prefix::longest_common_prefix(strs2)
    );
}

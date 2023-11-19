pub fn run(arr: &[u8]) -> Vec<u8> {
    let mut new_vec = Vec::new();
    let mut zero_count: u32 = 0;

    for &num in arr {
        if num != 0 {
            new_vec.push(num)
        } else {
            zero_count += 1
        }
    }

    new_vec.extend((0..zero_count).map(|n| 0));

    new_vec
}

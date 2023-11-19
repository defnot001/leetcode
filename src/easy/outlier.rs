use std::vec::Vec;

pub fn run(values: &[i32]) -> i32 {
    let even_count = values[..3].iter().filter(|&&x| x % 2 == 0).count();

    *values
        .iter()
        .find(|&&x| x as u32 % 2 == (even_count >= 2) as u32)
        .unwrap()
}

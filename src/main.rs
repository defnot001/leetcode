mod easy;

fn main() {
    let arr = vec![2, 7, 11, 15, 2, 5, 7, 3];
    let target = 18;

    println!("{:#?}", easy::two_sum::run_improved(arr, target));
}

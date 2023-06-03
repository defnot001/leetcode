pub fn roman_to_int(s: String) -> i32 {
    let mut sum = 0;
    let mut largest_seen = 0;

    for ch in s.chars().rev() {
        let num = match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => unreachable!(),
        };

        if num < largest_seen {
            sum -= num;
        } else {
            sum += num;
            largest_seen = num;
        }
    }

    return sum;
}

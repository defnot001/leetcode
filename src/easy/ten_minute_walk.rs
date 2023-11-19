pub fn run(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false;
    }

    let (mut east_west, mut north_south) = (0, 0);

    for &c in walk {
        match c {
            'n' => north_south += 1,
            's' => north_south -= 1,
            'e' => east_west += 1,
            'w' => east_west -= 1,
            _ => unreachable!(),
        }
    }

    east_west == 0 && north_south == 0
}

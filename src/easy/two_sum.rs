// O(n2)
pub fn run(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if j == i {
                continue;
            }

            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    panic!("How did we get here?");
}

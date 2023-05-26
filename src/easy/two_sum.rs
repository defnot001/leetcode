use std::collections::HashMap;

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

    unreachable!("How did we get here?");
}

pub fn run_improved(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut search_dict: HashMap<i32, i32> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        if let Some(&index) = search_dict.get(&(target - num)) {
            return vec![index as i32, i as i32];
        } else {
            search_dict.insert(num, i as i32);
        }
    }

    unreachable!("We really should not get there")
}

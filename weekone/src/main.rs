use std::collections::HashMap;

fn median_and_mode(nums: &Vec<i32>) -> (i32, i32) {
    let mut sorted = nums.clone();
    sorted.sort():


    let median = sorted[sorted.len() / 2 ];


    // calculating for the mode
    let mut counts = HashMap::new();
    for &num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut mode = nums[0];
    let mut max_count = 0;

    for (num, counts) in counts {
        if count > max_count {
            max_count = count;
            mode = num
        }
    }
    (median, mode)
}

fn main() {
    let numbers = vec![1,4,3,5,3,5,3,5];
    let (median, mode) = median_and_mode(&numbers);

    println!("Median : {}", median);
    println!("Mode: {}", mode);
}

use crate::solution::Solution;

impl Solution {
  /// Given an array of integers nums and an integer target, return indices of the two numbers such
  /// that they add up to target.
  ///
  /// You may assume that each input would have exactly one solution, and you may not use the same
  /// element twice.
  ///
  /// You can return the answer in any order.
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() < 2 || nums.len() == 2 && nums[0] + nums[1] == target {
      return vec![0, 1];
    }
    for left_index in 0..nums.len() {
      if nums.len() - left_index == 1 {
        break;
      }
      let right_indices = (left_index + 1)..nums.len();
      let left_num = nums[left_index];
      for right_index in right_indices {
        let right_num = nums[right_index];
        if left_num + right_num == target {
          return vec![left_index as i32, right_index as i32];
        }
      }
    }
    nums
  }
}

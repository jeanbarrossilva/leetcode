use crate::solution::Solution;

impl Solution {
  /// Given an array nums. We define a running sum of an array as runningSum[i] =
  /// sum(nums[0]…nums[i]).
  ///
  /// Return the running sum of nums.
  pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() <= 1 {
      return nums;
    }
    let mut sum = Vec::<i32>::with_capacity(nums.len());
    let mut index = 1;
    sum.push(nums[0]);
    loop {
      sum.push(sum[index - 1] + nums[index]);
      index += 1;
      if index == nums.len() {
        break;
      }
    }
    sum
  }
}

#[cfg(test)]
mod tests {
  use crate::solution::Solution;

  #[test]
  fn case1() {
    assert_eq!(vec![1, 3, 6, 10], Solution::running_sum(vec![1, 2, 3, 4]))
  }

  #[test]
  fn case2() {
    assert_eq!(
      vec![1, 2, 3, 4, 5],
      Solution::running_sum(vec![1, 1, 1, 1, 1])
    )
  }

  #[test]
  fn case3() {
    assert_eq!(
      vec![3, 4, 6, 16, 17],
      Solution::running_sum(vec![3, 1, 2, 10, 1])
    )
  }

  #[test]
  fn case4() {
    assert_eq!(vec![1], Solution::running_sum(vec![1]))
  }
}

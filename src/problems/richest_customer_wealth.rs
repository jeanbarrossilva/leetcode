use crate::solution::Solution;

impl Solution {
  /// You are given an m x n integer grid accounts where accounts[i][j] is
  /// the amount of money the i​​​​​​​​​​th​ customer has in the j​​​​​​​​​​​th​​​​ bank. Return the
  /// wealth that the richest customer has.
  ///
  /// A customer's wealth is the amount of money they have in all their
  /// bank accounts. The richest customer is the customer that has the
  /// maximum wealth.
  pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    if accounts.is_empty() {
      return 0;
    }
    let mut maximum_wealth = 0;
    for account in accounts {
      let mut wealth = 0;
      for amount in account {
        wealth += amount;
      }
      if wealth > maximum_wealth {
        maximum_wealth = wealth;
      }
    }
    maximum_wealth
  }
}

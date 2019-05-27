// @author  Shmish  "shmish90@gmail.com"
// @legal   MIT     "(c) Christopher K. Schmitt"


//! # Problem 001
//! 
//! Given a list of numbers, return whether any two sums to `k`. For example, 
//! given `[10, 15, 3, 7]` and `k` of `17`, return true since `10 + 7` is `17`.
//!
//! Bonus: Can you do this in one pass?


fn two_sum(list: &[i32], k: i32) -> bool {
  let mut set = std::collections::HashSet::new();

  for number in list {
    if set.contains(&(k - number)) {
      return true
    }
    set.insert(number);
  }

  false
}



#[test]
fn example_1() {
  assert!(two_sum(&vec![10, 15, 3, 7], 17) == true);
}

#[test]
fn example_2() {
  assert!(two_sum(&vec![3, 1, 4, 1, 5, 9], 6) == true);
}

#[test]
fn example_3() {
  assert!(two_sum(&vec![2, 7, 1, 8, 2], 13) == false);
}
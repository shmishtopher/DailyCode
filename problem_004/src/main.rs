// @author  Shmish  "shmish90@gmail.com"
// @legal   MIT     "(c) Christopher K. Schmitt"


//! # Problem 004
//! 
//! This problem was asked by Stripe.
//! 
//! Given an array of integers, find the first missing positive integer in 
//! linear time and constant space. In other words, find the lowest positive
//! integer that does not exist in the array. The array can contain duplicates 
//! and negative numbers as well. For example, the input `[3, 4, -1, 1]` should 
//! give `2`. The input `[1, 2, 0]` should give `3`. 
//! 
//! You can modify the input array in-place.


fn find_missing(list: &mut [i32]) -> i32 {
  for i in 0..list.len() {
    if list[i].is_positive() && list[i] < list.len() as i32 {
      list.swap(i, list[i] as usize);
      list[list[i] as usize] = list[i];
    }
  }

  for i in 1..list.len() {
    if i as i32 != list[i] {
      return i as i32
    }
  }

  list.last().map_or(1, |n| n + 1)
}


#[test]
fn example_1() {
  assert!(find_missing(&mut [3, 4, -1, 1]) == 2);
}

#[test]
fn example_2() {
  assert!(find_missing(&mut [1, 2, 0]) == 3);
}

#[test]
fn example_3() {
  assert!(find_missing(&mut []) == 1);
}
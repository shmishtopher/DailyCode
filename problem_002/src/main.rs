// @author  Shmish  "shmish90@gmail.com"
// @legal   MIT     "(c) Christopher K. Schmitt"


//! # Problem 002
//! 
//! This problem was asked by Uber.
//! 
//! Given an array of integers, return a new array such that each element at
//! index i of the new array is the product of all the numbers in the original
//! array except the one at i. For example, if our input was [1, 2, 3, 4, 5],
//! the expected output would be [120, 60, 40, 30, 24]. If our input was
//! [3, 2, 1], the expected output would be [2, 3, 6]. 
//! 
//! Follow-up: what if you can't use division?


// While this is an clean solution to the problem, it is miserably inefficient
// having a complexity of n^2 and should ever be used in production code.  It
// may be possible to improve efficency by cacheing the results of previous
// products

fn partial_product(list: &[i32]) -> Vec<i32> {
  let mut new_list = Vec::new();

  for (i, _) in list.iter().enumerate() {
    let left: i32 = list.iter().take(i).product();
    let right: i32 = list.iter().skip(i + 1).product();

    new_list.push(left * right);
  }

  new_list
}



#[test]
fn example_1() {
  assert!(partial_product(&vec![1, 2, 3, 4, 5]) == vec![120, 60, 40, 30, 24]);
}

#[test]
fn example_2() {
  assert!(partial_product(&vec![3, 2, 1]) == vec![2, 3, 6]);
}

#[test]
fn example_3() {
  assert!(partial_product(&vec![0, 1, 2]) == vec![2, 0, 0]);
}
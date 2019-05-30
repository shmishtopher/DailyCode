// @author  Shmish  "shmish90@gmail.com"
// @legal   MIT     "(c) Christopher K. Schmitt"


//! # Problem 005
//! 
//! This problem was asked by Jane Street.
//!
//! `cons(a, b)` constructs a pair, and `car(pair)` and `cdr(pair)` returns the first 
//! and last element of that pair. For example, `car(cons(3, 4))` returns `3`, and 
//! `cdr(cons(3, 4))` returns `4`.
//!
//! Given this implementation of cons:
//! ```
//! def cons(a, b):
//!    return lambda f : f(a, b)
//! ```
//! Implement `car` and `cdr`.


fn cons(a: i32, b: i32) -> impl Fn(&Fn(i32, i32) -> i32) -> i32 {
  move |func| func(a, b)
}


fn car(f: impl Fn(&Fn(i32, i32) -> i32) -> i32) -> i32 {
  f(&|left, _| left)
}


fn cdr(f: impl Fn(&Fn(i32, i32) -> i32) -> i32) -> i32 {
  f(&|_, right| right)
}



#[test]
fn example_1() {
  assert!(car(cons(3, 4)) == 3);
}


#[test]
fn example_2() {
  assert!(cdr(cons(3, 4)) == 4);
}
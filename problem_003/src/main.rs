// @author  Shmish  "shmish90@gmail.com"
// @legal   MIT     "(c) Christopher K. Schmitt"


//! # Problem 003
//! 
//! This problem was asked by Google.
//! 
//! Given the root to a binary tree, implement `serialize(root)`, which 
//! serializes the tree into a string, and `deserialize(s)`, which
//! deserializes the string back into the tree.


// WARNING!!!
// 
// This is LOW QUALITY code.  The style is poor and the algorithm is
// very inefficient.  Be sure to wear proper protection and make sure
// not to look at the program for any longer than five (5) minute
// sessions at a time.  I will not be heald responible for any damages
// caused by extended code viewing.


struct Tree {
  value: i32,
  left: Option<Box<Tree>>,
  right: Option<Box<Tree>>,
}


fn serialize(tree: &Tree) -> String {
  let mut left: String = String::default();
  let mut right: String = String::default();
  
  if let Some(tree) = &tree.left {
    left = serialize(tree);
  }

  if let Some(tree) = &tree.right {
    right = serialize(tree);
  }

  format!("{}({})({})", tree.value, left, right)
}


fn deserialize(tree: String) -> Tree {
  let mut left: Option<Box<Tree>> = Option::default();
  let mut right: Option<Box<Tree>> = Option::default();

  let mut indices: Vec<usize> = Vec::new();
  let mut counter: usize = 0;

  for (i, ch) in tree.char_indices() {
    if ch == '(' { counter += 1; }
    if ch == ')' { counter -= 1; }

    if counter == 1 && ch == '(' { indices.push(i); }
    if counter == 0 && ch == ')' { indices.push(i); }
  }

  if indices[1] - indices[0] > 1 {
    left = Some(Box::new(deserialize(tree[(indices[0] + 1)..indices[1]].to_owned())))
  }

  if indices[3] - indices[2] > 1 {
    right = Some(Box::new(deserialize(tree[(indices[2] + 1)..indices[3]].to_owned())))
  }

  Tree {
    value: tree[0..indices[0]].to_owned().parse::<i32>().unwrap(),
    left: left,
    right: right,
  }
}



#[test]
fn example_1() {
  let tree = Tree {
    value: 3,
    left: None,
    right: None,
  };

  assert!(serialize(&tree) == String::from("3()()"));
}


#[test]
fn example_2() {
  let tree = Tree {
    value: 20,
    left: None,
    right: None,
  };

  assert!(serialize(&tree) == String::from("20()()"));
}


#[test]
fn example_3() {
  let tree = Tree {
    value: 3,
    left: Some(Box::new(Tree {
      value: 2,
      left: None,
      right: None,
    })),
    right: Some(Box::new(Tree {
      value: 1,
      left: None,
      right: None,
    })),
  };

  assert!(serialize(&tree) == String::from("3(2()())(1()())"));
}


#[test]
fn example_4() {
  let tree = String::from("3()()");

  assert!(serialize(&deserialize(tree)) == String::from("3()()"));
}


#[test]
fn example_5() {
  assert!(serialize(&deserialize(String::from("30()()"))) == String::from("30()()"));
  assert!(serialize(&deserialize(String::from("2(2()())(2()(2()()))"))) == String::from("2(2()())(2()(2()()))"));
  assert!(serialize(&deserialize(String::from("20(20()())(20()(20()()))"))) == String::from("20(20()())(20()(20()()))"));
}
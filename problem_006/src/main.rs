// @author  Shmish  "shmish90@gmail.com"
// @legal   MIT     "(c) Christopher K. Schmitt"


//! # Problem 006
//! 
//! This problem was asked by Google.
//! 
//! An XOR linked list is a more memory efficient doubly linked list. Instead 
//! of each node holding next and prev fields, it holds a field named both, 
//! which is an XOR of the next node and the previous node. Implement an XOR 
//! linked list; it has an `add(element)` which adds the element to the end, and 
//! a `get(index)` which returns the node at index.
//! 
//! If using a language that has no pointers (such as Python), you can assume
//! you have access to `get_pointer` and `dereference_pointer` functions that 
//! converts between nodes and memory addresses.


// While this implementation is fairly clean and surprisingly easy to read,
// it is NOT a very "rusty" way of building a list.  A list like this one should
// almost never be used.


struct Node {
  value: usize,
  both: usize,
}

struct LinkedList {
  head: usize,
  last: usize,
}


impl LinkedList {
  fn new() -> Self {
    LinkedList {
      head: 0,
      last: 0,
    }
  }

  unsafe fn add(&mut self, value: usize) {
    let node = Node {
      value: value,
      both: self.last,
    };

    if self.head == 0 {
      let ptr = Box::into_raw(Box::new(node));
      self.head = ptr as usize;
      self.last = ptr as usize;
    }
    else {
      if let Some(last) = (self.last as *mut Node).as_mut() {
        let ptr = Box::into_raw(Box::new(node));
        self.last = ptr as usize;
        last.both ^= ptr as usize;
      }
    }
  }

  unsafe fn get(&self, index: usize) -> Option<usize> {
    let mut last_addr = 0usize;
    let mut curr_addr = self.head;

    for _ in 0..index {
      if let Some(current) = (curr_addr as *mut Node).as_ref() {
        let temp = curr_addr;
        curr_addr = current.both ^ last_addr;
        last_addr = temp;
      }
    }

    (curr_addr as *mut Node).as_ref().map(|node| node.value)
  }
}



#[test]
fn example_1() {
  unsafe {
    let mut list = LinkedList::new();
    list.add(9);
    list.add(8);
    list.add(7);

    assert!(list.get(0) == Some(9));
  }
}

#[test]
fn example_2() {
  unsafe {
    let mut list = LinkedList::new();
    list.add(9);
    list.add(8);
    list.add(7);

    assert!(list.get(1) == Some(8));
  }
}

#[test]
fn example_3() {
  unsafe {
    let mut list = LinkedList::new();
    list.add(9);
    list.add(8);
    list.add(7);

    assert!(list.get(2) == Some(7));
  }
}

#[test]
fn example_4() {
  unsafe {
    let mut list = LinkedList::new();
    list.add(9);
    list.add(8);
    list.add(7);

    assert!(list.get(3) == None);
  }
}
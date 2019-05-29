# DailyCode
This repository contains my solutions to the practice interview problems from [Daily Coding Problem](https://www.dailycodingproblem.com/) implemented in rust.  I encorage you to try the problems yourself before looking at my solutions.  Any questions may be directed to the [issues](https://github.com/shmishtopher/DailyCode/issues) tab or my inbox at shmish90@gmail.com.

## Problems

### Problem 001
Given a list of numbers, return whether any two sums to `k`. For example, 
given `[10, 15, 3, 7]` and `k` of `17`, return true since `10 + 7` is `17`.

Bonus: Can you do this in one pass?

[My Solution](https://github.com/shmishtopher/DailyCode/blob/master/problem_001/src/main.rs)


### Problem 002
This problem was asked by Uber.

Given an array of integers, return a new array such that each element at
index `i` of the new array is the product of all the numbers in the original
array except the one at `i`. For example, if our input was `[1, 2, 3, 4, 5]`,
the expected output would be `[120, 60, 40, 30, 24]`. If our input was
`[3, 2, 1]`, the expected output would be `[2, 3, 6]`. 

Follow-up: what if you can't use division?

[My Solution](https://github.com/shmishtopher/DailyCode/blob/master/problem_002/src/main.rs)


### Problem 003 
This problem was asked by Google.

Given the root to a binary tree, implement `serialize(root)`, which 
serializes the tree into a string, and `deserialize(s)`, which
deserializes the string back into the tree.

[My Solution](https://github.com/shmishtopher/DailyCode/blob/master/problem_003/src/main.rs)

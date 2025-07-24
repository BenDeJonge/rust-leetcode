# rust-leetcode

Just solvin' some LeetCode problems on the side in the best language in the
world (fight me). Hopefully learning some DSA along the way!

[My LeetCode profile](https://leetcode.com/u/BenZeen/).

## Overview

|     | Difficulty | Solved |
| --- | ---------- | ------ |
| 🟢  | Easy       | 27     |
| 🟡  | Medium     | 29     |
| 🔴  | Hard       | 8      |
|     | **Total**  | **64** |

A
[fetcher module](https://github.com/BenDeJonge/rust-leetcode/tree/main/src/fetcher)
is available to make web requests to LeetCode to automatically pull in question
data and create local resources.

```
$ cargo run fetch <problem-name>
```

Here, the problem name can be obtained from the URL:
`https://leetcode.com/problems/<problem-name>/`. Typically, this is simply the
title in lowercase with spaces replaced by dashes.

## Problem list

### 🟢 Easy

| Index | Name                                                                                                                                   | Tags                                                        |
| ----- | -------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| 0001  | [Two sum](https://leetcode.com/problems/two-sum/)                                                                                      | array, hash table                                           |
| 0009  | [Palindrome numbers](https://leetcode.com/problems/palindrome-number/)                                                                 | math                                                        |
| 0013  | [Roman to integers](https://leetcode.com/problems/roman-to-integer/)                                                                   | hash table, math, string                                    |
| 0014  | [Longest common prefix](https://leetcode.com/problems/longest-common-prefix/)                                                          | string, trie                                                |
| 0020  | [Valid parentheses](https://leetcode.com/problems/valid-parentheses/)                                                                  | string, stack                                               |
| 0021  | [Merge two sorted lists](https://leetcode.com/problems/merge-two-sorted-lists/)                                                        | linked list, recursion                                      |
| 0026  | [Remove duplicates from sorted array](https://leetcode.com/problems/merge-two-sorted-lists/)                                           | array, two pointers                                         |
| 0027  | [Remove elements](https://leetcode.com/problems/remove-element/)                                                                       | array, two pointers                                         |
| 0028  | [Find the index of the first occurence in a string](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/) | two pointers, string, string matching                       |
| 0035  | [Search insert position](https://leetcode.com/problems/search-insert-position/)                                                        | array, binary search                                        |
| 0070  | [Climbing stairs](https://leetcode.com/problems/climbing-stairs/)                                                                      | math, dynamic programming, memoization                      |
| 0094  | [Binary tree inorder traversal](https://leetcode.com/problems/binary-tree-inorder-traversal/)                                          | stack, tree, depth-first search, binary tree                |
| 0101  | [Symmetric tree](https://leetcode.com/problems/symmetric-tree/)                                                                        | tree, depth-first search, breadth-first search, binary tree |
| 0104  | [Maximum depth of binary tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/)                                            | tree, depth-first search, breadth-first search, binary tree |
| 0118  | [Pascals triangle](https://leetcode.com/problems/pascals-triangle/)                                                                    | array, dynamic programming                                  |
| 0121  | [Best time to buy and sell stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)                                      | array, dynamic programming                                  |
| 0136  | [Single number](https://leetcode.com/problems/single-number/)                                                                          | array, bit manipulation                                     |
| 0169  | [Majority element](https://leetcode.com/problems/majority-element/)                                                                    | array, hash table, divide and conquer, sorting, counting    |
| 0206 | [Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/) | linked-list, recursion |
| 0226  | [Invert binary tree](https://leetcode.com/problems/invert-binary-tree/)                                                                | tree, depth-first search, breadth-first search, binary tree |
| 0234 | [Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/) | linked-list, two-pointers, stack, recursion |
| 0283  | [Move zeroes](https://leetcode.com/problems/move-zeroes/)                                                                              | array, two pointer                                          |
| 0448  | [Find All Numbers Disappeared in an Array](https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/)                    | array, hash-table                                           |
| 0543  | [Diameter of a binary tree](https://leetcode.com/problems/diameter-of-a-binary-tree/)                                                  | tree, depth-first search, binary tree                       |
| 0704  | [Binary search](https://leetcode.com/problems/binary-search/)                                                                          | array, binary search                                        |
| 1518  | [Water bottles](https://leetcode.com/problems/water-bottles/)                                                                          | math, simulation                                            |
| 3617 | [Find the Original Typed String I](https://leetcode.com/problems/find-the-original-typed-string-i/) | string |

### 🟡 Medium

| Index | Name                                                                                                                            | Tags                                                    |
| ----- | ------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| 0002  | [Add two numbers](https://leetcode.com/problems/add-two-numbers/)                                                               | linked list, math, recursion                            |
| 0003  | [Longest substring without repeating characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/) | hash table, string, sliding window                      |
| 0005  | [Longest palindromic substring](https://leetcode.com/problems/longest-palindromic-substring/)                                   | two pointers, string, dynamic programming               |
| 0011  | [Container with the most water](https://leetcode.com/problems/container-with-most-water/)                                       | array, two pointers, greedy                             |
| 0015  | [3Sum](https://leetcode.com/problems/3sum/)                                                                                     | array, two pointers, sorting                            |
| 0017  | [Letter combinations of a phone number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)                   | hash table, string, backtracking                        |
| 0022  | [Generate parentheses](https://leetcode.com/problems/generate-parentheses/)                                                     | string, dynamic programming, backtracking               |
| 0045  | [Jump game II](https://leetcode.com/problems/jump-game-ii/)                                                                     | array, dynamic programming, greedy                      |
| 0046  | [Permutations](https://leetcode.com/problems/permutations/)                                                                     | array, backtracking                                     |
| 0047  | [Permutations II](https://leetcode.com/problems/permutations-ii/)                                                               | array, backtracking                                     |
| 0048  | [Rotate image](https://leetcode.com/problems/rotate-image/)                                                                     | array, math, matrix                                     |
| 0049  | [Group anagrams](https://leetcode.com/problems/group-anagrams/)                                                                 | array, hash table, string, sorting                      |
| 0053 | [Maximum Subarray](https://leetcode.com/problems/maximum-subarray/) | array, divide-and-conquer, dynamic-programming |
| 0054  | [Spiral matrix](https://leetcode.com/problems/spiral-matrix/)                                                                   | array, matrix, simulation                               |
| 0055  | [Jump game](https://leetcode.com/problems/jump-game/)                                                                           | array, dynamic programming, greedy                      |
| 0056  | [Merge Intervals](https://leetcode.com/problems/merge-intervals/)                                                               | array, sorting                                          |
| 0056  | [Merge Intervals](https://leetcode.com/problems/merge-intervals/)                                                               | array, sorting                                          |
| 0062  | [Unique Paths](https://leetcode.com/problems/unique-paths/)                                                                     | math, dynamic-programming, combinatorics                |
| 0064  | [Minimum Path Sum](https://leetcode.com/problems/minimum-path-sum/)                                                             | array, dynamic-programming, matrix                      |
| 0072  | [Edit Distance](https://leetcode.com/problems/edit-distance/)                                                                   | string, dynamic-programming                             |
| 0074  | [Search a 2D Matrix](https://leetcode.com/problems/search-a-2d-matrix/)                                                         | array, binary-search, matrix                            |
| 0079  | [Word Search](https://leetcode.com/problems/word-search/)                                                                       | array, string, backtracking, depth-first-search, matrix |
| 0167  | [Two sum ii](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)                                                   | array, two pointers, binary search                      |
| 0322  | [Coin Change](https://leetcode.com/problems/coin-change/)                                                                       | array, dynamic-programming, breadth-first-search        |
| 0442 | [Find All Duplicates in an Array](https://leetcode.com/problems/find-all-duplicates-in-an-array/) | array, hash-table |
| 1171 | [Shortest Path in Binary Matrix](https://leetcode.com/problems/shortest-path-in-binary-matrix/) | array, breadth-first-search, matrix |
| 1306  | [Jump Game III](https://leetcode.com/problems/jump-game-iii/)                                                                   | array, depth-first-search, breadth-first-search         |
| 1456 | [Find the City With the Smallest Number of Neighbors at a Threshold Distance](https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/) | dynamic-programming, graph, shortest-path |
| 2001  | [Jump Game VII](https://leetcode.com/problems/jump-game-vii/)                                                                   | string, dynamic-programming, sliding-window, prefix-sum |

### 🔴 Hard

| Index | Name                                                                                      | Tags                                                               |
| ----- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| 0004  | [Median of two sorted arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/) | array, binary search, divide and conquer                           |
| 0023  | [Merge k sorted lists](https://leetcode.com/problems/merge-k-sorted-lists/)               | linked list, divide and conquer, heap (priority queue), merge sort |
| 0032  | [Longest valid parentheses](https://leetcode.com/problems/longest-valid-parentheses/)     | string, dynamic programming, stack                                 |
| 0041  | [First Missing Positive](https://leetcode.com/problems/first-missing-positive/)           | array, hash-table                                                  |
| 0042  | [Longest valid parentheses](https://leetcode.com/problems/trapping-rain-water/)           | array, two pointer, dynamic programming, stack, monotonic stack    |
| 0051  | [N-Queens](https://leetcode.com/problems/n-queens/)                                       | array, backtracking                                                |
| 0052  | [N-Queens II](https://leetcode.com/problems/n-queens-ii/)                                 | backtracking                                                       |
| 1447  | [Jump Game IV](https://leetcode.com/problems/jump-game-iv/)                               | array, hash-table, breadth-first-search                            |

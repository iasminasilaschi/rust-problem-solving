# Rust Problem Solving

This repository contains solutions to various algorithmic challenges from HackerRank implemented in Rust.

## ATAD Grade Computation

- **Default**: 1
- **Easy**: 4 x 0.5 = 2
- **Medium**: 3 x 1 = 3
- **Hard**: 2 x 2 = 4
- **Total**: 1 + 2 + 3 + 4 = 10

## Solved Problems

### 1. Solve Me First (easy)

- **Source Code**: [solve_me_first.rs](./src/solve_me_first.rs)
- **Description**: This program takes two integers as input, computes their sum, and outputs the result.
- **How it Works**:
    - Reads two lines of input, each representing an integer.
    - Parses the strings into integers using `trim` and `parse`.
    - Computes the sum of the two integers.
    - Prints the result to the console.

---

### 2. Simple Array Sum (easy)

- **Source Code**: [simple_array_sum.rs](./src/simple_array_sum.rs)
- **Description**: This program calculates the sum of all elements in an array of integers.
- **How it Works**:
    - The function `simpleArraySum` takes a slice of integers (`&[i32]`) as input.
    - It uses the `.iter().sum()` method to iterate through the array and compute the sum of its elements.
    - The result is returned as an integer.

---

### 3. Drawing Book (easy)

- **Source Code**: [drawing_book.rs](./src/drawing_book.rs)
- **Description**: This program calculates the minimum number of page flips required to reach a specific page in a book.
  The user can flip pages either from the front or the back of the book, and the program determines the optimal
  direction.
- **How it Works**:
    - **From Front**: Calculates the number of flips needed to reach the target page starting from the front: `p / 2`.
    - **From Back**: Calculates the number of flips needed to reach the target page starting from the back:
      `(n / 2) - (p / 2)`.
    - **Optimization**: Returns the minimum value between the flips from the front and the flips from the back.

---

### 4. Sherlock and Squares (easy)

- **Source Code**: [sherlock_and_squares.rs](./src/sherlock_and_squares.rs)
- **Description**: This program determines the number of square integers within a given range `[a, b]`. A square integer
  is an integer that is the square of another integer (e.g., `4 = 2^2`, `9 = 3^2`).
- **How it Works**:
    - **Lower Bound**: Computes the smallest integer whose square is greater than or equal to `a` using
      `(a as f64).sqrt().ceil()`.
    - **Upper Bound**: Computes the largest integer whose square is less than or equal to `b` using
      `(b as f64).sqrt().floor()`.
    - **Count Squares**: If the lower bound is greater than the upper bound, there are no square integers in the range.
      Otherwise, calculates the count as `upper_bound - lower_bound + 1`.

---

### 5. Forming a Magic Square (medium)

- **Source Code**: [forming_magic_square.rs](./src/forming_magic_square.rs)
- **Description**: This program takes a 3x3 matrix of integers and transforms it into a "magic square" at the minimal
  cost. A magic square is a 3x3 grid where the sum of any row, column, or diagonal is the same.
- **How it Works**:
    - **Magic Squares**: Predefines all possible 3x3 magic squares as reference matrices.
    - **Cost Calculation**: For each possible magic square, calculates the total cost to convert the input matrix into
      that magic square. The cost of changing one number to another is the absolute difference.
    - **Optimization**: Determines the minimum cost among all possibilities and outputs it.
    - **Constraints**: Each number in the matrix must be a distinct integer between 1 and 9.

---

### 6. Climbing the Leaderboard (medium)

- **Source Code**: [climbing_leaderboard.rs](./src/climbing_the_leaderboard.rs)
- **Description**: This program calculates the player's rank on a dense leaderboard after each game. The leaderboard
  ranks are determined by unique scores in descending order.
- **How it Works**:
    - **Unique Ranks**: Removes duplicate scores from the leaderboard to create a unique list of scores in descending
      order.
    - **Rank Calculation**: For each player's score, compares it with the scores in the leaderboard to determine their
      rank.
    - **Efficient Traversal**: Iterates over the player's scores and adjusts the rank index efficiently to avoid
      redundant comparisons.

---

### 7. Non-Divisible Subset (medium)

- **Source Code**: [non_divisible_subset.rs](./src/non_divisible_subset.rs)
- **Description**: This program computes the size of the largest subset of integers such that the sum of any two
  integers in the subset is not divisible by a given number \( k \).
- **How it Works**:
    - **Step 1: Calculate Remainders**:
        - Compute the remainders of all elements in the array when divided by \( k \).
        - Count the frequency of each remainder.
    - **Step 2: Subset Construction**:
        - Include at most one element with a remainder of `0` since any two such elements would sum to \( k \).
        - For each pair of remainders \( i \) and \( k - i \), include the group with the larger count.
        - If \( k \) is even, include at most one element with a remainder of \( k/2 \) to avoid summing to \( k \).
    - **Output**:
        - Print the size of the largest subset that satisfies the condition.

---

### 8. Array Manipulation (hard)

- **Source Code**: [array_manipulation.rs](./src/array_manipulation.rs)
- **Description**: This program calculates the maximum value in an array after performing a series of range update
  operations. It uses an efficient difference array approach to handle range updates without directly modifying all
  elements in the range.
- **How it Works**:
    - **Difference Array**:
        - For each query `(a, b, k)`, add `k` to `arr[a]` and subtract `k` from `arr[b + 1]` (if within bounds).
        - This ensures that range updates are performed in \(O(1)\) time for each query.
    - **Prefix Sum**:
        - Compute the prefix sum of the difference array to get the final values of the array after all operations.
        - Track the maximum value during this step.

---

### 9. Array and Simple Queries (hard)

- **Source Code**: [array_and_simple_queries.rs](./src/array_and_simple_queries.rs)
- **Description**: This program modifies an array based on a series of queries of two types:
    - **Type 1**: Remove a subarray from indices `[i, j]` and add it to the front of the array.
    - **Type 2**: Remove a subarray from indices `[i, j]` and add it to the back of the array.
      After processing all queries, the program calculates and outputs the absolute difference between the first and
      last elements of the resulting array, along with the final array state.
- **How it Works**:
    - **Input Parsing**:
        - Reads the array size (`n`), number of queries (`q`), the array, and the list of queries.
    - **Processing Queries**:
        - Uses the `drain` method to extract subarrays for each query.
        - Inserts the extracted subarray at the front (`splice`) or appends it to the back (`extend`), depending on the
          query type.
    - **Result Calculation**:
        - Computes the absolute difference between the first and last elements of the final array: `|A[0] - A[n-1]|`.
    - **Output**:
        - Prints the absolute difference followed by the elements of the final array.

---

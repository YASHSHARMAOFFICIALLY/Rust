# Rust Daily Challenge Journey

This repository is my daily Rust learning journey. I am solving small Rust problems every day and using each challenge to understand Rust concepts through practice.

## Day 1

### Problems Solved

- Calculator
- Guess the Number

### What I Practiced

- Reading user input with `std::io`
- Parsing string input into numbers
- Handling invalid input with `match`
- Using `loop` for repeated input until the user enters valid data
- Using `break` to exit a loop
- Using `continue` to skip invalid attempts and ask again
- Generating a random integer with the `rand` crate
- Comparing values with `if`, `else if`, and `else`
- Using `match` for calculator operations
- Checking edge cases like division by zero and modulo by zero

### Projects

#### Calculator

Path: `day01/calculator`

A command-line calculator that asks the user for two numbers and an operator. It supports:

- Addition
- Subtraction
- Multiplication
- Division
- Modulo

It also checks invalid number input, invalid operators, division by zero, and modulo by zero.

Run it with:

```bash
cd day01/calculator
cargo run
```

#### Guess the Number

Path: `day01/guess_number`

A command-line guessing game where the program generates a random number between 1 and 100. The user keeps guessing until they find the correct number.

It also:

- Counts the number of attempts
- Tells the user if the guess is too low or too high
- Updates the valid guessing range after each guess
- Checks invalid input
- Checks guesses outside the current range

Run it with:

```bash
cd day01/guess_number
cargo run
```

## Current Goal

Keep solving one or more Rust problems every day and document what I learn from each challenge.

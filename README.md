# Rust Learning Journey 🦀

This repository documents my progress while learning Rust through small exercises and experiments.

## What I'm Doing

- Following a Rust beginner to proficient course covering all essential topics necessary to build robust and efficient software.
- Experimenting with Rust code using Cargo
- Practicing concepts through small example programs
- Exploring memory concepts like ownership and borrowing

## Topics Covered So Far

### Rust Setup & Tooling

- Installing Rust
- Using Cargo (Rust package manager and build tool)
- Creating and running Rust projects

### Core Language Fundamentals

- Primitive data types (`i32`, `u32`, `f64`, `bool`, `char`)
- Signed vs unsigned integers
- Arrays and tuples
- Expressions vs statements
- Basic functions

### Memory & Ownership Concepts

- Stack vs heap allocation
- `String` vs `&str`
- References and borrowing
- String slices
- Memory efficiency through references

### Variables & Scope

- Variable mutability (`mut`)
- Constants and global scope rules
- Shadowing in Rust
  - Shadowing ≠ mutability
  - Allows type changes during reassignment
  - Useful for transformations without explicit type annotations

### Control Flow

- `if` / `else` conditions
- `loop`
- `while`
- `for` loops
- Loop labels and breaking from outer loops

### Error Handling

- `Option<T>`
  - `Some(T)`
  - `None`
  - Handling absence of values safely
- `Result<T, E>`
  - `Ok(T)`
  - `Err(E)`
  - Handling recoverable errors

### Collection Types

- Vectors (`Vec<T>`)
- UTF-8 Strings
- HashMaps

## Repository Structure

To be organized...

Each folder contains small exercises and experiments while learning Rust concepts.

## Goal

Understand Rust fundamentals and eventually build more advanced programs using:

- Ownership and borrowing
- Structs and enums
- Pattern matching
- Cargo projects

## Recall Lessons (Accuracy Check)

Review previously learned Rust concepts **from memory first**, then verify accuracy by revisiting the lesson material.

Goal:

- Strengthen long-term retention
- Identify gaps in understanding
- Practice explaining concepts clearly

### Method

1. Write explanations **from memory** without looking at notes.
2. Provide a short code example if possible.
3. Revisit the lesson/tutorial.
4. Correct misunderstandings and update notes.

### Recall Topics

**Core Language**

- Functions
- Variables
- Mutability (`mut`)
- Constants
- Shadowing

**Ownership System**

- Ownership rules
- Move semantics
- Copy vs move types
- Stack vs heap memory
- Drop behavior

**Borrowing System**

- References (`&T`)
- Mutable references (`&mut T`)
- Borrowing rules
- Dangling references

**Control Flow**

- `if` / `else`
- `loop`
- `while`
- `for`

**Error Handling**

- `Option<T>`
- `Result<T, E>`
- Pattern matching with `match`

**Collections**

- Vectors (`Vec<T>`)
- Strings (`String`)
- HashMaps (`HashMap<K, V>`)

### Expected Output

For each concept:

- Short explanation
- Key rules
- Small code snippet
- Example use case

### Objective

Be able to **explain the concept clearly without looking at the documentation** and implement a small example from memory.

- Shorthand principles
- Concept breakdown
- systems thinking

  Learn → Recall → Verify → Refine

## Next Steps

- Practice with Rustlings
- Build a small end-to-end Rust project
- Explore structs and enums deeper
- Work with modules and project structure

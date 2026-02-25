# Collatz-Viz

A Rust CLI tool to visualize Collatz sequences as interactive graphs in a GUI window.

## What is the Collatz Conjecture?

The Collatz conjecture is one of the most famous unsolved problems in mathematics. Starting with any positive integer n:
- If n is even, divide it by 2
- If n is odd, multiply by 3 and add 1

The conjecture states that no matter what number you start with, you will always eventually reach 1.

## Features

- **Sequence Mode**: Visualize the path a number takes to reach 1
- **Inverse Mode**: Visualize the tree of numbers that eventually lead to a target number
- Interactive GUI with graph visualization showing nodes and connections

## Installation

```bash
cargo build --release
```

## Usage

### Visualize a Collatz Sequence

Show the path a number takes to reach 1:

```bash
cargo run --release -- sequence 27
```

This will open a GUI window showing the sequence: 27 → 82 → 41 → 124 → ... → 1

### Visualize Inverse Collatz Tree

Show all numbers that lead to a target number (up to a certain depth):

```bash
cargo run --release -- inverse 1 --depth 5
```

This will show a tree of all numbers that eventually reach 1, exploring up to 5 levels of predecessors.

You can also specify a different target number:

```bash
cargo run --release -- inverse 4 --depth 3
```

### Options

- `sequence <NUMBER>`: Visualize the Collatz sequence starting from NUMBER
- `inverse <NUMBER>`: Visualize the inverse tree leading to NUMBER
  - `-d, --depth <DEPTH>`: Maximum depth of the tree (default: 5)

### Examples

```bash
# Show the sequence for the number 27 (which has a notably long sequence)
cargo run --release -- sequence 27

# Show the inverse tree for 1 with depth 6
cargo run --release -- inverse 1 --depth 6

# Show the sequence for 7
cargo run --release -- sequence 7
```

## How It Works

- **Normal Sequence**: Follows the Collatz rules forward from a starting number until reaching 1
- **Inverse Tree**: Works backwards from a target number to find all possible predecessors:
  - Every number n has a predecessor 2n (since n/2 = n when even)
  - Odd numbers n where n ≡ 1 (mod 3) have a predecessor (n-1)/3 (since 3k+1 = n)

## Graph Visualization

The GUI displays:
- **Nodes**: Circles representing numbers in the sequence/tree
- **Edges**: Arrows showing the direction of transformation
- **Layout**: Tree-like structure with levels indicating transformation steps

## Requirements

- Rust 1.70 or later
- A display server (X11, Wayland, or Windows/macOS native)

## License

This is a fun educational project exploring the Collatz conjecture.

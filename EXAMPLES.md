# Example Visualizations

This document shows examples of what the Collatz visualizations look like.

## Sequence Mode - Example: Starting from 3

The sequence 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1 would be visualized as:

```
     3
     ↓
    10
     ↓
     5
     ↓
    16
     ↓
     8
     ↓
     4
     ↓
     2
     ↓
     1
```

## Sequence Mode - Example: Starting from 7

The sequence for 7 demonstrates a longer path:
7 → 22 → 11 → 34 → 17 → 52 → 26 → 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

```
 7 → 22 → 11 → 34 → 17 → 52 → 26 → 13
                                    ↓
                                   40
                                    ↓
                                   20
                                    ↓
                                   10 → 5 → 16 → 8 → 4 → 2 → 1
```

## Inverse Mode - Example: Target = 1, Depth = 3

Shows the tree of numbers that eventually reach 1:

```
         21 ─┐    42
         ↓   ↓    ↓
    5 ─  10 ─── 20
    ↓     ↓
    16    4 ─── 8
     ↓    ↓     ↓
      ─── 2 ────┘
           ↓
           1
```

In this inverse tree:
- 2 leads to 1 (2 ÷ 2 = 1)
- 4 leads to 2 (4 ÷ 2 = 2)
- 8 leads to 4, etc.
- Odd numbers like 5 lead to 16 (5 × 3 + 1 = 16)

## Inverse Mode - Example: Target = 4, Depth = 2

```
    5 ──┐
        ↓
        16
        ↓
    8 ─→ 4 ←── 13
```

## How to Use

Run the commands to see these visualizations in an interactive GUI window:

```bash
# Sequence examples
cargo run --release -- sequence 3
cargo run --release -- sequence 7
cargo run --release -- sequence 27

# Inverse examples
cargo run --release -- inverse 1 --depth 3
cargo run --release -- inverse 4 --depth 2
cargo run --release -- inverse 1 --depth 6
```

## GUI Features

The interactive GUI window shows:
- **Nodes**: Blue circles representing each number
- **Labels**: Each node shows its number value
- **Edges**: Gray arrows showing the transformation direction
- **Layout**: Automatic tree-like arrangement with levels
- **Navigation**: The window can be resized and the view is centered

The visualization makes it easy to see patterns in the Collatz sequence, such as:
- How different starting numbers converge to the same paths
- The branching structure of the inverse tree
- The relative lengths of different sequences

#!/usr/bin/env python3
"""
Odd Collatz Tree Printer

Prints the odd Collatz tree starting at 1, showing the binary representation
of each number as the primary display with decimal for reference.

In the odd Collatz tree an odd number m is a *child* of odd number n when
applying the Collatz step (3m + 1) and then repeatedly halving the even result
eventually yields n.  Equivalently:

    3m + 1 = n * 2^k  for some positive integer k
    m = (n * 2^k - 1) / 3

Only values of k for which the right-hand side is a positive odd integer are
valid children.  Because n * 2^k is always even, n * 2^k - 1 is always odd,
so whenever the expression is divisible by 3 the result is automatically odd.

Odd multiples of 3 have *no* children: n ≡ 0 (mod 3) means
n * 2^k - 1 ≡ -1 (mod 3) for every k, so no valid k exists.

Usage:
  python odd_collatz_tree.py -b BRANCHING -d DEPTH
  python odd_collatz_tree.py --branching BRANCHING --depth DEPTH
"""

import argparse
import sys
from dataclasses import dataclass, field
from typing import Iterator, List, Optional, Tuple


# ---------------------------------------------------------------------------
# Core Collatz math
# ---------------------------------------------------------------------------

def odd_collatz_children(n: int) -> Iterator[int]:
    """
    Yield the odd Collatz tree children of odd number *n* in the order they
    are encountered while incrementing k from 1.

    A child m satisfies  3m + 1 = n * 2^k  for some positive integer k,
    i.e.  m = (n * 2^k - 1) / 3.  Only integer (automatically odd) results
    are yielded.  The trivial fixed-point case (n = 1, k = 2 → child = 1) is
    skipped so the tree stays acyclic.
    """
    k = 1
    while True:
        numerator = n * (1 << k) - 1
        if numerator % 3 == 0:
            child = numerator // 3
            if child != n:          # skip the n = 1 fixed point
                yield child
        k += 1


def has_odd_collatz_children(n: int) -> bool:
    """
    Return True if *n* has any odd Collatz children.

    Odd multiples of 3 have no children; all other odd numbers have
    infinitely many.
    """
    return n % 3 != 0


# ---------------------------------------------------------------------------
# Tree data structure
# ---------------------------------------------------------------------------

@dataclass
class OddCollatzNode:
    """One node in the odd Collatz tree."""

    value: int
    children: List["OddCollatzNode"] = field(default_factory=list)
    # True when more children exist beyond the ones stored in `children`
    has_more_children: bool = False

    def all_values(self) -> Iterator[int]:
        """Yield every integer value present in this subtree."""
        yield self.value
        for child in self.children:
            yield from child.all_values()


def build_odd_collatz_tree(
    n: int,
    branching: int,
    max_depth: int,
    _current_depth: int = 0,
) -> OddCollatzNode:
    """
    Build the odd Collatz tree rooted at *n*.

    Args:
        n:              Root value (must be a positive odd integer).
        branching:      Maximum number of children to expand per node.
        max_depth:      Maximum depth to expand; 0 = root node only.
        _current_depth: Internal counter – do not pass manually.

    Returns:
        An :class:`OddCollatzNode` tree expanded to *max_depth* levels.
    """
    can_have_children = has_odd_collatz_children(n)

    # At maximum depth: keep the node but do not expand children
    if _current_depth >= max_depth:
        return OddCollatzNode(
            value=n,
            children=[],
            has_more_children=False,
        )

    # Odd multiple of 3: no children exist in the Collatz tree
    if not can_have_children:
        return OddCollatzNode(value=n, children=[], has_more_children=False)

    # Collect the first `branching` children and recurse
    children_values: List[int] = []
    for child in odd_collatz_children(n):
        children_values.append(child)
        if len(children_values) >= branching:
            break

    children = [
        build_odd_collatz_tree(c, branching, max_depth, _current_depth + 1)
        for c in children_values
    ]

    return OddCollatzNode(
        value=n,
        children=children,
        has_more_children=True,   # always more for non-multiples of 3
    )


# ---------------------------------------------------------------------------
# Rendering
# ---------------------------------------------------------------------------

# Each display line is: (indent_prefix, connector, value_or_None, is_ellipsis)
_DisplayLine = Tuple[str, str, Optional[int], bool]


def _collect_display_lines(root: OddCollatzNode) -> List[_DisplayLine]:
    """
    Walk the tree and return a flat list of display-line descriptors.

    Each descriptor is a 4-tuple:
      (prefix, connector, value, is_ellipsis)

    * prefix    – vertical-bar continuation string for ancestor levels
    * connector – "├── ", "└── ", or "" (root)
    * value     – integer for normal nodes; None for ellipsis lines
    * is_ellipsis – True for the "..." marker line
    """
    lines: List[_DisplayLine] = []

    def traverse(
        node: OddCollatzNode,
        prefix: str,
        is_last: bool,
        depth: int,
    ) -> None:
        if depth == 0:
            # Root: no connector, no indent
            connector = ""
            child_prefix = ""
        else:
            connector = "└── " if is_last else "├── "
            child_prefix = prefix + ("    " if is_last else "│   ")

        lines.append((prefix, connector, node.value, False))

        n_children = len(node.children)
        for i, child in enumerate(node.children):
            # The last visual item under this node is either the last real
            # child (when there are no more beyond) or the "..." marker.
            is_last_child = (i == n_children - 1) and not node.has_more_children
            traverse(child, child_prefix, is_last_child, depth + 1)

        # Ellipsis line indicating infinite additional children
        if node.has_more_children:
            lines.append((child_prefix, "└── ", None, True))

    traverse(root, "", True, 0)
    return lines


def format_tree(root: OddCollatzNode, max_depth: int) -> str:
    """
    Format the tree as a multi-line string.

    Binary representations are the primary display; they are right-aligned so
    that all " = " separators fall in the same column.  Decimal values follow
    for easy reference.  Ellipsis lines show that each node has infinitely
    more children beyond the *branching* displayed.

    The alignment column is  4 * max_depth + max_binary_width, where
    4 * max_depth accounts for the deepest tree-connector prefix and
    max_binary_width is the width of the longest binary number in the tree.
    """
    lines = _collect_display_lines(root)

    all_vals = list(root.all_values())
    if not all_vals:
        return ""

    max_bin_len = max(len(bin(v)[2:]) for v in all_vals)
    max_prefix_width = 4 * max_depth   # chars consumed by deepest connector

    result_lines: List[str] = []
    for prefix, connector, value, is_ellipsis in lines:
        prefix_and_connector = prefix + connector
        if is_ellipsis:
            result_lines.append(f"{prefix_and_connector}...")
        else:
            # Pad so that binary numbers end at the same absolute column
            padding = max_prefix_width - len(prefix_and_connector)
            bin_str = bin(value)[2:].rjust(max_bin_len)
            result_lines.append(
                f"{prefix_and_connector}{' ' * padding}{bin_str} = {value}"
            )

    return "\n".join(result_lines)


def print_odd_collatz_tree(
    branching: int,
    depth: int,
    output=sys.stdout,
) -> None:
    """Build and print the odd Collatz tree to *output*."""
    root = build_odd_collatz_tree(1, branching, depth)
    print(format_tree(root, depth), file=output)


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def main() -> None:
    parser = argparse.ArgumentParser(
        prog="odd_collatz_tree.py",
        description=(
            "Print the odd Collatz tree starting at 1.\n"
            "Binary representations are shown as the primary display,\n"
            "right-aligned together, with decimal values for reference.\n"
            "An ellipsis (...) marks the infinitely-many additional children\n"
            "beyond the displayed branching factor."
        ),
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=(
            "Examples:\n"
            "  python odd_collatz_tree.py -b 4 -d 1   # depth 1, branching 4\n"
            "  python odd_collatz_tree.py -b 5 -d 6   # depth 6, branching 5\n"
        ),
    )
    parser.add_argument(
        "-b", "--branching",
        type=int,
        required=True,
        metavar="BRANCHING",
        help="Number of children to display per node (branching factor)",
    )
    parser.add_argument(
        "-d", "--depth",
        type=int,
        required=True,
        metavar="DEPTH",
        help="Maximum depth of the tree (0 = root node only)",
    )

    args = parser.parse_args()

    if args.branching < 1:
        parser.error("Branching factor must be at least 1")
    if args.depth < 0:
        parser.error("Depth must be non-negative")

    print_odd_collatz_tree(args.branching, args.depth)


if __name__ == "__main__":
    main()

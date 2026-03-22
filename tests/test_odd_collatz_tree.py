"""
Tests for odd_collatz_tree.py
"""

import io
import os
import sys

import pytest

# Allow importing from the repository root
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from odd_collatz_tree import (
    OddCollatzNode,
    build_odd_collatz_tree,
    format_tree,
    has_odd_collatz_children,
    odd_collatz_children,
    print_odd_collatz_tree,
)

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

_TESTS_DIR = os.path.dirname(os.path.abspath(__file__))
_REFERENCE_FILE = os.path.join(_TESTS_DIR, "odd_collatz_d6_b5.txt")


def _capture(branching: int, depth: int) -> str:
    """Return the printed tree as a string."""
    buf = io.StringIO()
    print_odd_collatz_tree(branching, depth, buf)
    return buf.getvalue()


def _eq_cols(text: str) -> set:
    """Return the set of column positions where ' = ' appears (value lines)."""
    cols = set()
    for line in text.split("\n"):
        if " = " in line:
            cols.add(line.index(" = "))
    return cols


# ---------------------------------------------------------------------------
# Formula / child-generation tests
# ---------------------------------------------------------------------------

class TestOddCollatzChildren:
    def test_children_of_1(self):
        """The first five children of 1 are 5, 21, 85, 341, 1365."""
        gen = odd_collatz_children(1)
        assert [next(gen) for _ in range(5)] == [5, 21, 85, 341, 1365]

    def test_children_of_5(self):
        """5 ≡ 2 (mod 3) → odd k values give 3, 13, 53, …"""
        gen = odd_collatz_children(5)
        assert [next(gen) for _ in range(3)] == [3, 13, 53]

    def test_children_of_13(self):
        """13 ≡ 1 (mod 3) → even k values give 17, 69, 277, …"""
        gen = odd_collatz_children(13)
        assert [next(gen) for _ in range(3)] == [17, 69, 277]

    def test_child_collatz_step(self):
        """Every yielded child must eventually reach its parent via Collatz."""
        def odd_step(m):
            # Apply 3m+1 then divide by 2 until odd
            v = 3 * m + 1
            while v % 2 == 0:
                v //= 2
            return v

        for parent in [1, 5, 7, 13, 17, 85]:
            gen = odd_collatz_children(parent)
            for _ in range(4):
                child = next(gen)
                assert odd_step(child) == parent, (
                    f"odd_step({child}) should equal {parent}"
                )

    def test_children_are_odd(self):
        """All generated children must be odd."""
        for n in [1, 5, 7, 13, 17, 85, 341]:
            gen = odd_collatz_children(n)
            for _ in range(6):
                child = next(gen)
                assert child % 2 == 1, f"child {child} of {n} is not odd"


class TestHasOddCollatzChildren:
    def test_non_multiples_of_3_have_children(self):
        assert has_odd_collatz_children(1) is True
        assert has_odd_collatz_children(5) is True
        assert has_odd_collatz_children(7) is True
        assert has_odd_collatz_children(13) is True
        assert has_odd_collatz_children(17) is True

    def test_odd_multiples_of_3_have_no_children(self):
        assert has_odd_collatz_children(3) is False
        assert has_odd_collatz_children(9) is False
        assert has_odd_collatz_children(15) is False
        assert has_odd_collatz_children(21) is False
        assert has_odd_collatz_children(27) is False


# ---------------------------------------------------------------------------
# Tree-building tests
# ---------------------------------------------------------------------------

class TestBuildTree:
    def test_depth_0_root_only(self):
        tree = build_odd_collatz_tree(1, 4, 0)
        assert tree.value == 1
        assert tree.children == []
        # Root has infinite children, even though we don't expand them
        assert tree.has_more_children is True

    def test_depth_0_no_children_for_multiple_of_3(self):
        tree = build_odd_collatz_tree(3, 4, 0)
        assert tree.value == 3
        assert tree.children == []
        assert tree.has_more_children is False

    def test_depth_1_branching_4(self):
        tree = build_odd_collatz_tree(1, 4, 1)
        assert [c.value for c in tree.children] == [5, 21, 85, 341]
        assert tree.has_more_children is True
        # Children at max depth: leaves have no displayed children
        for c in tree.children:
            assert c.children == []

    def test_depth_1_child_21_no_further_children(self):
        """21 = 3 × 7 so it has no odd Collatz children."""
        tree = build_odd_collatz_tree(1, 4, 1)
        child_21 = next(c for c in tree.children if c.value == 21)
        assert child_21.has_more_children is False

    def test_depth_1_child_5_has_more_children(self):
        """5 is not a multiple of 3, so it has children."""
        tree = build_odd_collatz_tree(1, 4, 1)
        child_5 = next(c for c in tree.children if c.value == 5)
        assert child_5.has_more_children is True

    def test_depth_2_branching_3(self):
        tree = build_odd_collatz_tree(1, 3, 2)
        assert tree.value == 1
        assert len(tree.children) == 3
        # Children of 5
        child_5 = tree.children[0]
        assert child_5.value == 5
        assert [c.value for c in child_5.children] == [3, 13, 53]
        # 3 = 3×1 — no children
        assert child_5.children[0].has_more_children is False

    def test_all_values_collected(self):
        tree = build_odd_collatz_tree(1, 2, 2)
        vals = list(tree.all_values())
        # Root + 2 depth-1 + 2×2 depth-2 = 7 values (some may have no children)
        assert 1 in vals
        assert 5 in vals
        assert 21 in vals


# ---------------------------------------------------------------------------
# Output format / alignment tests
# ---------------------------------------------------------------------------

class TestFormatTree:
    def test_depth_0_has_root_and_ellipsis(self):
        text = _capture(4, 0)
        lines = text.strip().split("\n")
        assert len(lines) == 2
        assert "1 = 1" in lines[0]
        assert "..." in lines[1]

    def test_depth_1_branching_4_contains_all_values(self):
        text = _capture(4, 1)
        for v in [1, 5, 21, 85, 341]:
            assert f"= {v}" in text, f"Expected '= {v}' in output"

    def test_binary_representations_present(self):
        text = _capture(4, 1)
        assert "101" in text          # 5
        assert "10101" in text        # 21
        assert "1010101" in text      # 85
        assert "101010101" in text    # 341

    def test_global_alignment_depth_1_branching_4(self):
        """All ' = ' must appear at exactly one column position."""
        text = _capture(4, 1)
        assert len(_eq_cols(text)) == 1

    def test_global_alignment_depth_2_branching_3(self):
        text = _capture(3, 2)
        assert len(_eq_cols(text)) == 1

    def test_global_alignment_depth_3_branching_5(self):
        text = _capture(5, 3)
        assert len(_eq_cols(text)) == 1

    def test_21_has_no_ellipsis_child(self):
        """21 = 3×7 has no odd Collatz children, so no '...' follows it."""
        text = _capture(4, 1)
        lines = text.strip().split("\n")
        for i, line in enumerate(lines):
            if "= 21" in line:
                # The next line must NOT be an ellipsis for 21's children
                next_line = lines[i + 1] if i + 1 < len(lines) else ""
                # Any '...' line would use a '│   ' prefix from 21's branch;
                # since 21 has no children there should be no such line before
                # the next sibling (which starts with '├──' at the same depth)
                assert "│   └── ..." not in next_line, (
                    "21 should not have a '...' child line"
                )

    def test_depth_2_second_level_values(self):
        """Children of 5 (3, 13, 53) must appear in depth-2 output."""
        text = _capture(3, 2)
        assert "= 3" in text
        assert "= 13" in text
        assert "= 53" in text

    def test_ellipsis_count_depth_1(self):
        """With b=4, d=1 there are '...' lines for:
        nodes 5, 85, 341 (each has children not shown) and the root itself
        (has more children beyond 341), so at least 4 ellipsis lines."""
        text = _capture(4, 1)
        count = text.count("...")
        assert count >= 4

    def test_root_binary_is_rightmost(self):
        """The root '1' in binary should be right-aligned (preceded by spaces)."""
        text = _capture(4, 1)
        first_line = text.split("\n")[0]
        assert first_line.startswith(" "), (
            "Root line should start with spaces for right-alignment"
        )


# ---------------------------------------------------------------------------
# Reference-file test: depth 6, branch 5
# ---------------------------------------------------------------------------

class TestDepth6Branch5File:
    def test_output_matches_reference_file(self):
        """
        Generate the depth-6 branch-5 tree and compare it to the committed
        reference file.  This file is intentionally kept in the repository so
        that maintainers can read it and quickly see what the tree looks like.
        If the output algorithm changes intentionally, regenerate it with:

            python odd_collatz_tree.py -b 5 -d 6 > tests/odd_collatz_d6_b5.txt
        """
        assert os.path.exists(_REFERENCE_FILE), (
            f"Reference file not found: {_REFERENCE_FILE}\n"
            "Generate it with:  python odd_collatz_tree.py -b 5 -d 6 > "
            "tests/odd_collatz_d6_b5.txt"
        )

        generated = _capture(5, 6)

        with open(_REFERENCE_FILE) as f:
            reference = f.read()

        assert generated == reference, (
            "Output does not match reference file.\n"
            "If the change is intentional, update the reference file with:\n"
            "    python odd_collatz_tree.py -b 5 -d 6 > "
            "tests/odd_collatz_d6_b5.txt"
        )

    def test_reference_file_alignment(self):
        """All value lines in the reference file share a single ' = ' column."""
        assert os.path.exists(_REFERENCE_FILE)
        with open(_REFERENCE_FILE) as f:
            text = f.read()
        cols = _eq_cols(text)
        assert len(cols) == 1, f"Expected 1 column, got {cols}"

    def test_reference_file_line_count(self):
        """Sanity-check that the reference file has a non-trivial number of lines."""
        assert os.path.exists(_REFERENCE_FILE)
        with open(_REFERENCE_FILE) as f:
            lines = f.readlines()
        assert len(lines) > 500, f"Expected >500 lines, got {len(lines)}"

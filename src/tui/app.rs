//! Application state and logic for the interactive odd Collatz tree TUI.

use num_bigint::BigUint;
use num_traits::One;

use crate::engine::collatz::{has_odd_collatz_children, odd_collatz_children};

// ---------------------------------------------------------------------------
// Tree data structure
// ---------------------------------------------------------------------------

/// One node in the odd Collatz tree as built for display purposes.
pub struct OddCollatzNode {
    pub value: BigUint,
    pub children: Vec<OddCollatzNode>,
    /// True when more children exist beyond the ones stored in `children`.
    pub has_more_children: bool,
}

/// Build the odd Collatz tree rooted at `n` with limited branching and depth.
pub fn build_tree(n: &BigUint, branching: usize, max_depth: usize, current_depth: usize) -> OddCollatzNode {
    let can_have = has_odd_collatz_children(n);

    if current_depth >= max_depth || !can_have {
        return OddCollatzNode {
            value: n.clone(),
            children: Vec::new(),
            has_more_children: current_depth < max_depth && can_have,
        };
    }

    let children_values: Vec<BigUint> = odd_collatz_children(n.clone()).take(branching).collect();
    let children: Vec<OddCollatzNode> = children_values
        .iter()
        .map(|c| build_tree(c, branching, max_depth, current_depth + 1))
        .collect();

    OddCollatzNode {
        value: n.clone(),
        children,
        has_more_children: true,
    }
}

// ---------------------------------------------------------------------------
// Display lines (flattened tree)
// ---------------------------------------------------------------------------

/// A single rendered line in the tree view.
pub struct DisplayLine {
    pub prefix: String,
    pub connector: String,
    /// The display value for the node (None for ellipsis lines).
    pub value: Option<BigUint>,
    pub is_ellipsis: bool,
    /// The actual node value this line represents (for navigation).
    pub node_value: Option<BigUint>,
}

impl DisplayLine {
    /// Render the full text for this display line.
    pub fn text(&self) -> String {
        if self.is_ellipsis {
            format!("{}{}...", self.prefix, self.connector)
        } else if let Some(ref v) = self.value {
            let bin_str = format!("{:b}", v);
            format!("{}{}{} = {}", self.prefix, self.connector, bin_str, v)
        } else {
            String::new()
        }
    }
}

/// Walk the tree and return a flat list of `DisplayLine`s.
pub fn collect_display_lines(root: &OddCollatzNode) -> Vec<DisplayLine> {
    let mut lines = Vec::new();
    traverse(root, "", true, 0, &mut lines);
    lines
}

fn traverse(
    node: &OddCollatzNode,
    prefix: &str,
    is_last: bool,
    depth: usize,
    lines: &mut Vec<DisplayLine>,
) {
    let (connector, child_prefix): (&str, String) = if depth == 0 {
        ("", String::new())
    } else {
        let conn = if is_last { "└── " } else { "├── " };
        let child_pref = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
        (conn, child_pref)
    };

    lines.push(DisplayLine {
        prefix: prefix.to_string(),
        connector: connector.to_string(),
        value: Some(node.value.clone()),
        is_ellipsis: false,
        node_value: Some(node.value.clone()),
    });

    let n_children = node.children.len();
    for (i, child) in node.children.iter().enumerate() {
        let is_last_child = (i == n_children - 1) && !node.has_more_children;
        traverse(child, &child_prefix, is_last_child, depth + 1, lines);
    }

    if node.has_more_children {
        lines.push(DisplayLine {
            prefix: child_prefix.clone(),
            connector: "└── ".to_string(),
            value: None,
            is_ellipsis: true,
            node_value: None,
        });
    }
}

// ---------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------

/// All state for the interactive TUI application.
pub struct App {
    /// The value at the root of the currently displayed tree.
    pub root_value: BigUint,
    /// Depth limit for tree expansion.
    pub depth: usize,
    /// Maximum children shown per node (branching factor).
    pub branching: usize,
    /// Index of the currently selected display line.
    pub selected_idx: usize,
    /// Flattened display lines for the current tree.
    pub display_lines: Vec<DisplayLine>,
    /// Text in the command input box.
    pub command_input: String,
    /// Informational/error message shown in the status bar.
    pub status_message: String,
    /// Stack of previously visited root values (for back navigation).
    pub navigation_history: Vec<BigUint>,
    /// Set to true when the user requests to quit.
    pub should_quit: bool,
}

impl App {
    pub fn new(depth: usize, branching: usize) -> Self {
        let root_value = BigUint::one();
        let mut app = App {
            root_value,
            depth,
            branching,
            selected_idx: 0,
            display_lines: Vec::new(),
            command_input: String::new(),
            status_message: String::new(),
            navigation_history: Vec::new(),
            should_quit: false,
        };
        app.rebuild_tree();
        app
    }

    /// Rebuild display lines from the current root / depth / branching settings.
    pub fn rebuild_tree(&mut self) {
        let root = build_tree(&self.root_value, self.branching, self.depth, 0);
        self.display_lines = collect_display_lines(&root);
        if self.selected_idx >= self.display_lines.len() {
            self.selected_idx = self.display_lines.len().saturating_sub(1);
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_idx > 0 {
            self.selected_idx -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_idx + 1 < self.display_lines.len() {
            self.selected_idx += 1;
        }
    }

    /// Make the selected node the new root (dive into it).
    pub fn dive_into_selected(&mut self) {
        if let Some(line) = self.display_lines.get(self.selected_idx) {
            if !line.is_ellipsis {
                if let Some(ref val) = line.node_value {
                    if *val != self.root_value {
                        self.navigation_history.push(self.root_value.clone());
                        self.root_value = val.clone();
                        self.selected_idx = 0;
                        self.rebuild_tree();
                    }
                }
            }
        }
    }

    /// Return to the previous root in the navigation history.
    pub fn go_back(&mut self) {
        if let Some(prev) = self.navigation_history.pop() {
            self.root_value = prev;
            self.selected_idx = 0;
            self.rebuild_tree();
        }
    }

    /// Parse and execute the current command_input text.
    pub fn handle_command(&mut self) {
        let cmd = self.command_input.trim().to_lowercase();
        self.command_input.clear();

        let parts: Vec<&str> = cmd.split_whitespace().collect();
        match parts.as_slice() {
            ["q"] | ["quit"] | ["exit"] => {
                self.should_quit = true;
            }
            ["d", n] | ["depth", n] => match n.parse::<usize>() {
                Ok(d) if d > 0 => {
                    self.depth = d;
                    self.rebuild_tree();
                    self.status_message = format!("Depth set to {}", d);
                }
                _ => {
                    self.status_message = "Usage: depth <positive integer>".to_string();
                }
            },
            ["b", n] | ["branching", n] => match n.parse::<usize>() {
                Ok(b) if b > 0 => {
                    self.branching = b;
                    self.rebuild_tree();
                    self.status_message = format!("Branching set to {}", b);
                }
                _ => {
                    self.status_message = "Usage: branching <positive integer>".to_string();
                }
            },
            ["goto", n] | ["g", n] => match n.parse::<u64>() {
                Ok(v) if v > 0 && v % 2 == 1 => {
                    self.navigation_history.push(self.root_value.clone());
                    self.root_value = BigUint::from(v);
                    self.selected_idx = 0;
                    self.rebuild_tree();
                    self.status_message = format!("Navigated to {}", v);
                }
                _ => {
                    self.status_message = "Usage: goto <positive odd integer>".to_string();
                }
            },
            [] => {}
            _ => {
                self.status_message = format!("Unknown command: {}", cmd);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn big(n: u64) -> BigUint {
        BigUint::from(n)
    }

    #[test]
    fn test_initial_root_is_one() {
        let app = App::new(3, 4);
        assert_eq!(app.root_value, big(1));
    }

    #[test]
    fn test_tree_has_display_lines() {
        let app = App::new(3, 4);
        assert!(!app.display_lines.is_empty());
        // Root line should show value 1
        assert_eq!(app.display_lines[0].node_value, Some(big(1)));
    }

    #[test]
    fn test_move_up_at_top_is_noop() {
        let mut app = App::new(3, 4);
        assert_eq!(app.selected_idx, 0);
        app.move_up();
        assert_eq!(app.selected_idx, 0);
    }

    #[test]
    fn test_move_down_and_up() {
        let mut app = App::new(3, 4);
        app.move_down();
        assert_eq!(app.selected_idx, 1);
        app.move_up();
        assert_eq!(app.selected_idx, 0);
    }

    #[test]
    fn test_dive_into_child_and_go_back() {
        let mut app = App::new(3, 4);
        // Move to a child line (not the root itself)
        app.move_down();
        let initial_root = app.root_value.clone();
        // The second display line is a child (if branching >= 1 and depth >= 1)
        if !app.display_lines[1].is_ellipsis {
            app.dive_into_selected();
            assert_ne!(app.root_value, initial_root);
            app.go_back();
            assert_eq!(app.root_value, initial_root);
        }
    }

    #[test]
    fn test_command_depth() {
        let mut app = App::new(3, 4);
        app.command_input = "depth 6".to_string();
        app.handle_command();
        assert_eq!(app.depth, 6);
    }

    #[test]
    fn test_command_branching() {
        let mut app = App::new(3, 4);
        app.command_input = "branching 2".to_string();
        app.handle_command();
        assert_eq!(app.branching, 2);
    }

    #[test]
    fn test_command_quit() {
        let mut app = App::new(3, 4);
        app.command_input = "q".to_string();
        app.handle_command();
        assert!(app.should_quit);
    }

    #[test]
    fn test_command_unknown() {
        let mut app = App::new(3, 4);
        app.command_input = "foobar".to_string();
        app.handle_command();
        assert!(app.status_message.contains("Unknown command"));
    }

    #[test]
    fn test_depth_shorthand() {
        let mut app = App::new(3, 4);
        app.command_input = "d 5".to_string();
        app.handle_command();
        assert_eq!(app.depth, 5);
    }

    #[test]
    fn test_branching_shorthand() {
        let mut app = App::new(3, 4);
        app.command_input = "b 3".to_string();
        app.handle_command();
        assert_eq!(app.branching, 3);
    }

    #[test]
    fn test_goto_command() {
        let mut app = App::new(3, 4);
        app.command_input = "goto 5".to_string();
        app.handle_command();
        assert_eq!(app.root_value, big(5));
        assert_eq!(app.navigation_history, vec![big(1)]);
    }

    #[test]
    fn test_goto_even_rejected() {
        let mut app = App::new(3, 4);
        app.command_input = "goto 4".to_string();
        app.handle_command();
        // Even number should be rejected - root stays at 1
        assert_eq!(app.root_value, big(1));
    }

    #[test]
    fn test_ellipsis_lines_present_when_has_more_children() {
        let app = App::new(3, 1);
        let has_ellipsis = app.display_lines.iter().any(|l| l.is_ellipsis);
        // Root (1) is not a multiple of 3, so it has more children
        assert!(has_ellipsis);
    }
}

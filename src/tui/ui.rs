//! Rendering logic for the interactive TUI.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use super::app::App;

/// Render the full TUI: tree view on top, command panel on the bottom.
pub fn render(app: &App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),    // Main tree view (takes remaining space)
            Constraint::Length(6),  // Command panel (fixed height)
        ])
        .split(frame.area());

    render_tree_view(app, frame, chunks[0]);
    render_command_panel(app, frame, chunks[1]);
}

fn render_tree_view(app: &App, frame: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = app
        .display_lines
        .iter()
        .map(|line| ListItem::new(Text::raw(line.text())))
        .collect();

    let back_hint = if app.navigation_history.is_empty() {
        String::new()
    } else {
        format!(
            "  ← back to {}",
            app.navigation_history.last().unwrap()
        )
    };

    let title = format!(
        " Odd Collatz Tree  root: {}  depth: {}  branching: {}{}",
        app.root_value, app.depth, app.branching, back_hint,
    );

    let list = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    let mut state = ListState::default();
    state.select(Some(app.selected_idx));

    frame.render_stateful_widget(list, area, &mut state);
}

fn render_command_panel(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Status / help line
            Constraint::Length(3), // Command input
        ])
        .split(area);

    // Status / help -------------------------------------------------------
    let status_text = if app.status_message.is_empty() {
        "↑↓ Navigate  Enter/→ Dive in  ←/Esc Back  \
         Commands: depth N | branching N | goto N | q"
            .to_string()
    } else {
        app.status_message.clone()
    };

    let status_widget = Paragraph::new(status_text)
        .block(Block::default().title(" Info ").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(status_widget, chunks[0]);

    // Command input -------------------------------------------------------
    let input_text = format!("> {}", app.command_input);
    let input_widget = Paragraph::new(input_text)
        .block(Block::default().title(" Command ").borders(Borders::ALL));
    frame.render_widget(input_widget, chunks[1]);

    // Position the cursor at the end of the input text
    frame.set_cursor_position((
        chunks[1].x + 2 + app.command_input.len() as u16, // "> " is 2 chars
        chunks[1].y + 1,
    ));
}

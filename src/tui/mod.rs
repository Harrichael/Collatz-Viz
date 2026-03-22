//! Interactive TUI for the odd Collatz tree.
//!
//! # Layout
//!
//! ```text
//! ┌─────────────────────────────────────────────┐
//! │  Odd Collatz Tree  root: 1  depth: 4  …     │
//! │                                             │
//! │  ▶  1 = 1                                   │
//! │     ├── 101 = 5                             │
//! │     │   ├── 11 = 3                          │
//! │     │   └── 1101 = 13                       │
//! │     └── ...                                 │
//! ├─────────────────────────────────────────────┤
//! │ Info: ↑↓ Navigate  Enter/→ Dive  ← Back … │
//! ├─────────────────────────────────────────────┤
//! │ Command: > _                                │
//! └─────────────────────────────────────────────┘
//! ```

mod app;
mod ui;

pub use app::App;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::time::Duration;

/// Launch the interactive TUI.
///
/// Sets up the terminal, runs the event loop, then restores the terminal on
/// exit (whether normal or via panic).
pub fn run(depth: usize, branching: usize) -> io::Result<()> {
    // Setup terminal -------------------------------------------------------
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, depth, branching);

    // Restore terminal -----------------------------------------------------
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    depth: usize,
    branching: usize,
) -> io::Result<()> {
    let mut app = App::new(depth, branching);

    loop {
        terminal.draw(|f| ui::render(&app, f))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        // Typing into the command input
                        KeyCode::Char(c) => {
                            app.command_input.push(c);
                            app.status_message.clear();
                        }
                        KeyCode::Backspace => {
                            app.command_input.pop();
                        }

                        // Submit command (or dive if input is empty)
                        KeyCode::Enter => {
                            if app.command_input.is_empty() {
                                app.dive_into_selected();
                            } else {
                                app.handle_command();
                            }
                        }

                        // Tree navigation
                        KeyCode::Up => app.move_up(),
                        KeyCode::Down => app.move_down(),
                        KeyCode::Right => app.dive_into_selected(),
                        KeyCode::Left => {
                            if app.command_input.is_empty() {
                                app.go_back();
                            } else {
                                app.command_input.clear();
                            }
                        }

                        // Clear input / go back
                        KeyCode::Esc => {
                            if app.command_input.is_empty() {
                                app.go_back();
                            } else {
                                app.command_input.clear();
                            }
                        }

                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

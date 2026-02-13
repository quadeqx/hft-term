// src/lib.rs
pub mod commands;
pub mod helpers;
pub mod structs;

use commands::dispatch;
use std::io;
pub use structs::Pane;
use unicode_segmentation::UnicodeSegmentation;

pub struct TerminalCLI {
    pub commands: std::collections::HashMap<&'static str, structs::Command>,
    pub pane: Pane,
}

impl TerminalCLI {
    pub fn new(commands: std::collections::HashMap<&'static str, structs::Command>) -> Self {
        Self {
            commands,
            pane: Pane {
                buffer: String::new(),
                cursor_grapheme: 0,
                output: None,
            },
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use crate::helpers::{cursor_position_soft_wrapped, grapheme_to_byte_idx};
        use crossterm::{
            event::{self, Event, KeyCode, KeyModifiers},
            execute,
            terminal::{
                EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
            },
        };
        use ratatui::{
            Terminal,
            backend::CrosstermBackend,
            layout::{Constraint, Direction, Layout},
            style::{Color, Style},
            widgets::{Block, Borders, Paragraph, Wrap},
        };
        use unicode_width::UnicodeWidthStr;

        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| {
                let size = f.area();
                let rows = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(100)])
                    .split(size);

                let area = rows[0];
                let prompt = "hft-engine> ";
                let border_style = Style::default().fg(Color::Cyan);

                let mut render = String::new();
                if let Some(out) = &self.pane.output {
                    render.push_str(out);
                    render.push_str("");
                }
                render.push_str(prompt);
                render.push_str(&self.pane.buffer);

                let widget = Paragraph::new(render).wrap(Wrap { trim: false }).block(
                    Block::default()
                        .title("Pane 1")
                        .borders(Borders::ALL)
                        .border_style(border_style),
                );
                f.render_widget(widget, area);

                let prompt_width = UnicodeWidthStr::width(prompt) as u16;
                let usable_width = area.width.saturating_sub(2).saturating_sub(prompt_width);

                let mut cursor_row = 0;
                if let Some(out) = &self.pane.output {
                    for line in out.lines() {
                        let line_width = UnicodeWidthStr::width(line) as u16;
                        let wraps = (line_width / usable_width) as u16;
                        cursor_row += 1 + wraps;
                    }
                }

                let (input_row, input_col) = cursor_position_soft_wrapped(
                    &self.pane.buffer,
                    self.pane.cursor_grapheme,
                    usable_width,
                );
                cursor_row += input_row;
                let cursor_x = area.x + 1 + prompt_width + input_col;
                let cursor_y = area.y + 1 + cursor_row;
                f.set_cursor_position((cursor_x, cursor_y));
            })?;

            if !event::poll(std::time::Duration::from_millis(50))? {
                continue;
            }

            if let Event::Key(key) = event::read()? {
                match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => break,
                    (KeyModifiers::NONE, KeyCode::Char(c)) => {
                        let byte_idx = self
                            .pane
                            .buffer
                            .grapheme_indices(true)
                            .nth(self.pane.cursor_grapheme)
                            .map(|(i, _)| i)
                            .unwrap_or(self.pane.buffer.len());
                        self.pane.buffer.insert(byte_idx, c);
                        self.pane.cursor_grapheme += 1;
                    }
                    (KeyModifiers::CONTROL, KeyCode::Enter) => {
                        let idx =
                            grapheme_to_byte_idx(&self.pane.buffer, self.pane.cursor_grapheme);
                        self.pane.buffer.insert(idx, '\n');
                        self.pane.cursor_grapheme += 1;
                    }
                    (KeyModifiers::NONE, KeyCode::Enter) => {
                        let cmd = self.pane.buffer.trim().to_string();
                        if !cmd.is_empty() {
                            dispatch(&cmd, &self.commands, &mut self.pane);
                            self.pane.buffer.clear();
                        }
                    }
                    (_, KeyCode::Backspace) => {
                        if self.pane.cursor_grapheme > 0 {
                            let start = grapheme_to_byte_idx(
                                &self.pane.buffer,
                                self.pane.cursor_grapheme - 1,
                            );
                            let end =
                                grapheme_to_byte_idx(&self.pane.buffer, self.pane.cursor_grapheme);
                            self.pane.buffer.replace_range(start..end, "");
                            self.pane.cursor_grapheme -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}

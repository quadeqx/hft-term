//! src/strucrs.rs
use std::collections::HashMap;

pub struct Pane {
    pub buffer: String,
    pub cursor_grapheme: usize,
    pub output: Option<String>,
}

// Alias
pub type CommandFn = fn(&[&str], &HashMap<&'static str, Command>, &mut Pane);

pub struct Command {
    pub run: Option<CommandFn>,
    pub help: &'static str,
    pub subcommands: HashMap<&'static str, Command>,
}

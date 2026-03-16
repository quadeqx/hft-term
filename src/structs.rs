//! src/strucrs.rs
use std::collections::HashMap;

#[derive(Default)]
pub struct Pane {
    pub buffer: String,
    pub cursor_grapheme: usize,
    pub output: Option<String>,
}

// Alias
// pub type CommandFn = fn(&[&str]) -> Option<String>;

pub struct Command {
    pub run: Option<fn(&[&str]) -> Option<String>>,
    pub help: &'static str,
    pub subcommands: HashMap<&'static str, Command>,
}

pub trait Runnable: Send + Sync {
    fn run(&self, args: &[&str]) -> Option<String>;
    fn help(&self) -> &str;
}

impl Runnable for Command {
    fn run(&self, args: &[&str]) -> Option<String> {
        if let Some(f) = self.run {
            f(args)
        } else {
            Some(format!("{} (no handler)", self.help))
        }
    }

    fn help(&self) -> &str {
        self.help
    }
}

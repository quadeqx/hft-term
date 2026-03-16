//! src/commands.rs
use crate::{
    Runnable,
    structs::{Command, Pane},
};
use std::collections::HashMap;

// Trait implemented for genericity
//pub trait Runnable {
//    fn run(&self, args: &[&str]) -> Option<String>;
//    fn help(&self) -> &str;
//    fn subcommands(&self) -> &HashMap<&'static str, Self>
//    where
//        Self: Sized;
//}

// Dispatch the commands
pub fn dispatch(input: &str, commands: &HashMap<&'static str, Box<dyn Runnable>>, pane: &mut Pane) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    dispatch_recursive(&parts, commands, pane);
}

fn dispatch_recursive(
    args: &[&str],
    commands: &HashMap<&'static str, Box<dyn Runnable>>,
    pane: &mut Pane,
) {
    let name = args[0];

    match commands.get(name) {
        Some(cmd) => {
            let remaining = &args[1..];

            // If next token matches a subcommand, recurse...
            // To be done

            // Execute command if runnable
            if let Some(output) = cmd.run(remaining) {
                pane.output = Some(output);
            } else {
                pane.output = Some(format!("Incomplete command: {}\n", name));
            }
        }
        None => {
            pane.output = Some(format!("Unknown command: {}\n", name));
        }
    }
}

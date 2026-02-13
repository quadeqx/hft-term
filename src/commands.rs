//! src/commands.rs
use crate::structs::{Command, Pane};
use std::collections::HashMap;

// Dispatch the commands
pub fn dispatch(input: &str, commands: &HashMap<&'static str, Command>, pane: &mut Pane) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    dispatch_recursive(&parts, commands, pane);
}

fn dispatch_recursive(parts: &[&str], commands: &HashMap<&'static str, Command>, pane: &mut Pane) {
    let name = parts[0];

    match commands.get(name) {
        Some(cmd) => {
            let remaining = &parts[1..];

            // If next token matches a subcommand, recurse
            if let Some(next) = remaining.first()
                && cmd.subcommands.contains_key(next)
            {
                dispatch_recursive(remaining, &cmd.subcommands, pane);
                return;
            }

            // Execute command if runnable
            if let Some(run) = cmd.run {
                run(remaining, commands, pane);
            } else {
                pane.output = Some(format!("Incomplete command: {}\n", name));
            }
        }
        None => {
            pane.output = Some(format!("Unknown command: {}\n", name));
        }
    }
}

//! examples/usage.rs

use hft_term::structs::Pane;
use hft_term::{TerminalCLI, structs::Command};
use std::collections::HashMap;

pub fn hello(_args: &[&str], _commands: &HashMap<&'static str, Command>, pane: &mut Pane) {
    pane.output = Some("Welcome buddy!\n".to_string());
}

pub fn cmd_help(args: &[&str], commands: &HashMap<&'static str, Command>, pane: &mut Pane) {
    if args.is_empty() {
        // Top-level help
        let mut out = String::from("Available commands:\n");

        for (name, cmd) in commands {
            out.push_str(&format!("  {:<10} - {}\n", name, cmd.help));
        }

        pane.output = Some(out);
        return;
    }

    // Traverse command tree
    let mut current_commands = commands;
    let mut current_cmd: Option<&Command> = None;

    for part in args {
        match current_commands.get(part) {
            Some(cmd) => {
                current_cmd = Some(cmd);
                current_commands = &cmd.subcommands;
            }
            None => {
                pane.output = Some(format!("No such command: {:?}\n", args));
                return;
            }
        }
    }

    if let Some(cmd) = current_cmd {
        let mut out = format!("{}\n", cmd.help);

        if !cmd.subcommands.is_empty() {
            out.push_str("\nSubcommands:\n");
            for (name, sub) in &cmd.subcommands {
                out.push_str(&format!("  {:<10} - {}\n", name, sub.help));
            }
        }
        pane.output = Some(out)
    }
}

pub fn cmd_hello_you(_args: &[&str], _commands: &HashMap<&'static str, Command>, pane: &mut Pane) {
    pane.output = Some("hello you".to_string());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // User defines their own commands
    let mut commands: HashMap<&'static str, Command> = HashMap::new();

    commands.insert(
        "hello",
        Command {
            run: Some(hello),
            help: "Welcome!!",
            subcommands: HashMap::new(),
        },
    );

    commands.insert(
        "help",
        Command {
            run: Some(cmd_help),
            help: "Available commands",
            subcommands: HashMap::new(),
        },
    );

    let mut cli = TerminalCLI::new(commands);
    cli.run()
}

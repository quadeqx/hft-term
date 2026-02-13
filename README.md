# hft-term

**Rust-based interactive terminal framework for HFT engines and custom command-line tools.**

`hft-term` provides a lightweight terminal interface built with **crossterm** and **ratatui**, suited for high-frequency trading (HFT) tooling, REPLs, and other interactive text-based applications. It handles input, rendering, and command dispatching with minimal overhead.

---

## 📦 Features

* Interactive terminal UI with full keyboard support
* Command buffer with soft-wrapped input & cursor movement
* Output pane and prompt rendering
* Designed to support custom HFT engines or trading CLIs
* Built with low-latency Rust crates (`tokio`, `crossterm`, `ratatui`)

---

## 🚀 Getting Started

### Requirements

You need Rust (1.70+ recommended) installed.

```sh
rustup toolchain install stable
```

Clone the repo:

```sh
git clone https://github.com/quadeqx/hft-term.git
cd hft-term
```

### Build

```sh
cargo build --release
```

### Run Example CLI

You can run the example in `examples/`:

```sh
cargo run --example usage
```


---

## 🧠 How It Works

At its core, `hft-term` exposes a `TerminalCLI` type you can use to:

1. Provide a set of commands.
2. Take in user input interactively.
3. Render output in a custom pane.
4. Dispatch commands to your logic.

It uses:

* **crossterm** — for cross-platform terminal input/output & events
* **ratatui** — for text UI rendering
* **tokio** — async support if needed

---

## 🧩 Example Usage

Here’s a basic example showing how you might set up a CLI:

```rust
use hft_term::TerminalCLI;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let mut commands = HashMap::new();
    // register your commands
    // commands.insert("hello", ...);

    let mut cli = TerminalCLI::new(commands);
    cli.run()?;

    Ok(())
}
```

Customize your receiver logic in the command dispatch modules.

---

## 📁 Project Structure

```
├── examples/          # sample usage code
├── src/
│   ├── commands       # command parsing & dispatch
│   ├── helpers        # layout helpers
│   ├── structs        # pane and buffer structs
│   └── lib.rs         # core CLI runner
├── Cargo.toml         # Rust manifest
└── .gitignore
```

---

## 🛠 Dependencies

* **crossterm** — terminal interaction
* **ratatui** — TUI rendering
* **unicode-segmentation** & **unicode-width** — grapheme & width handling
* **tokio** — async runtime

Full dependencies available in `Cargo.toml`. ([GitHub][1])

---

## 🎯 Use Cases

* Build HFT command consoles
* Custom REPL interfaces
* Trading engine dashboards in terminal
* Interactive bots or trading tools

---

## 🤝 Contributing

Contributions are welcome:

1. Fork the repository
2. Create a feature branch
3. Submit a PR

Include tests & update this README when adding new features.

---





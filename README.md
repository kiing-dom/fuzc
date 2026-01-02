# fuzc

A fuzzy finder for code comments

## Features

- **TUI Mode**: Interactive terminal interface for browsing and searching comments
- **CLI Mode**: Quick command-line searches for automation
- **Multi-language Support**: Java, Python, JavaScript, and TypeScript
- **Search Modes**: AND/OR search modes (toggle with Tab in TUI)
- **Simple Scoring**: Results ranked by relevance

## Installation

Clone and build from source:

```bash
git clone https://github.com/kiing-dom/fuzc.git
cd fuzc
cargo build --release
```

The binary will be available at `target/release/fuzc`. 

Note: Global installation (like `fzf`) where you can simply run `fuzc` from any directory is planned for future releases.

## Usage

### TUI Mode (default)

```bash
cargo run
```

**Keyboard Shortcuts:**
- Type to search
- `↑/↓` - Navigate results
- `Tab` - Toggle strict mode (AND/OR search)
- `Esc` or `Ctrl+C` - Quit

### CLI Mode

```bash
cargo run -- --cli --query "TODO" --directory ./src
cargo run -- --cli --query "bug fix" --strict
```

## Current Limitations

This project is in early development and is missing significant functionality. It was built without AI assistance as a learning exercise, so expect rough edges and incomplete features.

## License

MIT License - see [LICENSE](LICENSE) for details.
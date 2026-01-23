# Installation Guide

## Quick Install (Recommended)

```bash
cd ~/code/brui
cargo install --path .
```

This will install `brui` to `~/.cargo/bin/brui` (make sure `~/.cargo/bin` is in your PATH).

## Development Build

```bash
cd ~/code/brui
cargo build --release
```

The binary will be at `target/release/brui`.

## Adding to PATH

If `~/.cargo/bin` is not in your PATH, add it to your shell profile:

### Bash
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Zsh
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

## Testing

Navigate to a beads project and run:

```bash
cd ~/code/ralphy  # or any beads project
brui
```

## Keyboard Shortcuts

### Board View
- `←/→` or `h/l` - Navigate columns
- `↑/↓` or `k/j` - Select issue
- `Enter` - View details
- `r` - Refresh
- `q` - Quit

### Detail View
- `Esc` or `q` - Back to board

## Troubleshooting

### "Not in a beads project" error
Make sure you're in a directory that contains a `.beads/` folder.

### Database not found
Ensure `beads.db` exists in the `.beads/` directory.

### Permission denied
Check that the database file is readable:
```bash
ls -la .beads/beads.db
```

### Build errors
Make sure you have Rust installed:
```bash
rustc --version
cargo --version
```

If not, install from https://rustup.rs/

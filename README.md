# brui - Real-Time Beads Kanban Board TUI

A beautiful, interactive terminal UI for viewing your Beads issues in a Kanban board layout with stunning Material Design colors.

## Features

- **Beautiful Material Design**: Cohesive color palette with excellent contrast and accessibility
- **Kanban Board View**: See issues organized by status (Open, In Progress, Done)
- **Interactive Navigation**: Arrow keys or vim bindings (h/j/k/l) to navigate
- **Detailed Task View**: Press Enter to see full task details
- **Real-time Updates**: Automatically refreshes when beads data changes
- **Label Filtering**: Filter by label (defaults to "ralph")
- **Priority Highlighting**: Color-coded priorities (P0-P4)
- **Dependency Indicators**: Visual indicators for blocked issues

## Installation

### From Source

```bash
cd ~/code/brui
cargo build --release
cargo install --path .
```

### Development

```bash
cargo run -- --help
```

## Usage

```bash
# Show issues with label "ralph" (default)
brui

# Show all issues
brui --all

# Filter by specific label
brui --label critical

# Disable real-time watching
brui --no-watch
```

## Keyboard Shortcuts

### Board View
- `←/→` or `h/l` - Navigate between columns
- `↑/↓` or `k/j` - Select issue within column
- `Enter` - View issue details
- `r` - Manual refresh
- `q` - Quit

### Detail View
- `Esc` or `q` - Back to board

## Design System

BRUI uses Material Design 3 color palettes for a beautiful, cohesive experience:

- **Status Colors**: Blue (Open), Amber (In Progress), Green (Done)
- **Priority Colors**: Red (P0-Critical), Orange (P1-High), Light Blue (P2-Medium), Gray (P3-Low), Blue-Gray (P4-Lowest)
- **Semantic Colors**: Darker red for blocked items, distinct orange for blocking relationships
- **UI Elements**: Cyan accents, blue-gray backgrounds, optimized contrast for readability

All colors are carefully selected for excellent terminal compatibility and accessibility.

## Architecture

```
brui/
├── src/
│   ├── beads/          # Beads CLI wrapper
│   │   ├── models.rs   # Data structures
│   │   ├── db.rs       # bd CLI client
│   │   └── mod.rs
│   ├── ui/             # TUI layer
│   │   ├── app.rs      # Application state & event handling
│   │   ├── board.rs    # Kanban board view
│   │   ├── detail.rs   # Task detail view
│   │   └── mod.rs
│   ├── watcher/        # File watching
│   │   └── mod.rs
│   └── main.rs         # Entry point
└── Cargo.toml
```

## Dependencies

- **ratatui**: TUI framework
- **crossterm**: Terminal manipulation
- **notify**: File system watcher
- **bd CLI**: Beads CLI for data access (Dolt backend)

## License

MIT

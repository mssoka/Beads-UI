mod beads;
mod ui;
mod watcher;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::Event,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use beads::BeadsDb;
use ui::{App, View};
use watcher::FileWatcher;

#[derive(Parser, Debug)]
#[command(name = "brui")]
#[command(about = "Real-Time Beads Kanban Board TUI", long_about = None)]
#[command(version)]
struct Cli {
    /// Filter by label (default: ralph)
    #[arg(short, long, default_value = "ralph")]
    label: Option<String>,

    /// Show all issues (no label filter)
    #[arg(short, long)]
    all: bool,

    /// Disable real-time file watching
    #[arg(long)]
    no_watch: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Find and open beads database
    let beads_dir = BeadsDb::find_beads_dir()?;
    let db = BeadsDb::new(&beads_dir)?;

    // Set up label filter
    let label_filter = if cli.all {
        None
    } else {
        cli.label
    };

    // Set up file watcher
    let watcher = if cli.no_watch {
        None
    } else {
        Some(FileWatcher::new(db.db_path())?)
    };

    // Create app
    let mut app = App::new(db, label_filter)?;

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run_app(&mut terminal, &mut app, watcher);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    watcher: Option<FileWatcher>,
) -> Result<()> {
    loop {
        // Draw UI and capture scroll info from detail view
        let mut new_scroll_max: u16 = 0;
        let mut new_viewport_height: u16 = 0;

        terminal.draw(|f| match app.current_view {
            View::Board => ui::render_board(f, app),
            View::Detail => {
                let (sm, vh) = ui::render_detail(f, app);
                new_scroll_max = sm;
                new_viewport_height = vh;
            }
            View::Search => ui::render_search(f, app),
        })?;

        // Update scroll state after render
        app.detail_scroll_max = new_scroll_max;
        app.detail_viewport_height = new_viewport_height;
        app.detail_scroll = app.detail_scroll.min(app.detail_scroll_max);

        // Check for file changes
        if let Some(ref w) = watcher {
            if w.poll().is_some() {
                app.reload_issues()?;
            }
        }

        // Handle events
        if let Some(Event::Key(key)) = app.poll_event()? {
            app.handle_key(key)?;
        }

        // Check if should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

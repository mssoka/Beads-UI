use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::time::Duration;

use crate::beads::{BeadsDb, Issue, Status};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Board,
    Detail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Column {
    Open,
    InProgress,
    Done,
}

impl Column {
    pub fn next(&self) -> Self {
        match self {
            Column::Open => Column::InProgress,
            Column::InProgress => Column::Done,
            Column::Done => Column::Open,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Column::Open => Column::Done,
            Column::InProgress => Column::Open,
            Column::Done => Column::InProgress,
        }
    }

    pub fn status(&self) -> Status {
        match self {
            Column::Open => Status::Open,
            Column::InProgress => Status::InProgress,
            Column::Done => Status::Closed,
        }
    }
}

pub struct App {
    pub db: BeadsDb,
    pub label_filter: Option<String>,
    pub issues: Vec<Issue>,
    pub current_view: View,
    pub selected_column: Column,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new(db: BeadsDb, label_filter: Option<String>) -> Result<Self> {
        let mut app = App {
            db,
            label_filter,
            issues: Vec::new(),
            current_view: View::Board,
            selected_column: Column::Open,
            selected_index: 0,
            scroll_offset: 0,
            should_quit: false,
        };
        app.reload_issues()?;
        Ok(app)
    }

    pub fn reload_issues(&mut self) -> Result<()> {
        self.issues = self.db.load_issues(self.label_filter.as_deref())?;
        self.clamp_selection();
        Ok(())
    }

    pub fn get_column_issues(&self, column: Column) -> Vec<&Issue> {
        let status = column.status();
        self.issues
            .iter()
            .filter(|i| i.status == status)
            .collect()
    }

    pub fn selected_issue(&self) -> Option<&Issue> {
        let issues = self.get_column_issues(self.selected_column);
        issues.get(self.selected_index).copied()
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        match self.current_view {
            View::Board => self.handle_board_key(key),
            View::Detail => self.handle_detail_key(key),
        }
    }

    fn handle_board_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('r') => {
                self.reload_issues()?;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.selected_column = self.selected_column.prev();
                self.selected_index = 0;
                self.scroll_offset = 0;
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.selected_column = self.selected_column.next();
                self.selected_index = 0;
                self.scroll_offset = 0;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let count = self.get_column_issues(self.selected_column).len();
                if count > 0 && self.selected_index < count - 1 {
                    self.selected_index += 1;
                }
            }
            KeyCode::Enter => {
                if self.selected_issue().is_some() {
                    self.current_view = View::Detail;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_detail_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.current_view = View::Board;
            }
            _ => {}
        }
        Ok(())
    }

    fn clamp_selection(&mut self) {
        let count = self.get_column_issues(self.selected_column).len();
        if count == 0 {
            self.selected_index = 0;
        } else if self.selected_index >= count {
            self.selected_index = count - 1;
        }
    }

    pub fn poll_event(&self) -> Result<Option<Event>> {
        if event::poll(Duration::from_millis(100))? {
            Ok(Some(event::read()?))
        } else {
            Ok(None)
        }
    }
}

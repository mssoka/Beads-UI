use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::time::Duration;

use crate::beads::{BeadsDb, Issue, Status};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Board,
    Detail,
    Search,
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

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub issue_id: String,
    pub score: i64,
    pub title_match_indices: Vec<usize>,
}

pub struct App {
    pub db: BeadsDb,
    pub label_filter: Option<String>,
    pub issues: Vec<Issue>,
    pub current_view: View,
    pub selected_column: Column,
    pub selected_index: usize,
    pub should_quit: bool,
    // Detail scrolling
    pub detail_scroll: u16,
    pub detail_scroll_max: u16,
    pub detail_viewport_height: u16,
    // Search
    pub search_query: String,
    pub search_results: Vec<SearchResult>,
    pub search_selected: usize,
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
            should_quit: false,
            detail_scroll: 0,
            detail_scroll_max: 0,
            detail_viewport_height: 0,
            search_query: String::new(),
            search_results: Vec::new(),
            search_selected: 0,
        };
        app.reload_issues()?;
        Ok(app)
    }

    pub fn reload_issues(&mut self) -> Result<()> {
        self.issues = self.db.load_issues(self.label_filter.as_deref())?;
        self.clamp_selection();
        // Clamp detail scroll in case content changed
        self.detail_scroll = self.detail_scroll.min(self.detail_scroll_max);
        // Refresh search results if in search view
        if self.current_view == View::Search {
            self.update_search_results();
        }
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
            View::Search => self.handle_search_key(key),
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
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.selected_column = self.selected_column.next();
                self.selected_index = 0;
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
                    self.detail_scroll = 0;
                    self.current_view = View::Detail;
                }
            }
            KeyCode::Char('/') => {
                self.search_query.clear();
                self.search_selected = 0;
                self.update_search_results();
                self.current_view = View::Search;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_detail_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.detail_scroll = 0;
                self.current_view = View::Board;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.detail_scroll < self.detail_scroll_max {
                    self.detail_scroll += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.detail_scroll = self.detail_scroll.saturating_sub(1);
            }
            KeyCode::Char('g') | KeyCode::Home => {
                self.detail_scroll = 0;
            }
            KeyCode::Char('G') | KeyCode::End => {
                self.detail_scroll = self.detail_scroll_max;
            }
            KeyCode::PageDown => {
                let jump = self.detail_viewport_height.max(1);
                self.detail_scroll = (self.detail_scroll + jump).min(self.detail_scroll_max);
            }
            KeyCode::PageUp => {
                let jump = self.detail_viewport_height.max(1);
                self.detail_scroll = self.detail_scroll.saturating_sub(jump);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_search_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.current_view = View::Board;
            }
            KeyCode::Enter => {
                if let Some(result) = self.search_results.get(self.search_selected) {
                    let issue_id = result.issue_id.clone();
                    // Find the issue and navigate to its position on the board
                    if let Some(issue) = self.issues.iter().find(|i| i.id == issue_id) {
                        let status = issue.status;
                        let col = match status {
                            Status::Open => Column::Open,
                            Status::InProgress => Column::InProgress,
                            Status::Closed => Column::Done,
                        };
                        // Find index within that column
                        let col_issues = self.get_column_issues(col);
                        let idx = col_issues
                            .iter()
                            .position(|i| i.id == issue_id)
                            .unwrap_or(0);
                        self.selected_column = col;
                        self.selected_index = idx;
                        self.detail_scroll = 0;
                        self.current_view = View::Detail;
                    }
                }
            }
            KeyCode::Up => {
                if self.search_selected > 0 {
                    self.search_selected -= 1;
                }
            }
            KeyCode::Down => {
                if !self.search_results.is_empty()
                    && self.search_selected < self.search_results.len() - 1
                {
                    self.search_selected += 1;
                }
            }
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.search_query.clear();
                self.search_selected = 0;
                self.update_search_results();
            }
            KeyCode::Backspace => {
                self.search_query.pop();
                self.search_selected = 0;
                self.update_search_results();
            }
            KeyCode::Char(c) => {
                self.search_query.push(c);
                self.search_selected = 0;
                self.update_search_results();
            }
            _ => {}
        }
        Ok(())
    }

    pub fn update_search_results(&mut self) {
        let matcher = SkimMatcherV2::default();

        if self.search_query.is_empty() {
            // Show all issues sorted by priority then updated_at
            self.search_results = self
                .issues
                .iter()
                .map(|issue| SearchResult {
                    issue_id: issue.id.clone(),
                    score: 0,
                    title_match_indices: Vec::new(),
                })
                .collect();
        } else {
            let mut results: Vec<SearchResult> = self
                .issues
                .iter()
                .filter_map(|issue| {
                    let haystack = format!(
                        "{} {} {} {} {}",
                        issue.id,
                        issue.title,
                        issue.description.as_deref().unwrap_or(""),
                        issue.assignee.as_deref().unwrap_or(""),
                        issue.labels.join(" ")
                    );
                    let score = matcher.fuzzy_match(&haystack, &self.search_query)?;
                    // Get title-specific match indices for highlighting
                    let title_match_indices = matcher
                        .fuzzy_indices(&issue.title, &self.search_query)
                        .map(|(_, indices)| indices)
                        .unwrap_or_default();
                    Some(SearchResult {
                        issue_id: issue.id.clone(),
                        score,
                        title_match_indices,
                    })
                })
                .collect();
            results.sort_by(|a, b| {
                b.score.cmp(&a.score).then_with(|| {
                    // Tiebreak by priority (lower = higher priority)
                    let a_issue = self.issues.iter().find(|i| i.id == a.issue_id);
                    let b_issue = self.issues.iter().find(|i| i.id == b.issue_id);
                    let a_pri = a_issue.map(|i| i.priority.0).unwrap_or(255);
                    let b_pri = b_issue.map(|i| i.priority.0).unwrap_or(255);
                    a_pri.cmp(&b_pri).then_with(|| a.issue_id.cmp(&b.issue_id))
                })
            });
            self.search_results = results;
        }
        // Clamp selection
        if self.search_results.is_empty() {
            self.search_selected = 0;
        } else if self.search_selected >= self.search_results.len() {
            self.search_selected = self.search_results.len() - 1;
        }
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

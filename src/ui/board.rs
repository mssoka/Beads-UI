use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::app::{App, Column};
use super::theme::*;
use crate::beads::Issue;

pub fn render_board(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Board
            Constraint::Length(1), // Footer
        ])
        .split(f.area());

    render_header(f, chunks[0], app);
    render_columns(f, chunks[1], app);
    render_footer(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let label = app.label_filter
        .as_ref()
        .map(|l| format!("🏷  label: {}", l))
        .unwrap_or_else(|| "📋 all issues".to_string());

    // Create multi-span line with visual separators
    let header_line = Line::from(vec![
        Span::styled(" ▓▓ ", Style::default()
            .fg(COLOR_HEADER)
            .bg(COLOR_HEADER_BG)
            .add_modifier(Modifier::BOLD)),
        Span::styled("BRUI", Style::default()
            .fg(COLOR_HEADER)
            .add_modifier(Modifier::BOLD)),
        Span::styled("  │  ", Style::default().fg(COLOR_SEPARATOR)),
        Span::raw("Beads Kanban"),
        Span::styled("  │  ", Style::default().fg(COLOR_SEPARATOR)),
        Span::styled(&label, Style::default().fg(COLOR_IN_PROGRESS)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)  // Use rounded corners
        .style(Style::default().fg(COLOR_HEADER));

    let paragraph = Paragraph::new(header_line)
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

fn render_columns(f: &mut Frame, area: Rect, app: &App) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

    render_column(f, columns[0], app, Column::Open);
    render_column(f, columns[1], app, Column::InProgress);
    render_column(f, columns[2], app, Column::Done);
}

fn render_column(f: &mut Frame, area: Rect, app: &App, column: Column) {
    let issues = app.get_column_issues(column);
    let is_selected = app.selected_column == column;

    let (title, color) = match column {
        Column::Open => (format!("OPEN ({})", issues.len()), COLOR_OPEN),
        Column::InProgress => (format!("IN PROGRESS ({})", issues.len()), COLOR_IN_PROGRESS),
        Column::Done => (format!("DONE ({})", issues.len()), COLOR_DONE),
    };

    let border_style = if is_selected {
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(COLOR_BORDER)
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(border_style);

    let items: Vec<ListItem> = issues
        .iter()
        .enumerate()
        .map(|(idx, issue)| {
            let is_item_selected = is_selected && idx == app.selected_index;
            format_issue_item(issue, is_item_selected)
        })
        .collect();

    let list = List::new(items).block(block);

    f.render_widget(list, area);
}

fn format_issue_item(issue: &Issue, is_selected: bool) -> ListItem<'_> {
    let priority_clr = priority_color(issue.priority.0);

    let mut spans = vec![
        Span::styled(
            format!("{} ", issue.priority.label()),
            Style::default().fg(priority_clr).add_modifier(Modifier::BOLD),
        ),
        Span::styled(&issue.id, Style::default().fg(COLOR_SECONDARY_TEXT)),
        Span::raw(" "),
        Span::raw(&issue.title),
    ];

    if issue.is_blocked() {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            "🚫",
            Style::default().fg(COLOR_BLOCKED),
        ));
    }

    let style = if is_selected {
        Style::default()
            .bg(COLOR_SELECTED_BG)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    ListItem::new(Line::from(spans)).style(style)
}

fn render_footer(f: &mut Frame, area: Rect) {
    let help = "[←/→ or h/l] Navigate  [↑/↓ or k/j] Select  [Enter] Details  [/] Search  [r] Refresh  [q] Quit";
    let paragraph = Paragraph::new(help).style(Style::default().fg(COLOR_HELP_TEXT));
    f.render_widget(paragraph, area);
}

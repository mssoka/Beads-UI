use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::app::App;
use super::theme::*;

pub fn render_search(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search input
            Constraint::Min(1),    // Results list
            Constraint::Length(1), // Footer
        ])
        .split(f.area());

    render_search_input(f, chunks[0], app);
    render_search_results(f, chunks[1], app);
    render_search_footer(f, chunks[2]);
}

fn render_search_input(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Search ")
        .style(Style::default().fg(COLOR_SEARCH_BORDER));

    let input_text = format!("/ {}_", app.search_query);
    let paragraph = Paragraph::new(input_text)
        .block(block)
        .style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(paragraph, area);
}

fn render_search_results(f: &mut Frame, area: Rect, app: &App) {
    let title = format!(" Results ({}) ", app.search_results.len());

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default());

    let items: Vec<ListItem> = app
        .search_results
        .iter()
        .enumerate()
        .map(|(idx, result)| {
            let is_selected = idx == app.search_selected;
            let issue = app.issues.iter().find(|i| i.id == result.issue_id);

            if let Some(issue) = issue {
                let priority_clr = priority_color(issue.priority.0);

                let mut spans: Vec<Span> = vec![
                    Span::styled(
                        format!("{} ", issue.priority.label()),
                        Style::default()
                            .fg(priority_clr)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{} ", issue.id),
                        Style::default().fg(COLOR_SECONDARY_TEXT),
                    ),
                ];

                // Build title with match highlighting
                let title_chars: Vec<char> = issue.title.chars().collect();
                let mut i = 0;
                while i < title_chars.len() {
                    if result.title_match_indices.contains(&i) {
                        // Collect consecutive matched chars
                        let start = i;
                        while i < title_chars.len() && result.title_match_indices.contains(&i) {
                            i += 1;
                        }
                        let matched: String = title_chars[start..i].iter().collect();
                        spans.push(Span::styled(
                            matched,
                            Style::default()
                                .fg(COLOR_SEARCH_MATCH)
                                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                        ));
                    } else {
                        // Collect consecutive non-matched chars
                        let start = i;
                        while i < title_chars.len() && !result.title_match_indices.contains(&i) {
                            i += 1;
                        }
                        let unmatched: String = title_chars[start..i].iter().collect();
                        spans.push(Span::raw(unmatched));
                    }
                }

                // Labels
                if !issue.labels.is_empty() {
                    spans.push(Span::styled(
                        format!(" [{}]", issue.labels.join(", ")),
                        Style::default().fg(COLOR_SECONDARY_TEXT),
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
            } else {
                // Issue was removed — show stale ID
                let style = if is_selected {
                    Style::default()
                        .bg(COLOR_SELECTED_BG)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(COLOR_SECONDARY_TEXT)
                };
                ListItem::new(format!("  {} (not found)", result.issue_id)).style(style)
            }
        })
        .collect();

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

fn render_search_footer(f: &mut Frame, area: Rect) {
    let help = "[Type] Search  [Up/Down] Navigate  [Enter] View  [Esc] Back  [Ctrl+U] Clear";
    let paragraph = Paragraph::new(help).style(Style::default().fg(COLOR_HELP_TEXT));
    f.render_widget(paragraph, area);
}

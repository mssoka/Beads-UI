use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::app::App;
use crate::beads::Issue;

pub fn render_detail(f: &mut Frame, app: &App) {
    if let Some(issue) = app.selected_issue() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Details
                Constraint::Length(1), // Footer
            ])
            .split(f.area());

        render_detail_header(f, chunks[0], issue);
        render_detail_body(f, chunks[1], issue);
        render_detail_footer(f, chunks[2]);
    }
}

fn render_detail_header(f: &mut Frame, area: Rect, issue: &Issue) {
    let title = format!(" {} - {} ", issue.id, issue.title);

    let priority_color = match issue.priority.0 {
        0 => Color::Red,
        1 => Color::Yellow,
        2 => Color::Blue,
        _ => Color::Gray,
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(priority_color));

    let paragraph = Paragraph::new(title)
        .block(block)
        .style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(paragraph, area);
}

fn render_detail_body(f: &mut Frame, area: Rect, issue: &Issue) {
    let mut lines = vec![];

    // Status and Priority
    lines.push(Line::from(vec![
        Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(format!("{:?}", issue.status)),
    ]));

    lines.push(Line::from(vec![
        Span::styled("Priority: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(format!("{} ({})", issue.priority.label(), issue.priority.0)),
    ]));

    lines.push(Line::from(vec![
        Span::styled("Type: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(format!("{}", issue.issue_type)),
    ]));

    lines.push(Line::raw(""));

    // Assignee
    if let Some(ref assignee) = issue.assignee {
        lines.push(Line::from(vec![
            Span::styled("Assignee: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(assignee),
        ]));
    }

    // Labels
    if !issue.labels.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("Labels: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(issue.labels.join(", ")),
        ]));
    }

    lines.push(Line::raw(""));

    // Description
    if let Some(ref desc) = issue.description {
        lines.push(Line::from(Span::styled(
            "Description:",
            Style::default().add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::raw(desc.clone()));
        lines.push(Line::raw(""));
    }

    // Dependencies
    if !issue.blocked_by.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("Blocked by: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(issue.blocked_by.join(", ")),
        ]));
    }

    if !issue.blocks.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("Blocks: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(issue.blocks.join(", ")),
        ]));
    }

    lines.push(Line::raw(""));

    // Timestamps
    lines.push(Line::from(vec![
        Span::styled("Created: ", Style::default().fg(Color::DarkGray)),
        Span::styled(&issue.created_at, Style::default().fg(Color::DarkGray)),
    ]));

    lines.push(Line::from(vec![
        Span::styled("Updated: ", Style::default().fg(Color::DarkGray)),
        Span::styled(&issue.updated_at, Style::default().fg(Color::DarkGray)),
    ]));

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Details ")
        .style(Style::default());

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn render_detail_footer(f: &mut Frame, area: Rect) {
    let help = "[Esc or q] Back to board";
    let paragraph = Paragraph::new(help).style(Style::default().fg(Color::DarkGray));
    f.render_widget(paragraph, area);
}

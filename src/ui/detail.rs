use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
    },
    Frame,
};
use tui_markdown::from_str as markdown_from_str;

use super::app::App;
use super::theme::*;
use crate::beads::Issue;

/// Renders the detail view. Returns (scroll_max, viewport_height) for the description area.
pub fn render_detail(f: &mut Frame, app: &App) -> (u16, u16) {
    if let Some(issue) = app.selected_issue() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(1),    // Metadata
                Constraint::Min(3),    // Description
                Constraint::Length(1), // Footer
            ])
            .split(f.area());

        render_detail_header(f, chunks[0], issue);
        render_detail_metadata(f, chunks[1], issue);
        let (scroll_max, viewport_height) =
            render_detail_description(f, chunks[2], issue, app.detail_scroll);
        render_detail_footer(f, chunks[3], app.detail_scroll, scroll_max);
        (scroll_max, viewport_height)
    } else {
        (0, 0)
    }
}

fn render_detail_header(f: &mut Frame, area: Rect, issue: &Issue) {
    let title = format!(" {} - {} ", issue.id, issue.title);

    let priority_clr = priority_color(issue.priority.0);

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(priority_clr));

    let paragraph = Paragraph::new(title)
        .block(block)
        .style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(paragraph, area);
}

fn render_detail_metadata(f: &mut Frame, area: Rect, issue: &Issue) {
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

    // Dependencies
    if !issue.blocked_by.is_empty() {
        lines.push(Line::from(vec![
            Span::styled(
                "Blocked by: ",
                Style::default()
                    .fg(COLOR_BLOCKED)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(issue.blocked_by.join(", ")),
        ]));
    }

    if !issue.blocks.is_empty() {
        lines.push(Line::from(vec![
            Span::styled(
                "Blocks: ",
                Style::default()
                    .fg(COLOR_BLOCKS)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(issue.blocks.join(", ")),
        ]));
    }

    // Timestamps
    lines.push(Line::from(vec![
        Span::styled("Created: ", Style::default().fg(COLOR_SECONDARY_TEXT)),
        Span::styled(&issue.created_at, Style::default().fg(COLOR_SECONDARY_TEXT)),
    ]));

    lines.push(Line::from(vec![
        Span::styled("Updated: ", Style::default().fg(COLOR_SECONDARY_TEXT)),
        Span::styled(&issue.updated_at, Style::default().fg(COLOR_SECONDARY_TEXT)),
    ]));

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Metadata ")
        .style(Style::default());

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

/// Renders the description area with scrolling. Returns (scroll_max, viewport_height).
fn render_detail_description(
    f: &mut Frame,
    area: Rect,
    issue: &Issue,
    scroll: u16,
) -> (u16, u16) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Description ")
        .style(Style::default());

    let inner_area = block.inner(area);
    let viewport_height = inner_area.height;

    if let Some(ref desc) = issue.description {
        let markdown_text = markdown_from_str(desc);

        let lines: Vec<Line> = markdown_text
            .lines
            .into_iter()
            .map(|line| {
                let spans: Vec<Span> = line
                    .spans
                    .into_iter()
                    .map(|span| {
                        let mut style = Style::default();
                        if let Some(fg) = span.style.fg {
                            style = style.fg(convert_color(fg));
                        }
                        if let Some(bg) = span.style.bg {
                            style = style.bg(convert_color(bg));
                        }
                        style = style.add_modifier(convert_modifier(span.style.add_modifier));
                        style = style.remove_modifier(convert_modifier(span.style.sub_modifier));
                        Span::styled(span.content.to_string(), style)
                    })
                    .collect();
                Line::from(spans)
            })
            .collect();

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false });

        let total_lines = paragraph.line_count(inner_area.width);
        let total_lines_u16 = total_lines.min(u16::MAX as usize) as u16;
        let scroll_max = total_lines_u16.saturating_sub(viewport_height);
        let clamped_scroll = scroll.min(scroll_max);

        let paragraph = paragraph.scroll((clamped_scroll, 0));
        f.render_widget(paragraph, area);

        // Render scrollbar when content overflows
        if scroll_max > 0 {
            let mut scrollbar_state = ScrollbarState::new(scroll_max as usize)
                .position(clamped_scroll as usize);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .thumb_style(Style::default().fg(COLOR_SCROLLBAR_THUMB))
                .track_style(Style::default().fg(COLOR_SCROLLBAR_TRACK));
            f.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
        }

        (scroll_max, viewport_height)
    } else {
        let paragraph = Paragraph::new("No description available.")
            .block(block)
            .style(Style::default().fg(COLOR_SECONDARY_TEXT));
        f.render_widget(paragraph, area);
        (0, viewport_height)
    }
}

// Helper function to convert ratatui_core::Color to ratatui::Color
fn convert_color(color: ratatui_core::style::Color) -> ratatui::style::Color {
    use ratatui::style::Color as RatatuiColor;
    use ratatui_core::style::Color as CoreColor;

    match color {
        CoreColor::Reset => RatatuiColor::Reset,
        CoreColor::Black => RatatuiColor::Black,
        CoreColor::Red => RatatuiColor::Red,
        CoreColor::Green => RatatuiColor::Green,
        CoreColor::Yellow => RatatuiColor::Yellow,
        CoreColor::Blue => RatatuiColor::Blue,
        CoreColor::Magenta => RatatuiColor::Magenta,
        CoreColor::Cyan => RatatuiColor::Cyan,
        CoreColor::Gray => RatatuiColor::Gray,
        CoreColor::DarkGray => RatatuiColor::DarkGray,
        CoreColor::LightRed => RatatuiColor::LightRed,
        CoreColor::LightGreen => RatatuiColor::LightGreen,
        CoreColor::LightYellow => RatatuiColor::LightYellow,
        CoreColor::LightBlue => RatatuiColor::LightBlue,
        CoreColor::LightMagenta => RatatuiColor::LightMagenta,
        CoreColor::LightCyan => RatatuiColor::LightCyan,
        CoreColor::White => RatatuiColor::White,
        CoreColor::Rgb(r, g, b) => RatatuiColor::Rgb(r, g, b),
        CoreColor::Indexed(i) => RatatuiColor::Indexed(i),
    }
}

// Helper function to convert ratatui_core::Modifier to ratatui::Modifier
fn convert_modifier(modifier: ratatui_core::style::Modifier) -> ratatui::style::Modifier {
    use ratatui::style::Modifier as RatatuiModifier;
    use ratatui_core::style::Modifier as CoreModifier;

    let mut result = RatatuiModifier::empty();

    if modifier.contains(CoreModifier::BOLD) {
        result |= RatatuiModifier::BOLD;
    }
    if modifier.contains(CoreModifier::DIM) {
        result |= RatatuiModifier::DIM;
    }
    if modifier.contains(CoreModifier::ITALIC) {
        result |= RatatuiModifier::ITALIC;
    }
    if modifier.contains(CoreModifier::UNDERLINED) {
        result |= RatatuiModifier::UNDERLINED;
    }
    if modifier.contains(CoreModifier::SLOW_BLINK) {
        result |= RatatuiModifier::SLOW_BLINK;
    }
    if modifier.contains(CoreModifier::RAPID_BLINK) {
        result |= RatatuiModifier::RAPID_BLINK;
    }
    if modifier.contains(CoreModifier::REVERSED) {
        result |= RatatuiModifier::REVERSED;
    }
    if modifier.contains(CoreModifier::HIDDEN) {
        result |= RatatuiModifier::HIDDEN;
    }
    if modifier.contains(CoreModifier::CROSSED_OUT) {
        result |= RatatuiModifier::CROSSED_OUT;
    }

    result
}

fn render_detail_footer(f: &mut Frame, area: Rect, scroll: u16, scroll_max: u16) {
    let mut parts = vec![
        Span::styled("[j/k] Scroll  ", Style::default().fg(COLOR_HELP_TEXT)),
        Span::styled("[g/G] Top/Bottom  ", Style::default().fg(COLOR_HELP_TEXT)),
        Span::styled("[Esc/q] Back", Style::default().fg(COLOR_HELP_TEXT)),
    ];

    if scroll_max > 0 {
        parts.push(Span::styled(
            format!("  [{}/{}]", scroll + 1, scroll_max + 1),
            Style::default().fg(COLOR_SECONDARY_TEXT),
        ));
    }

    let paragraph = Paragraph::new(Line::from(parts));
    f.render_widget(paragraph, area);
}

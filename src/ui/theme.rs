use ratatui::style::Color;
use ratatui::style::palette::material::*;

// Column Status Colors
pub const COLOR_OPEN: Color = BLUE.c400;           // Bright, inviting blue
pub const COLOR_IN_PROGRESS: Color = AMBER.c500;   // Warm, active amber
pub const COLOR_DONE: Color = GREEN.c400;          // Satisfying green

// Priority Colors (P0-P4)
pub const COLOR_P0: Color = RED.c500;              // Critical red
pub const COLOR_P1: Color = DEEP_ORANGE.c400;     // High orange
pub const COLOR_P2: Color = LIGHT_BLUE.c400;      // Medium blue
pub const COLOR_P3: Color = GRAY.c400;            // Low gray
pub const COLOR_P4: Color = BLUE_GRAY.c300;       // Very low blue-gray

// Semantic Colors
pub const COLOR_BLOCKED: Color = RED.c700;         // Darker red for blocked
pub const COLOR_BLOCKS: Color = ORANGE.c500;      // Orange for blocking

// UI Element Colors
pub const COLOR_HEADER: Color = CYAN.c300;         // Header branding
pub const COLOR_HEADER_BG: Color = BLUE_GRAY.c900; // Header background
pub const COLOR_SELECTED_BG: Color = BLUE_GRAY.c800; // Selection highlight
pub const COLOR_BORDER: Color = BLUE_GRAY.c600;   // Inactive borders
#[allow(dead_code)]
pub const COLOR_BORDER_ACTIVE: Color = CYAN.c400; // Active borders (reserved for future use)

// Text Colors
pub const COLOR_SECONDARY_TEXT: Color = GRAY.c500; // Issue IDs, timestamps
pub const COLOR_HELP_TEXT: Color = BLUE_GRAY.c400; // Footer help
pub const COLOR_SEPARATOR: Color = BLUE_GRAY.c700; // Visual separators

// Scrollbar Colors
pub const COLOR_SCROLLBAR_THUMB: Color = BLUE_GRAY.c400;
pub const COLOR_SCROLLBAR_TRACK: Color = BLUE_GRAY.c800;

// Search Colors
pub const COLOR_SEARCH_MATCH: Color = AMBER.c400;
pub const COLOR_SEARCH_BORDER: Color = CYAN.c400;

// Helper function for priority colors
pub fn priority_color(priority: u8) -> Color {
    match priority {
        0 => COLOR_P0,
        1 => COLOR_P1,
        2 => COLOR_P2,
        3 => COLOR_P3,
        _ => COLOR_P4,
    }
}

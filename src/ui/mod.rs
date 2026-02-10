pub mod app;
pub mod board;
pub mod detail;
pub mod search;
pub mod theme;

pub use app::{App, View};
pub use board::render_board;
pub use detail::render_detail;
pub use search::render_search;

use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Row, Table};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: &Rect) {
    let (x, y, w, h) = (rect.x, rect.y, rect.width, rect.height);
    frame.render_widget(
        Table::new(vec![
            Row::new(vec![""]).height((h - 6) / 2),
            Row::new(vec!["", "(p)", "Primary Keys"]),
            Row::new(vec!["", "(s)", "Secondary Keys"]),
            Row::new(vec!["", "(c)", "Components"]),
            Row::new(vec!["", "(i)", "Simplified Codes"]),
            Row::new(vec!["", "(f)", "Fullcoded Characters"]),
            Row::new(vec!["", "(q)(<Esc>)", "Exit"]),
        ])
        .style(Style::default())
        .block(
            Block::new()
                .title("Wafel Toaster")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .widths(&[
            Constraint::Length(w / 12),
            Constraint::Length(w / 3),
            Constraint::Length(w / 2),
        ]),
        *rect,
    );

    let home = &mut app.home;
    let (x, y) = (x + 2, y + 1);
    let w = w - 4;
    if home.get_elapsed() / 250 >= 2 * w as u64 {
        home.reset_elapsed();
    }
    let mut length = (home.get_elapsed() / 250) as u16;
    if length > w {
        length = 2 * w - length;
    }
    frame.render_widget(
        Paragraph::new("❖".repeat(length as usize)),
        Rect::new(x, y, w, 1),
    );
}

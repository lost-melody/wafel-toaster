use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: &Rect) {
    frame.render_widget(
        Block::new()
            .title("Primary")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL),
        *rect,
    );

    let rect = Rect::new(rect.x + 1, rect.y + 1, rect.width - 2, rect.height - 2);
    if let Some(current) = app.primary.get_current(&app.data) {
        let current = current.borrow();
        let elapsed = app.primary.get_elapsed();
        let code = if elapsed >= 3000
            && !current
                .comp
                .to_lowercase()
                .starts_with(&app.primary.get_input().to_lowercase().to_string())
        {
            &current.code
        } else {
            "*"
        };
        let y = rect.y + (rect.height - 4) / 2;
        frame.render_widget(
            Paragraph::new(format!("[ {} ]", current.comp)).alignment(Alignment::Center),
            Rect::new(rect.x, y, rect.width, rect.height),
        );
        frame.render_widget(
            Paragraph::new(format!("{:-8}{:8}", code, current.freq)).alignment(Alignment::Center),
            Rect::new(rect.x, y + 1, rect.width, rect.height),
        );
        frame.render_widget(
            Paragraph::new(format!(
                "{:-8}{:8.2}",
                app.primary.get_input(),
                elapsed as f64 / 1e3
            ))
            .alignment(Alignment::Center),
            Rect::new(rect.x, y + 3, rect.width, rect.height),
        );
    } else {
        frame.render_widget(Paragraph::new("No components found"), rect);
    }
}

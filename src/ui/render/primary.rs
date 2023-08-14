use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, rect: &Rect) {
    let para = if let Some(current) = app.primary.get_current(&app.data) {
        let current = current.borrow();
        let elapsed = app.primary.get_elapsed();
        let code = if elapsed > 3000
            && !current
                .comp
                .to_lowercase()
                .starts_with(&app.primary.get_input().to_lowercase().to_string())
        {
            &current.code
        } else {
            "**"
        };
        format!(
            "Comp: {}\nWeight: {}\nCode: {}\nElapsed: {:.2}\nInput: {}",
            current.comp,
            current.freq,
            code,
            elapsed as f64 / 1e3,
            app.primary.get_input()
        )
    } else {
        "No components found".to_string()
    };

    frame.render_widget(
        Paragraph::new(para).block(
            Block::new()
                .title("Primary")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        ),
        *rect,
    );
}

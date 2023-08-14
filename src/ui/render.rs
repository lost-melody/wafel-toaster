mod home;
mod primary;

use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use crate::app::App;
use crate::app::AppPage;
use crate::def;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let rect = frame.size();
    if rect.width >= def::MIN_WIDTH && rect.height >= def::MIN_HEIGHT {
        let rect = frame.size();
        let rect = Rect::new(
            rect.width / 4,
            rect.height / 4,
            rect.width / 2,
            rect.height / 2,
        );
        match app.page {
            AppPage::Home => home::render(app, frame, &rect),
            AppPage::Primary => primary::render(app, frame, &rect),
            _ => {}
        }
    } else {
        frame.render_widget(
            Paragraph::new(format!(
                "An area of at least {}x{} is required \n\
                to render in the terminal,\n\
                while currently only {}x{} is available.",
                def::MIN_WIDTH,
                def::MIN_HEIGHT,
                rect.width,
                rect.height,
            )),
            Rect::new(0, 0, rect.width, rect.height),
        )
    }

    frame.render_widget(
        Paragraph::new(format!("Version: {}", def::VERSION)),
        Rect::new(0, rect.height - 1, rect.width, 1),
    )
}

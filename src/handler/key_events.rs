mod home;
mod primary;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, AppPage, AppResult};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc => {
            if let AppPage::Home = app.page {
                app.quit();
                return Ok(());
            } else {
                app.page = AppPage::Home;
                return Ok(());
            }
        }
        KeyCode::Char('c') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
                return Ok(());
            }
        }
        _ => {}
    }

    // distribute key events by current page
    match app.page {
        AppPage::Home => home::handle_key_events(key_event, app)?,
        AppPage::Primary => primary::handle_key_events(key_event, app)?,
        _ => {}
    }

    Ok(())
}

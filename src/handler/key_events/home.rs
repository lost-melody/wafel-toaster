use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, AppPage, AppResult};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char(chr) => {
            if key_event.modifiers & !KeyModifiers::SHIFT == KeyModifiers::NONE {
                match chr {
                    'p' => app.page = AppPage::Primary,
                    's' => app.page = AppPage::Secondary,
                    'c' => app.page = AppPage::Component,
                    'i' => app.page = AppPage::Simp,
                    'f' => app.page = AppPage::Full,
                    'q' => app.quit(),
                    _ => {}
                }
            }
        }
        _ => {}
    }
    Ok(())
}

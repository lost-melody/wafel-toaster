use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, AppResult};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char(chr) => {
            if key_event.modifiers & !KeyModifiers::SHIFT == KeyModifiers::NONE {
                app.primary.set_input(chr);
                if let Some(current) = app.primary.get_current(&app.data) {
                    if current
                        .borrow()
                        .code
                        .to_lowercase()
                        .starts_with(&chr.to_lowercase().to_string())
                    {
                        app.primary.next();
                        app.primary.reset_elapsed();
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}

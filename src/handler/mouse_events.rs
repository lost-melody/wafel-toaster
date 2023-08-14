use crossterm::event::MouseEvent;

use crate::app::{App, AppResult};

/// Handles the mouse events and updates the state of [`App`].
pub fn handle_mouse_events(_: MouseEvent, _: &mut App) -> AppResult<()> {
    Ok(())
}

use crate::app::{App, AppResult};

/// Handles the resize events and updates the state of [`App`].
pub fn handle_resize_events(_: u16, _: u16, _: &mut App) -> AppResult<()> {
    Ok(())
}

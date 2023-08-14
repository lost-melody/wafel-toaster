use std::io;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use wafel_toaster::app::{App, AppResult};
use wafel_toaster::event::{Event, EventHandler};
use wafel_toaster::handler::{handle_key_events, handle_mouse_events, handle_resize_events};
use wafel_toaster::tui::Tui;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();
    app.init()?;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(250),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
            Event::Resize(x, y) => handle_resize_events(x, y, &mut app)?,
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

mod appdata;
mod home;
mod primary;

/// Application current page
#[derive(Debug)]
pub enum AppPage {
    /// The home page
    Home,
    /// Practise the primary keys of components
    Primary,
    /// Practise the secondary keys of components
    Secondary,
    /// Practise both the primary and the secondary keys of components
    Component,
    /// Practise the characters with simplified codes
    Simp,
    /// Practise the characters with full codes
    Full,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running
    pub running: bool,
    /// Data store
    pub data: appdata::AppData,
    /// Current page
    pub page: AppPage,
    /// Home page states
    pub home: home::Home,
    /// Primary page states
    pub primary: primary::Primary,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            data: Default::default(),
            page: AppPage::Home,
            home: Default::default(),
            primary: Default::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self, elapsed: u64) {
        match self.page {
            AppPage::Home => self.home.tick(elapsed),
            AppPage::Primary => self.primary.tick(elapsed),
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Read mappings data from file
    pub fn init(&mut self) -> super::AppResult<()> {
        self.data.init()?;

        Ok(())
    }
}

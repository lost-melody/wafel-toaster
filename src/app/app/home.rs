#[derive(Debug, Default)]
pub struct Home {
    elapsed: u64,
}

impl Home {
    pub fn tick(&mut self, elapsed: u64) {
        self.elapsed += elapsed;
    }

    pub fn get_elapsed(&self) -> u64 {
        self.elapsed
    }

    pub fn reset_elapsed(&mut self) {
        self.elapsed = 0;
    }
}

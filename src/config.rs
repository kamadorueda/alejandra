#[derive(Clone)]
pub enum Layout {
    Tall,
    Wide,
}

#[derive(Clone)]
pub struct Config {
    debug:     bool,
    layout:    Layout,
    max_width: usize,
}

impl Config {
    pub fn new() -> Config {
        Config { debug: false, layout: Layout::Tall, max_width: 80 }
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn max_width(&self) -> usize {
        self.max_width
    }

    pub fn with_debug(&self, debug: bool) -> Config {
        Config { debug, layout: self.layout.clone(), max_width: self.max_width }
    }

    pub fn with_layout(&self, layout: Layout) -> Config {
        Config { debug: self.debug, layout, max_width: self.max_width }
    }

    pub fn with_max_width(&self, max_width: usize) -> Config {
        Config { debug: self.debug, layout: self.layout.clone(), max_width }
    }
}

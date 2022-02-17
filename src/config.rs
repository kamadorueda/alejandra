#[derive(Clone, Debug)]
pub enum Layout {
    Tall,
    Wide,
}

#[derive(Clone)]
pub struct Config {
    debug:  bool,
    layout: Layout,
}

impl Config {
    pub fn new() -> Config {
        Config { debug: false, layout: Layout::Tall }
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn with_debug(&self, debug: bool) -> Config {
        Config { debug, layout: self.layout.clone() }
    }

    pub fn with_layout(&self, layout: Layout) -> Config {
        Config { debug: self.debug, layout }
    }
}

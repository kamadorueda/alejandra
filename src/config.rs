#[derive(Clone, Debug)]
pub enum Layout {
    Tall,
    Wide,
}

#[derive(Clone)]
pub struct Config {
    layout: Layout,
}

impl Config {
    pub fn new() -> Config {
        Config { layout: Layout::Tall }
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn with_layout(&self, layout: Layout) -> Config {
        Config { layout }
    }
}

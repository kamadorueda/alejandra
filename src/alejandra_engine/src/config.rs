#[derive(Clone, Debug)]
pub enum Layout {
    Tall,
    Wide,
}

#[derive(Clone)]
pub struct Config {
    layout: Layout,
}

impl Default for Config {
    fn default() -> Config {
        Config { layout: Layout::Tall }
    }
}

impl Config {
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn with_layout(&self, layout: Layout) -> Config {
        Config { layout }
    }
}

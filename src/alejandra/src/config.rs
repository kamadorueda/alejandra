use serde::Deserialize;

/// Configuration used by the formatter
#[derive(Clone, Copy, Default, Deserialize)]
pub struct Config {
    /// Indentation to use
    pub indentation: Indentation,
}

#[derive(Clone, Copy, Default, Deserialize)]
/// Indentation options
pub enum Indentation {
    /// Four spaces
    FourSpaces,
    /// Tabs
    Tabs,
    #[default]
    /// Two spaces
    TwoSpaces,
}

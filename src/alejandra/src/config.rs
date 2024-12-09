use serde::Deserialize;

/// Configuration used by the formatter
#[derive(Clone, Copy, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Indentation to use
    #[serde(default)]
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

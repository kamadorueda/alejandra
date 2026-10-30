use serde::Deserialize;

/// Configuration used by the formatter
#[derive(Clone, Copy, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Indentation to use
    #[serde(default)]
    pub indentation: Indentation,

    /// Whether to put spaces around brackets
    #[serde(default)]
    pub space_around_brackets: bool,

    /// Whether to keep attribute sets written on one line on one line
    #[serde(default)]
    pub preserve_one_line_attr_sets: bool,
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

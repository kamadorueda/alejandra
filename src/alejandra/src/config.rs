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

    /// Whether `then` ends the line of `if`, with each branch on the
    /// next lines, indented, and `else` at the level of `if`
    #[serde(default)]
    pub if_then_on_one_line: bool,
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

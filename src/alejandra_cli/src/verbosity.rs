#[derive(Clone, Copy)]
pub(crate) enum Verbosity {
    Everything,
    NoInfo,
    NoErrors,
}

impl Verbosity {
    pub(crate) fn allows_info(&self) -> bool {
        matches!(self, Verbosity::Everything)
    }

    pub(crate) fn allows_errors(&self) -> bool {
        self.allows_info() || matches!(self, Verbosity::NoInfo)
    }
}

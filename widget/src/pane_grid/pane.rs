/// A rectangular region in a [`PaneGrid`] used to display widgets.
///
/// [`PaneGrid`]: super::PaneGrid
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pane(pub(super) usize);

impl From<usize> for Pane {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Pane> for usize {
    fn from(value: Pane) -> usize {
        value.0
    }
}

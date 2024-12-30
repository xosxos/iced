use std::fmt;
use std::hash::Hash;
use std::sync::atomic::{self, AtomicU64};

/// The id of the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id(u64);

static COUNT: AtomicU64 = AtomicU64::new(1);

impl Id {
    /// Creates a new unique window [`Id`].
    pub fn unique() -> Id {
        Id(COUNT.fetch_add(1, atomic::Ordering::Relaxed))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Id> for u64 {
    fn from(value: Id) -> u64 {
        value.0
    }
}

impl From<u64> for Id {
    fn from(value: u64) -> Id {
        Self(value)
    }
}

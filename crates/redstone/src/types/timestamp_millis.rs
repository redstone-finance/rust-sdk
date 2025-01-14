use core::fmt::Debug;

/// Type describing timpestamp, we use to directly show we expect milliseconds.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimestampMillis(u64);

impl Debug for TimestampMillis {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<u64> for TimestampMillis {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl TimestampMillis {
    pub const fn from_millis(millis: u64) -> Self {
        Self(millis)
    }

    pub fn as_millis(&self) -> u64 {
        self.0
    }

    pub fn add(&self, other: impl Into<Self>) -> Self {
        Self(self.0 + other.into().0)
    }

    pub fn is_same_or_before(&self, other: Self) -> bool {
        self.0 <= other.0
    }

    pub fn is_same_or_after(&self, other: Self) -> bool {
        self.0 >= other.0
    }
}
